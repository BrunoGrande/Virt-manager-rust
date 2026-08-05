---
id: 02
title: virt crate API coverage vs required libvirt surface
type: research
status: resolved
blocked_by: []
claimed_by: null
---

## Question

Does the `virt` crate (Rust libvirt bindings, https://crates.io/crates/virt)
cover the libvirt C API surface that `virtManager`/`virtinst` actually call —
domain event registration (for live UI updates without polling), migration
APIs, snapshot APIs, nodedev, secrets, storage-pool/volume jobs, and
domcapabilities/capabilities queries across multiple drivers (QEMU/KVM, Xen,
LXC, bhyve)? List concrete gaps, if any, and whether they'd require raw FFI
against `libvirt-sys`/the C headers directly.

This feeds ticket 06 (XML modeling approach) and the eventual GUI-shell
implementation — a polling-only fallback is a real but worse alternative if
event registration isn't covered.

## Resolution

Verified by reading `libvirt-rust`'s (the `virt` crate's) actual source
(`src/*.rs`) and its own CI coverage test (`tests/api.rs`), not just
docs.rs prose. Migration, snapshots, node devices, secrets, storage
pools/volumes, and capabilities/domcapabilities are all fully wrapped
(secrets/storage-pool/storage-vol are even CI-enforced at 100% coverage
against the C API). The one real gap: `virConnectDomainEventRegisterAny`
and its network/nodedev/secret/storage-pool siblings are **not** wrapped
anywhere in the safe `virt` API — only the low-level event-loop plumbing
(`virEventAddHandle`/`AddTimeout`/`RegisterDefaultImpl`/`RunDefaultImpl`) is
covered. The raw C symbols do exist in `virt-sys`'s bindgen output, so a
small `unsafe` FFI shim (using `Connect::as_ptr()` plus a C-callback
trampoline, mirroring the pattern `virt`'s own `src/event.rs` already uses
internally) is sufficient to unblock live domain-event notifications
without falling back to polling. Full findings, method, and per-area
evidence: [`docs/research/virt-crate-coverage.md`](../../../docs/research/virt-crate-coverage.md).
