# `virt` crate API coverage vs. libvirt C API surface

Research for `.scratch/rust-conversion/issues/02-virt-crate-api-coverage.md`.

Date: 2026-08-04
Crate examined: `virt` v0.4.3 (latest on crates.io as of this research),
source at <https://gitlab.com/libvirt/libvirt-rust> (mirrored read-only at
<https://github.com/libvirt/libvirt-rust>), docs at
<https://docs.rs/virt/latest/virt/>.

Method: cloned `libvirt/libvirt-rust` (git SHA at time of research: HEAD of
`master`, matching the `0.4.3` `Cargo.toml` version) and read the actual
Rust source in `src/*.rs`, plus the crate's own CI coverage-tracking test
(`tests/api.rs`), rather than relying solely on docs.rs prose. Cross-checked
against the libvirt C API reference at libvirt.org.

## Summary

The `virt` crate wraps the vast majority of the libvirt C API directly and
fairly mechanically (one Rust method per C function, per its own README).
Migration, snapshots, node devices, secrets, storage pools/volumes, and
capabilities/domcapabilities queries are all covered. **The one substantial,
confirmed gap is domain (and network/storage-pool/node-device/secret) event
registration** — `virConnectDomainEventRegisterAny` and siblings are not
wrapped anywhere in the crate's safe API. The crate does wrap the low-level
libvirt event-loop plumbing (`virEventAddHandle`/`virEventAddTimeout`/
`virEventRegisterDefaultImpl`/`virEventRunDefaultImpl`), which is necessary
infrastructure for event delivery but is not itself the event-subscription
API.

## Evidence the crate self-tracks API coverage

The crate ships a test, `tests/api.rs`, that walks the raw `virt-sys`
bindgen output and asserts Rust-level coverage for selected C API function
prefixes, gated by Cargo features:

```rust
// tests/api.rs (libvirt-rust master, matches virt 0.4.3)
#[cfg(all(feature = "api_coverage", not(feature = "api_coverage_full")))]
const ENFORCED_FUNC_PREFIXES: &[&str] = &["virSecret", "virStoragePool", "virStorageVol"];
#[cfg(all(feature = "api_coverage", not(feature = "api_coverage_full")))]
const ENFORCED_ENUM_PREFIXES: &[&str] = &["VIR_FROM", "VIR_ERR"];

// Informative-only, full-crate coverage check (not enforced in CI):
#[cfg(feature = "api_coverage_full")]
const ENFORCED_FUNC_PREFIXES: &[&str] = &[""];
```

This confirms, from the maintainers' own CI, that `virSecret*`,
`virStoragePool*`, and `virStorageVol*` are treated as **100%-covered**
surfaces (CI fails if a new libvirt release adds an uncovered function in
those families). Everything else (including domain events) is only checked
under the informational `api_coverage_full` feature, i.e. not held to a
completeness bar.

`IGNORE_FUNCS` in the same file lists functions deliberately excluded from
coverage tracking (thread-unsafe error accessors, `virTypedParams*` direct
exposure, deprecated `virDomainCreateLinux`) — domain/network/storage-pool
event functions are **not** in this exclusion list, meaning they are a real,
acknowledged gap rather than an intentional omission.

## Domain event registration — GAP

Searched `src/connect.rs`, `src/domain.rs`, and `src/event.rs` (the entire
crate) for `EventRegister`, `event_register`, `DomainEvent`: no matches
outside `event.rs`'s low-level handle/timeout plumbing.

`src/event.rs` provides only:
- `event_add_handle()` / `EventHandleWatch` — wraps `virEventAddHandle`
- `event_add_timeout()` / `EventTimeoutWatch` — wraps `virEventAddTimeout`
- `event_register_default_impl()` — wraps `virEventRegisterDefaultImpl`
- `event_run_default_impl()` — wraps `virEventRunDefaultImpl`

None of these is `virConnectDomainEventRegisterAny` /
`virConnectDomainEventDeregisterAny` (documented at
<https://libvirt.org/html/libvirt-libvirt-domain.html#virConnectDomainEventRegisterAny>,
confirmed live on libvirt.org: registers a callback for one of the
`VIR_DOMAIN_EVENT_ID_*` event classes — lifecycle, reboot, RTC change,
watchdog, IO error, graphics, block job, disk change, tray change, PM
wakeup/suspend, device removed, tunable, agent lifecycle, migration
iteration, job completed, device-added, metadata-change, block-threshold —
on a `Connect`, optionally scoped to one `Domain`).

Equivalent gaps exist for the sibling APIs: `virConnectNetworkEventRegisterAny`,
`virConnectNodeDeviceEventRegisterAny`, `virConnectSecretEventRegisterAny`,
`virConnectStoragePoolEventRegisterAny` — none are wrapped either (verified
by their absence from `src/network.rs`, `src/nodedev.rs`, `src/secret.rs`,
`src/storage_pool.rs`, and `src/connect.rs`).

**However**, the raw C functions ARE present in the `virt-sys` bindgen output
(`virt-sys/bindgen/bindings.rs`), because `virt-sys` runs bindgen over the
full libvirt headers (`virt-sys/wrapper.h` includes `libvirt/libvirt.h`,
`libvirt/virterror.h`, `libvirt/libvirt-qemu.h`) rather than a curated
subset:

```
5097:    pub fn virConnectDomainEventRegister(
5781:    pub fn virConnectDomainEventRegisterAny(
7177:    pub fn virConnectNetworkEventRegisterAny(
7495:    pub fn virConnectNodeDeviceEventRegisterAny(
7797:    pub fn virConnectSecretEventRegisterAny(
8351:    pub fn virConnectStoragePoolEventRegisterAny(
```

**Implication for this project**: getting live domain-event notifications
(the "no polling" requirement) means either (a) calling
`virt_sys::virConnectDomainEventRegisterAny` directly via `unsafe` FFI —
using the crate's own `Connect::as_ptr()` to get the underlying
`virConnectPtr` and building the C callback glue by hand (`extern "C" fn`
trampoline + boxed closure, similar to what `virt`'s own `event.rs` does
internally for handles/timeouts) — or (b) upstreaming a wrapper into
`libvirt-rust` itself (patches go to GitLab, not the GitHub mirror:
<https://gitlab.com/libvirt/libvirt-rust>) and vendoring/patching until
merged. Both are viable; (a) is the faster unblock, (b) is more idiomatic
long-term and the maintainers are active (feature-gated `qemu` support,
recent `2023`+ commits present in `tests/api.rs` header).  A polling
fallback via `Domain::get_state()`/`Connect::list_all_domains()` remains
available regardless as a degraded-mode option.

## Migration APIs — covered

`src/domain.rs` (`impl Domain`) has the full migration family:

```rust
pub fn migrate(&self, dconn: &Connect, flags: u32, dname: Option<&str>, uri: Option<&str>, bandwidth: u64) -> Result<Domain, Error>
pub fn migrate2(&self, dconn: &Connect, dxml: Option<&str>, flags: u32, dname: Option<&str>, uri: Option<&str>, bandwidth: u64) -> Result<Domain, Error>
pub fn migrate3(&self, dconn: &Connect, parameters: MigrateParameters, flags: u32) -> Result<Domain, Error>
pub fn migrate_to_uri(&self, ...) -> Result<(), Error>
pub fn migrate_to_uri2(&self, ...) -> Result<(), Error>
pub fn migrate_to_uri3(&self, ...) -> Result<(), Error>
pub fn migrate_set_max_speed(&self, ...) -> Result<(), Error>
pub fn migrate_get_max_speed(&self) -> Result<u64, Error>
pub fn migrate_set_max_downtime(&self, ...) -> Result<(), Error>
pub fn migrate_set_compression_cache(&self, ...) -> Result<(), Error>
pub fn migrate_get_compression_cache(&self) -> Result<u64, Error>
```

Covers `virDomainMigrate`/`Migrate2`/`Migrate3` and their `ToURI` variants
(peer-to-peer and direct/managed migration modes), plus tunables
(bandwidth/speed, max downtime, compression cache). `get_job_stats()` /
`get_job_info()` on `Domain` cover migration progress monitoring
(`virDomainGetJobStats`/`virDomainGetJobInfo`). No gap identified here.

## Snapshot APIs — covered

`src/domain_snapshot.rs` (`impl DomainSnapshot`) plus `Domain` methods:

- `Domain::create_snapshot_xml()` → `virDomainSnapshotCreateXML`
- `Domain::lookup_snapshot_by_name()` → `virDomainSnapshotLookupByName`
- `Domain::current_snapshot()` → `virDomainHasCurrentSnapshot`/`virDomainSnapshotCurrent`
- `Domain::list_all_snapshots()` → `virDomainListAllSnapshots`
- `DomainSnapshot::{connect, domain, name, xml_desc, parent, revert, delete,
  num_children, is_current, has_metadata, list_all_children}`

Covers the full snapshot lifecycle (create/list/inspect/revert/delete,
parent/child tree walking, metadata check). No gap identified.

## Node device APIs — covered

`src/nodedev.rs` (`impl NodeDevice`) plus `Connect` lookups:

- `Connect::{list_all_node_devices, lookup_node_device_by_name,
  lookup_node_device_scsi_host_by_wwn, create_node_device_xml,
  num_of_node_devices}`
- `NodeDevice::{name, parent, xml_desc, destroy, detach, reset, reattach,
  detach_flags, num_of_caps, list_caps}`

Covers device enumeration, XML description, capability listing, and
attach/detach for host device passthrough. No gap identified. (Event
registration for node devices is the one sub-gap, noted above.)

## Secrets — covered

`src/secret.rs` (`impl Secret`) plus `Connect` lookups:

- `Connect::{list_all_secrets, list_secrets, define_secret_xml,
  lookup_secret_by_uuid, lookup_secret_by_uuid_string,
  lookup_secret_by_usage, num_of_secrets}`
- `Secret::{connect, usage_id, usage_type, uuid, uuid_string, xml_desc,
  set_value, value, undefine}`

This is one of the CI-enforced 100%-coverage families (`virSecret*` in
`ENFORCED_FUNC_PREFIXES`). No gap identified.

## Storage pool / volume APIs — covered

`src/storage_pool.rs` (`impl StoragePool`) and `src/storage_vol.rs`
(`impl StorageVol`), both also CI-enforced 100%-coverage families
(`virStoragePool*`, `virStorageVol*`):

- Pool: lookup by name/UUID/UUID-string/target-path, define/create/build
  from XML, destroy/delete/undefine, refresh, autostart get/set,
  active/persistent checks, `info()`, volume enumeration
  (`list_volumes`/`list_all_volumes`/`num_of_volumes`), `find_storage_pool_sources`
  on `Connect` for pool-source discovery.
- Volume: create from XML (including cloning via `create_xml_from`), lookup
  by key/path, `info()`/`info_flags()`, `resize()`, `delete()`,
  `wipe()`/`wipe_pattern()`, `download()`/`upload()` (volume data
  streaming, backed by `src/stream.rs`).

No gap identified for the storage-job/volume surface itself. (Storage pool
*event* registration, `virConnectStoragePoolEventRegisterAny`, is the one
sub-gap, part of the general event-registration gap above.)

## Capabilities / domcapabilities — covered

On `Connect` (`src/connect.rs`):

```rust
pub fn capabilities(&self) -> Result<String, Error>                     // virConnectGetCapabilities
pub fn domain_capabilities(&self, emulatorbin, arch, machine, virttype, flags)
    -> Result<String, Error>                                            // virConnectGetDomainCapabilities
```

Both return the raw capabilities XML (as libvirt itself does — capabilities
and domcapabilities are XML documents, not structured C data beyond the
query parameters). `domain_capabilities()` accepts optional
emulator-binary/arch/machine-type/virt-type filters, matching the C
signature, so per-hypervisor-driver queries (QEMU/KVM, Xen via `virttype`,
etc.) are reachable. Also present: `get_max_vcpus`/`max_vcpus`,
`get_cpu_models_names`/`cpu_models_names`, `compare_cpu`, `baseline_cpu` —
useful adjuncts for capability-driven UI (CPU model pickers, live
migration compatibility checks). No gap identified; multi-driver coverage
is a caller-side XML-parsing concern (the crate returns XML, same as the C
API), not a crate coverage gap.

LXC and bhyve are libvirt *drivers*, selected via connection URI
(`lxc:///system`, `bhyve:///system`) — the `virt` crate's API is
driver-agnostic (it calls the generic `virDomain*`/`virConnect*` functions),
so driver coverage is a libvirt-side property, not something the Rust
binding layer restricts. Confirmed no driver-specific gating in
`src/connect.rs`'s `open()`/`open_auth()` (they take an arbitrary URI
string).

## Concrete gap list

| Area | Status | Detail |
|---|---|---|
| Domain event registration (`virConnectDomainEventRegisterAny`) | **Missing** | Not wrapped; raw C symbol present in `virt-sys` bindgen output only |
| Network event registration (`virConnectNetworkEventRegisterAny`) | **Missing** | Same as above |
| Node device event registration (`virConnectNodeDeviceEventRegisterAny`) | **Missing** | Same as above |
| Secret event registration (`virConnectSecretEventRegisterAny`) | **Missing** | Same as above |
| Storage pool event registration (`virConnectStoragePoolEventRegisterAny`) | **Missing** | Same as above |
| Low-level event loop (`virEventAddHandle`/`AddTimeout`/`RegisterDefaultImpl`/`RunDefaultImpl`) | Covered | `src/event.rs` |
| Migration (`virDomainMigrate`/`2`/`3`, `ToURI` variants, tunables, job stats) | Covered | `src/domain.rs` |
| Snapshots (create/list/revert/delete/tree-walk) | Covered | `src/domain_snapshot.rs`, `src/domain.rs` |
| Node device (enumerate/attach/detach/reset/caps) | Covered | `src/nodedev.rs` |
| Secrets (CRUD + value get/set) | Covered, CI-enforced 100% | `src/secret.rs` |
| Storage pool (CRUD, autostart, volume enum) | Covered, CI-enforced 100% | `src/storage_pool.rs` |
| Storage volume (create/clone/resize/wipe/upload/download) | Covered, CI-enforced 100% | `src/storage_vol.rs` |
| Capabilities (`virConnectGetCapabilities`) | Covered | `src/connect.rs` |
| Domain capabilities (`virConnectGetDomainCapabilities`) | Covered | `src/connect.rs` |

## Recommendation for ticket 06 / GUI shell

Event registration is the only real blocker for "live UI updates without
polling." Two paths, not mutually exclusive:

1. **Short term**: write a small `unsafe` FFI shim over
   `virt_sys::virConnectDomainEventRegisterAny` (and the network/nodedev/
   secret/storage-pool siblings as needed) inside this project, using
   `Connect::as_ptr()` from the `virt` crate to get the `virConnectPtr` and
   modeling the C-callback trampoline the same way `virt`'s own
   `src/event.rs` does for handle/timeout callbacks (boxed closure + opaque
   pointer + `extern "C"` shim + free callback). This keeps the rest of the
   codebase on the safe `virt` API and isolates `unsafe` to one module.
2. **Longer term**: contribute the wrapper upstream to
   `gitlab.com/libvirt/libvirt-rust` (GitHub is a read-only mirror; patches
   must go to GitLab) so the project (and others) can drop the local shim
   later.

A polling-only fallback (periodic `list_all_domains()` +
`Domain::get_state()`/`get_info()` diffing) remains available as a
degraded-mode option per the ticket's framing, but is not necessary given
option 1 above is a small, contained amount of `unsafe` code.

## Sources

- `virt` crate on crates.io: <https://crates.io/crates/virt> (v0.4.3)
- `virt` crate docs: <https://docs.rs/virt/latest/virt/>
- `virt::event` module docs: <https://docs.rs/virt/latest/virt/event/index.html>
- `Connect` struct docs: <https://docs.rs/virt/latest/virt/connect/struct.Connect.html>
- `Domain` struct docs: <https://docs.rs/virt/latest/virt/domain/struct.Domain.html>
- Canonical source (GitLab, where patches are accepted): <https://gitlab.com/libvirt/libvirt-rust>
- Read-only GitHub mirror (used for `git clone` in this research):
  <https://github.com/libvirt/libvirt-rust> — files read directly:
  `src/connect.rs`, `src/domain.rs`, `src/domain_snapshot.rs`,
  `src/event.rs`, `src/nodedev.rs`, `src/secret.rs`, `src/storage_pool.rs`,
  `src/storage_vol.rs`, `Cargo.toml`, `tests/api.rs`,
  `virt-sys/wrapper.h`, `virt-sys/bindgen/bindings.rs`
- libvirt C API reference:
  <https://libvirt.org/html/libvirt-libvirt-domain.html#virConnectDomainEventRegisterAny>
  (confirms the function's existence, signature, and purpose)
