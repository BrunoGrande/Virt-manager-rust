---
id: 17
title: Per-driver device-default behavior
type: grilling
status: resolved
blocked_by: [02, 06, 15]
claimed_by: null
---

## Question

QEMU/KVM vs Xen vs LXC vs bhyve differences in add-hardware/create-VM
device defaults (e.g. default disk bus, video model) — how much of a
distinct architectural concern is this really, and how does it fit into
`virtinst-core`'s already-decided module boundaries (ticket 15)?

## Resolution

Grounded directly in source before asking anything, and it turned out
much more contained than "per-driver behavior" sounds like it should be.

**Driver detection is trivial** — `is_qemu()`/`is_xen()`/`is_lxc()`/
`is_bhyve()` are plain URI-scheme-prefix checks (`qemu:///system` →
qemu, etc.) already living in `connection.py`, i.e. ticket 15's
`connection` module. No new mechanism needed for this part.

**No driver-strategy abstraction upstream, and none built here either.**
Traced `disk.py`'s `_default_bus` — the real shape of "per-driver device
defaults" — and it's one ordered `if`-chain mixing three signal sources:
driver family (`conn.is_bhyve()`), guest OS/machine specifics
(`guest.os.is_q35()`, `is_xenpv()`, `is_x86()`), and live capability
queries (`guest.supports_virtiodisk()`). Grepped the whole
`devices/`+`domain/` tree for the driver-check pattern: only **9 files**
branch on driver family at all (`disk`, `graphics`, `video`, `hostdev`,
`filesystem`, `interface`, `clock`, `cpu`, plus `guest.py`) — not a
pervasive axis through the whole device model, and the branching is
per-*field* (disk bus), not per-whole-device. A `trait DriverDefaults`
with one impl per driver would need one method per field anyway — more
ceremony than upstream's already-reasonable ordered-chain structure, not
less. **Decided: direct port** — an ordered `match`/`if` chain inside
each relevant `devices::*`/`domain::*` type's own default-resolution
method, typed instead of duck-typed, same shape as upstream, on the same
9-file scope.

**`DomainCapabilities` gets its own lightweight path, separate from
ticket 06's DOM.** It's an `XMLBuilder`-descended typed object upstream,
but critically **read-only**: fetched fresh per-connection via
`virConnectGetDomainCapabilities`, cached on `Guest`
(`lookup_capsinfo()`, invalidated only when arch/machine/os-type change),
never edited or serialized back. None of ticket 06's order-preserving DOM
— built for interactive multi-field *editing* that must round-trip
byte-exact — does anything useful on a hot, cached, read-only path; it'd
be pure bookkeeping overhead on every capability lookup. **Decided:** a
separate simple deserialize-only path (typed struct, direct XML→struct
parse, no DOM, no preservation machinery) rather than routing through
ticket 06's derive macro in some "read-only mode." Two small,
clearly-scoped XML mechanisms beat one doing double duty.

**Validation**: ticket 03's ~316-fixture golden-XML corpus already spans
exactly this matrix (QEMU/KVM, Xen, LXC, Virtuozzo, bhyve, HVF, wide
OS/device combinations) — it's the acceptance bar for "these default
chains reproduce upstream's output," not a new test-design question this
ticket needs to solve.
