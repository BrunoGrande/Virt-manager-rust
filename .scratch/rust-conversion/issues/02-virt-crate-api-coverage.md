---
id: 02
title: virt crate API coverage vs required libvirt surface
type: research
status: open
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
