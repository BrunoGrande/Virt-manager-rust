# Upstream virt-manager test suite inventory

Source: https://github.com/virt-manager/virt-manager, cloned `--depth 1` on
2026-08-04 at commit `ab75ff49bc8cdd2154d0014a852e84f6b3285ae2` (2026-07-20).
Note this is the upstream Python source tree's `tests/` directory, **not**
the Debian-packaged `/usr/share/virt-manager` on this machine — the package
ships no test suite at all.

All paths below are relative to the repo root as cloned, e.g.
`tests/test_cli.py` = `<clone>/tests/test_cli.py`.

## TL;DR for the Rust port

The acceptance bar upstream already enforces on itself is: **run
virt-install/virt-clone/virt-xml (or the internal `Guest`/`DeviceXxx`
builder objects) against a fixed fake libvirt connection, and diff the
resulting XML string against a checked-in golden file, byte-for-byte modulo
a trailing newline.** There are two independent harnesses doing this:

1. **CLI-level golden tests** (`tests/test_cli.py` +
   `tests/data/cli/compare/*.xml`) — invoke the actual `virt-install`,
   `virt-clone`, `virt-xml` argument parsers/`main()` functions in-process
   against a scripted libvirt test driver, capture stdout XML, diff against
   a golden file.
2. **XMLBuilder-level round-trip tests** (`tests/test_xmlparse.py` +
   `tests/data/xmlparse/*.xml`) — construct/parse/mutate `Guest`,
   `StoragePool`, `StorageVolume`, `Network` etc. objects directly and diff
   `.get_xml()` output against golden `-out.xml` files (paired with
   `-in.xml` fixtures for edit/round-trip cases).

A Rust port that reproduces the ~300 golden XML files under `tests/data/`
byte-for-semantic-equivalent (same libvirt XML, attribute order doesn't
strictly matter but content does — the diff is a normal text diff so in
practice order matters unless the Rust harness re-serializes and
re-diffs through the same XML-canonicalization path) has strong evidence of
parity with upstream.

## Golden-file counts

| Fixture set | Location | File count | Driving call sites |
|---|---|---:|---:|
| CLI compare (virt-install) | `tests/data/cli/compare/virt-install-*.xml` | 124 | `vinst.add_compare(...)` in `tests/test_cli.py` |
| CLI compare (virt-xml) | `tests/data/cli/compare/virt-xml-*.xml` | 115 | `vixml.add_compare(...)` |
| CLI compare (virt-clone) | `tests/data/cli/compare/virt-clone-*.xml` | 15 | `vclon.add_compare(...)` |
| **CLI compare total** | `tests/data/cli/compare/` | **254** | ~250+ call sites (`.add_compare(` appears 259 times in `test_cli.py`, including a handful of golden files intentionally reused by two invocations to check idempotency, e.g. `xen-hvm`, `edit-simple-controller`, `clone-manual`) |
| XMLBuilder round-trip (`in`/`out` pairs) | `tests/data/xmlparse/*.xml` | 62 (≈31 in/out pairs) | 33 `testXxx()` functions in `tests/test_xmlparse.py`, via `utils.diff_compare()` / `_alter_compare()` |

Plus smaller golden-XML consumers that aren't the primary "conversion
correctness" surface but are worth knowing about:
- `tests/test_storage.py` — 2 `diff_compare()` calls (pool/volume XML).
- `tests/test_nodedev.py` — 1 `diff_compare()` call.
- `tests/test_misc.py` — 7 `diff_compare()` calls (misc XML utility checks,
  e.g. XML diff/indent helpers themselves).
- `tests/test_capabilities.py`, `tests/test_conn.py` — parse
  `tests/data/capabilities/*.xml` (libvirt `virsh capabilities`/domcapabilities
  fixtures) but assert on parsed Python objects, not golden output XML.

So the "acceptance bar" figure to hold a Rust port to is realistically
**~316 golden-XML fixtures** (254 CLI-compare + 62 xmlparse), with the CLI
compare set being by far the larger and more end-to-end relevant one since
it exercises the full `virt-install`/`virt-clone`/`virt-xml` option-parsing
→ device-object-building → XML-serialization pipeline, not just an
isolated builder class.

## How the CLI harness invokes virtinst (`tests/test_cli.py`)

Structure, top to bottom:

- **`TEST_DATA`** dict (`tests/test_cli.py:71-97`) — a printf-style
  substitution table for command strings: fake libvirt URIs (`URI-KVM-X86`,
  `URI-KVM-AARCH64`, `URI-TEST-FULL`, …), pre-created disk image paths under
  `/pool-dir/`, fake install media paths under `tests/data/fakemedia/`.
- **`Command`** class (`tests/test_cli.py:205-388`) — represents one CLI
  invocation. `_launch_command()` monkeypatches `sys.argv`/`sys.stdout`/
  `sys.stdin` and calls straight into `virtinstall.main(conn=conn)`,
  `virtclone.main(conn=conn)`, or `virtxml.main(conn=conn)` **in-process**
  (no subprocess spawn) against a `conn` opened from the fake URI.
  `_check_compare_file()` then calls `utils.diff_compare(output,
  self.compare_file)`.
- **`App`** class (`tests/test_cli.py:416-495`) — one instance per tool
  (`vinst = App("virt-install")`, `vixml = App("virt-xml")`, `vclon =
  App("virt-clone")`). `add_category()` groups tests under shared default
  args (e.g. all "kvm-generic" tests share `--connect
  %(URI-KVM-X86)s --autoconsole none`). Each category exposes
  `add_valid()` (should succeed, no XML check), `add_invalid()` (should
  fail, checked via `grep=`), and `add_compare(args, compbase)` (build the
  command, capture output, diff against
  `tests/data/cli/compare/<toolname>-<compbase>.xml`).
- For compare tests, `_default_args()` auto-appends `--print-step all`
  (virt-install) or `--print-xml --__test-nodry` (virt-clone) so the tool
  prints XML instead of actually defining a domain against real libvirt.
- **Fake libvirt backend**: `utils.URIs` (`tests/utils.py:54-131`) builds
  `__virtinst_test__test://<xmlfile>,predictable` URIs — a custom
  in-process fake driver, not real libvirt — pointed at static
  `tests/data/testdriver/*.xml` state files and paired with static
  `tests/data/capabilities/*.xml` capabilities/domCapabilities XML per
  target arch/hypervisor. `,predictable` makes generated UUIDs/MACs
  deterministic so golden-file diffs are stable.
- **Golden-file comparison**: `utils.diff_compare()`
  (`tests/utils.py:243-271`) does a straight string diff
  (`virtinst.xmlutil.diff`) between actual and expected output. If the
  golden file doesn't exist yet, or pytest is run with
  `--regenerate-output`, it **writes** the actual output as the new golden
  file — i.e. golden fixtures are generated by running the real
  virt-install code once and eyeballing/reviewing the diff, not
  hand-written. `--regenerate-output` is registered in
  `tests/conftest.py:17`.
- **Test discovery**: `_make_testcases()` (`tests/test_cli.py:2797-2822`)
  dynamically synthesizes one module-level test function per registered
  `Command`, named `testCLI0001<compare-file-basename>`, etc., which pytest
  then collects normally — there's no `@pytest.mark.parametrize`, it's done
  via `globals()[name] = testfunc`.
- Run via `pytest` (Meson wires `pytest_prog` as the `pytest` test target
  in `tests/meson.build:31-36`, invoked from repo root with default pytest
  discovery — `setup.cfg` sets `addopts=--tb=native`, no special test
  selection).

## Driver / architecture / OS-variant / device coverage

### Hypervisor drivers exercised (`tests/utils.py:54-131`)
Each maps to a fake URI + static capabilities/domCapabilities XML fixture
under `tests/data/capabilities/`:

- **QEMU/KVM x86_64** — several URI variants: `kvm_x86` (session and
  system), `kvm_x86_nodomcaps`, `kvm_x86_cpu_insecure`,
  `kvm_x86_oldfirmware`, `kvm_amd_sev`, plus a TLS-remote variant.
- **QEMU/KVM non-x86**: `armv7l`, `aarch64`, `ppc64le`, `s390x`,
  `loongarch64`; plain QEMU (non-KVM) `riscv64`.
- **Xen** (`xen:///`, `xen-rhel5.4.xml` caps) — dedicated `xen` category in
  `test_cli.py:1926`.
- **LXC** (`lxc:///`, `lxc.xml` caps) — dedicated `lxc` category
  (`test_cli.py:1914`).
- **Virtuozzo/vz** (`vz:///`) — dedicated `vz` category
  (`test_cli.py:1952`).
- **bhyve** (`bhyve:///`, plus domCapabilities) — dedicated `bhyve`
  category, two sub-variants (`test_cli.py:1975`, `1984`).
- **HVF** (`hvf_x86`, macOS Hypervisor.framework via QEMU) — used in a
  handful of compare tests.
- **Generic `test://` driver** (`test_full`, `test_suite`, `test_default`,
  `test_remote`, `test_empty`, `test_defaultpool_collision`) — the base
  fake driver most non-hypervisor-specific tests run against, backed by
  static state files in `tests/data/testdriver/*.xml`.

### Architectures (via `--arch`)
x86_64 (implicit default), aarch64, armv7l/arm, ppc64le, s390x, riscv64,
loongarch64 — counted directly in golden filenames: 8 `aarch64-*`, 4
`arm*`, 1 `ppc64le*`, 3 `s390x*`, 6 `riscv64*`, 6 `loongarch64*` compare
fixtures (`tests/data/cli/compare/`).

### OS variants (`--os-variant`/`--osinfo`, via libosinfo)
Wide spread across compare tests, e.g. `fedora19/20/21/26/27/28/29/30`,
`generic` (33 uses — the default for the big device-coverage tests),
`win7`, `winxp`, `win2k3`, `win10`, `win11` (Secure Boot/TPM paths),
`rhel7.0`, `debian9`, `solaris10`, `silverblue29`, `linux2020`, plus
`detect`/`detect=yes` (media-based OS detection against fake ISO/tree
fixtures in `tests/data/fakemedia/`).

### Device/feature coverage (flag frequency across `test_cli.py`, all
apps combined)
`--disk` (218), `--connect` (102), `--os-variant` (86), `--edit` (78,
virt-xml), `--osinfo` (68), `--boot` (60), `--pxe` (53), `--file` (51),
`--graphics` (44), `--network` (43), `--cdrom` (35), `--location` (32),
`--add-device` (31, virt-xml), `--cpu` (30), `--arch` (30),
`--print-diff` (29), `--auto-clone` (29, virt-clone), `--import` (28),
`--define` (28), `--cloud-init` (27), `--controller` (24), `--unattended`
(22), `--sound` (22), `--remove-device` (20), `--hostdev`/`--host-device`
(19+17), `--tpm` (16), `--machine` (15), `--filesystem` (15), `--sysinfo`
(14), `--video` (12), `--vcpus`/`--memory`/`--channel`/`--input`/`--mac`
present but lower-frequency. There's also a single mega test — the "many
devices" comment at `tests/test_cli.py:524` describing itself as "the main
XML coverage tester" — that in one invocation sets ~15 boot/firmware
options, vCPU topology + NUMA cells + cache/distance tables, and dozens of
CPU flags, specifically to touch as much of the XML surface as possible in
one golden comparison.

### Representative example
`tests/test_cli.py:1799-1801` registers:
```python
c.add_compare(
    '--arch aarch64 --osinfo fedora19 --machine virt --cpu default '
    '--boot kernel=/f19-arm.kernel,initrd=/f19-arm.initrd,'
    'kernel_args="console=ttyAMA0,1234 rw root=/dev/vda3" --disk %(EXISTIMG1)s',
    "aarch64-machvirt",
)
```
which is diffed against `tests/data/cli/compare/virt-install-aarch64-machvirt.xml`,
a full `<domain>` document (UEFI-less direct-kernel-boot aarch64/virtio
guest, deterministic UUID `00000000-1111-2222-3333-444444444444` and MAC
`00:11:22:33:44:55` thanks to the `,predictable` fake-URI flag).

## `tests/test_xmlparse.py` round-trip pattern

33 `testXxx()` functions (old unittest-style names, still pytest-collected)
build or parse a `Guest`/`StoragePool`/`StorageVolume`/`Network` object from
an `-in.xml` fixture (or from scratch), apply a mutation via the Python
XMLBuilder property API, call `.get_xml()`, and diff against a `-out.xml`
golden fixture via `_alter_compare()` (`tests/test_xmlparse.py:29-31`),
which also calls `utils.check_create()` to confirm the resulting XML
actually validates against the fake libvirt driver (`defineXML` round-trip),
not just that the string matches. Fixture pairs cover: guest device
add/remove, CPU mode changes, disk bus changes, controller changes, NIC
changes, boot order, snapshots, storage pools (fs/iscsi/gluster/rbd),
networks, `xmlns:qemu` passthrough namespace preservation, and general
`<domain>` round-tripping (parse → re-serialize with no changes → byte
match, `testDomainRoundtrip` at `tests/test_xmlparse.py:972`).

## Coverage self-check: `tests/test_checkprops.py`

`testCheckXMLBuilderProps()` (`tests/test_checkprops.py:16-38`) is a
meta-test: with an env var set, the internal `XMLBuilder` base class
tracks every declared XML property (`_allprops`) versus every property
actually touched by any test (`_seenprops`) during the whole run, and fails
the suite if any XML property was never exercised. This is upstream's own
mechanism for keeping the golden-file suite from silently losing coverage
as new XML properties are added — a Rust port's own test suite should
budget for an equivalent "every field got exercised at least once" check
rather than assuming the ported golden files alone guarantee completeness.

## Other `tests/` contents (not XML-comparison, for completeness)

- `tests/test_urls.py`, `test_urldetect.py`, `test_uriparse.py` — install
  media / tree URL detection (distro autodetection from install trees),
  backed by `tests/data/urldetect/{centos,debian,fedora,mageia,opensuse,
  rhel,suse,ubuntu,generic}/`.
- `tests/test_storage.py`, `test_nodedev.py`, `test_capabilities.py`,
  `test_conn.py`, `test_osdict.py`, `test_dist.py`, `test_inject.py`,
  `test_misc.py` — unit tests for storage pool/volume handling, host
  device listing, capabilities/domCapabilities parsing, connection
  handling, osinfo DB lookups, and OS media injection; mostly assert on
  parsed Python object state rather than golden XML.
- `tests/uitests/` — a separate, large (22-file) Dogtail-driven GUI
  test suite for the `virt-manager` GTK application itself; out of scope
  for a CLI/library-only Rust port's XML-correctness bar.

Total `tests/*.py` (excluding `uitests/`) is ~6,306 lines across 16 files;
`test_cli.py` alone is 2,826 lines and is the single most important file
for XML-conversion parity purposes.

## Practical implications for the Rust port (feeds ticket 06)

1. The ~254 `tests/data/cli/compare/*.xml` files are a ready-made,
   drop-in golden-file corpus: each filename encodes tool + scenario
   (`virt-install-aarch64-cloud-init.xml`, `virt-xml-...`,
   `virt-clone-...`), and the CLI args that produced them are visible
   right next to the `add_compare()` call in `tests/test_cli.py`. A Rust
   test harness can replay the same CLI argv strings against a Rust
   `virt-install`/`virt-xml`/`virt-clone` and diff against the identical
   golden files without needing to re-derive expected output.
2. The `,predictable` fake-URI flag is load-bearing — without it, UUIDs
   and MAC addresses are randomized, which would make byte-diff golden
   testing impossible. A Rust port needs an equivalent deterministic
   test-mode RNG/MAC-allocator hook to reuse these fixtures as-is.
3. `tests/data/capabilities/*.xml` and `tests/data/testdriver/*.xml` are
   themselves fixtures (fake libvirt capabilities/domCapabilities/state)
   that would need to be ported or reused verbatim, since the golden XML
   output depends on what capabilities the "connection" reports (e.g.
   which machine types, firmware, CPU models are available).
4. `tests/data/xmlparse/*-in.xml` / `*-out.xml` pairs are the second
   corpus to port for exercising the pure XML-object-model layer
   (`Guest`, device classes) independent of CLI argument parsing.
