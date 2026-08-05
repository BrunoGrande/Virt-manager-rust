---
id: 03
title: Upstream test suite inventory
type: research
status: resolved
blocked_by: []
claimed_by: null
---

## Question

Pull virt-manager's upstream test suite from
https://github.com/virt-manager/virt-manager (`tests/` directory — not
shipped in the Debian-installed package at `/usr/share/virt-manager` on this
machine) and catalog what it actually covers: how many XML-comparison/golden
tests exist, which drivers/OS variants/device combinations they exercise,
and how the test harness invokes `virtinst`. This becomes the acceptance bar
for "converted correctly" — a Rust port that reproduces these XML outputs
byte-for-semantic-equivalent has real evidence of parity, not just a
plausible-looking implementation.

This feeds ticket 06 (XML modeling approach).

## Resolution

Cloned upstream at commit `ab75ff49bc8cdd2154d0014a852e84f6b3285ae2`
(2026-07-20) and inventoried `tests/`. The acceptance-bar corpus is
~254 golden XML fixtures under `tests/data/cli/compare/` (124
virt-install, 115 virt-xml, 15 virt-clone), driven by `tests/test_cli.py`
which runs the real CLI `main()` functions in-process against a
deterministic fake libvirt driver and diffs stdout XML against the golden
files, plus a second corpus of 62 `tests/data/xmlparse/*-in/out.xml`
round-trip fixtures for the pure `Guest`/device object model. Coverage
spans QEMU/KVM (x86_64, aarch64, armv7l, ppc64le, s390x, riscv64,
loongarch64), Xen, LXC, Virtuozzo, bhyve, and HVF, a wide OS-variant
matrix (Fedora, RHEL, Windows 7/XP/2k3/10/11, Debian, Solaris, generic),
and dozens of device/feature flags (disk, boot/UEFI, NUMA/CPU topology,
TPM, hostdev, cloud-init, unattended install, etc.). Full inventory with
file:line citations: `docs/research/upstream-test-inventory.md`.
