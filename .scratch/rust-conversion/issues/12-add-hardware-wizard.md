---
id: 12
title: Add Hardware wizard
type: grilling
status: resolved
blocked_by: [06, 07, 08, 09, 10]
claimed_by: null
---

## Question

Ticket 07 flagged this as the third-biggest screen (`addhardware.py`, 1622
LOC). Its structure: a static `hw-list` of 17 device-type categories
(Disk/Controller/Network/Input/Graphics/Sound/Hostdev/Char/Video/Watchdog/
Filesystem/Smartcard/USB-redir/TPM/RNG/Panic/Vsock — some conditionally
grayed out by capability checks), selecting one swaps a type-specific
config page, and Finish builds one fully-typed device against the target
VM's existing XML (`dev.set_defaults(vm.get_xmlobj())`) before attaching
it. Unlike ticket 08's Create VM (fresh document) or ticket 10's VM
details (editing existing fields), this adds a *new* device to a domain
that may already be running — what does that live-attach-plus-persist
sequencing look like in the Rust port, and how does the device-type
selection get modeled?

## Resolution

Grilled across 2 rounds, grounded directly in `addhardware.py`: traced
`_add_device` (the actual live/persistent pattern — attempts
`vm.attach_device()` if the domain is active, degrades to "apply after
next shutdown" with a confirm dialog on hotplug failure, then *always*
also calls `vm.add_device()` for the persistent config — two explicit
calls, not a single `AFFECT_LIVE|AFFECT_CONFIG` flag), `_build_device_page`
(17 mutually-exclusive device-type builders), `_build_xmleditor_device`
(XML-editor override re-parses the *same* device class, doesn't fork into
a separate path), `_validate_device` (thin dispatch, mostly `dev.validate()`
— validation logic lives in the typed device object, not the GUI), and the
`_remove_usb_controller` special case (a narrow, self-contained business
rule: upstream explicitly points users to the VM details screen for
non-add controller changes, not a general add/edit blur).

**Live + persistent attach — the concrete answer to what ticket 10
deferred.** Port `attach_device()`-then-`add_device()` directly, including
the confirm-and-degrade-to-persistent-only step on hotplug failure. This
is the concrete shape of the live/persistent duality ticket 10 punted to
`virtinst-core` — **amending ticket 10's note** with it now rather than
re-discovering it later.

**Device-type data model.** `enum NewDevice { Disk(DiskConfig),
Controller(ControllerConfig), Network(NetworkConfig), .. }` (17 variants,
compile-time exhaustive) — direct reuse of ticket 08's `InstallMethod`
pattern, and an even cleaner fit: these variants are genuinely mutually
exclusive with no async-detection wrinkle complicating "which variant is
current." The XML-editor override doesn't get its own variant — same as
upstream, it re-derives the already-selected variant's data from parsed
XML text instead of widget state.

**Controller-dependency modeling.** A device's implicit companion
controller (e.g. a new Disk needing a new SCSI controller) is folded
directly into that variant — `DiskConfig { companion_controller:
Option<ControllerConfig>, .. }` — not a bolted-on side-channel field next
to `NewDevice`, matching upstream's `vmm_controller` attribute. Keeping it
inside the variant makes controller-first attach/persist sequencing
structurally guaranteed rather than checked ad hoc at the call site, which
is the entire reason enum-of-structs was chosen in the first place.

**Explicitly deferred, not this ticket's problem:**
- **Scope**: stays existing-domain-only (`vmmAddHardware` always takes a
  real, already-defined domain upstream). The not-yet-created-`Guest` case
  (ticket 08's deferred "customize before install" dialog) is left to
  whichever ticket eventually reconciles it — not retroactively pulled in
  here.
- **Per-driver device defaults** (`dev.set_defaults(...)`): opaque
  call-through. The map already earmarks "per-driver device-default
  behavior" as its own not-yet-opened item; ticket 12 needs to know the
  call happens in the pipeline, not design QEMU-vs-Xen-vs-LXC dispatch now.
