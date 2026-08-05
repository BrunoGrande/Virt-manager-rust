---
wayfinder: map
tracker: local-markdown
---

# Rust conversion of virt-manager

## Destination

A locked, hand-off-ready build spec for a full Rust rewrite of virt-manager
(GUI + `virtinst` core + `virt-install`/`virt-clone`/`virt-xml` CLI tools),
targeting Linux only, matching the existing GTK UI's screen layout and
behavior closely, with full driver parity across whatever libvirt backends
`virtinst`/domcapabilities expose (QEMU/KVM, Xen, LXC, bhyve, etc.) — not the
working code itself, which gets handed to `/to-spec`/`/to-tickets` once the
map's tickets are resolved.

## Notes

- Domain: virtualization management (libvirt/QEMU-KVM), desktop GUI + CLI tools.
- Upstream reference: virt-manager 5.1.0, installed source at
  `/usr/share/virt-manager` on this machine (`virtManager/` 28.9k LOC GUI,
  `virtinst/` 24.4k LOC core+CLI, 31 GtkBuilder `.ui` files). Upstream repo:
  https://github.com/virt-manager/virt-manager (pull `tests/` from there —
  not shipped in the installed package).
- Skills every session should consult: `grilling`, `domain-modeling` for
  ticket resolution; `research` for research tickets; `to-spec`/`to-tickets`
  once the map is clear.
- Reference project for Rust GUI conventions (predates GTK-fidelity decision;
  superseded by ticket 05's `gtk4-rs` choice, kept only as a pattern
  reference for non-widget-tree concerns like async plumbing):
  `~/Downloads/watermark_pro-main/` — eframe/egui, single App-struct-per-window,
  `rfd` for file dialogs, texture-blit pattern for image display.
- GitHub remote: https://github.com/BrunoGrande/Virt-manager-rust (Claude
  GitHub Actions integration installed). Tracker is local-markdown regardless
  (no `setup-matt-pocock-skills` config in this repo) — map and tickets live
  under `.scratch/rust-conversion/`, not GitHub Issues.

## Decisions so far

- [Destination](./map.md) — locked build-spec, Linux-only, close GTK UI
  match, full libvirt driver parity. (See Destination section above; this is
  the map's own founding decision, not a separate ticket.)
- [GTK4 console-widget availability](./issues/01-gtk4-console-widget-availability.md) — no: neither `gtk-vnc` nor `spice-gtk` has a GTK4 build, on any mainstream distro. A `gtk4-rs` UI gets no free console widget either — kills the main premise for choosing it over `egui` on that basis. Ticket 05 is now unblocked.
- [virt crate API coverage](./issues/02-virt-crate-api-coverage.md) — full coverage of migration/snapshots/nodedev/secrets/storage/capabilities; the one gap is domain/network/nodedev/secret/storage-pool event registration. **Amended by ticket 09**: upstream actually solves this via `libvirt-glib` (gir-bound, no GTK in the link chain), not a hand-rolled shim — bind it instead, for however much of the gap its `domain-added`/`domain-removed`/`started`/`stopped` signals cover.
- [Upstream test suite inventory](./issues/03-upstream-test-suite-inventory.md) — ~316 golden-XML fixtures (254 CLI compare + 62 xmlparse round-trip) spanning QEMU/KVM/Xen/LXC/Virtuozzo/bhyve/HVF and a wide OS/device matrix; this is the acceptance-bar corpus for the Rust port. Ticket 06 is now unblocked.
- [libosinfo bindgen feasibility](./issues/04-libosinfo-bindgen-feasibility.md) — practical: pure GObject C API, no GTK in the link chain, tiny init sequence, osinfo-db is a separately-versioned distro-installed data package with its own update tooling. Feeds the OS-detection/create-VM-wizard fog once tickets 05/06 land.
- [GUI toolkit choice](./issues/05-gui-toolkit-choice.md) — `gtk4-rs`: neither toolkit gets a free console widget (ticket 01), which cancels out both the "free widget" case for `gtk4-rs` and the "avoid GtkVnc/SpiceClientGtk binding risk" case for `egui`; on the remaining ~95% of the app, `gtk4-rs` gets close-GTK-match fidelity (theming, HIG, AT-SPI) by construction. Console rendering: pure-Rust protocol crates (`vnc-rs`/`rust-vnc`, `spice-client`) first, escalating a channel to gir-bound `libgvnc`/`libspice-client-glib` only if that crate can't deliver a required parity feature — full per-channel specifics deferred to the console-viewer GUI-screen ticket.
- [XML modeling approach](./issues/06-xml-modeling-approach.md) — typed structs bound to XPaths via a derive macro, reading/writing through a mutable order-preserving DOM built on the `edit-xml` crate. Neither original option alone satisfies ticket 03's acceptance bar (byte-exact round-trip of untouched XML, `virt-xml --edit` preserving foreign `xmlns:qemu` elements) without converging here anyway — this gets Rust compile-time field safety plus upstream's preserve-the-rest-of-the-document behavior, without porting xmlbuilder.py's runtime dynamic-dispatch descriptor engine.
- [GUI screen inventory](./issues/07-gui-screen-inventory.md) — the map's 15-screen list is really 17 top-level screens (`docs/research/gui-screen-inventory.md` has the full `.ui`-to-controller table) plus a 13-item shared sub-widget layer the original list missed. Inventory only, no per-screen specs yet — those are separate tickets, opened as needed.
- [Create VM wizard](./issues/08-create-vm-wizard.md) — `GtkStack` state machine, hand-written transition/skip `match` functions (one real skip rule), incrementally-mutated `WizardState` (plain struct, `Option<T>` fields, no typestate), `InstallMethod` enum-of-structs for compile-time-exhaustive per-method fields, `Guest` built directly at Finish bypassing ticket 06's DOM layer (nothing to preserve on a fresh document). Spun off the async-detect-gate question as ticket 09 and deferred the "customize before install" handoff to the not-yet-opened VM-details-window ticket.
- [Async / background-task model](./issues/09-async-background-task-model.md) — two separate problems: libvirt's own event loop binds `libvirt-glib` via `gir` (what upstream actually does, found in `virtmanager.py`'s bootstrap — amends ticket 02); blocking-call offload is plain OS threads + `glib::MainContext::channel()`, no `tokio` (every `virt` call is synchronous, nothing for a runtime to multiplex). Cooperative `Arc<AtomicBool>` cancellation, one shared progress-modal wrapper all long-running calls route through, matching upstream's `asyncjob.py` exactly.
- [VM details window](./issues/10-vm-details-window.md) — `GtkListBox` for the `hw-list` sidebar (a dozen-ish rows, not `ListView`-scale), direct port of upstream's pending-edit/Apply architecture (edits scoped to the selected row, confirm-or-discard on switch), a per-`HW_LIST_TYPE` `Option<T>`-field pending-edit struct (not an enum-of-structs — row fields can be simultaneously dirty, unlike ticket 08's mutually-exclusive `InstallMethod`). Remove Device is immediate, routes through ticket 09's async wrapper. Spun off Snapshot management as ticket 11 and deferred the live/persistent (`AFFECT_LIVE`/`AFFECT_CONFIG`) config duality to the `virtinst-core` ticket. **Amended by ticket 12** with that duality's concrete shape.
- [Add Hardware wizard](./issues/12-add-hardware-wizard.md) — `enum NewDevice` (17 mutually-exclusive variants, same compile-time-exhaustive pattern as ticket 08's `InstallMethod`), companion-controller dependencies folded directly into a variant rather than a bolted-on side-channel. Found and ported the concrete live+persistent attach pattern (`attach_device()` then always `add_device()`, degrading to persistent-only on hotplug failure) — the answer ticket 10 had deferred. Device-default dispatch stays opaque, deferred to the not-yet-opened per-driver-defaults item.

## Not yet specified

- [Snapshot management](./issues/11-snapshot-management.md) — ticket 11, open. Spun out of ticket 10: create/delete/revert, the snapshot list/tree widget, and how snapshot XML interacts with ticket 06's XML modeling.
- Per-screen specs for the remaining 14 screens + 13 shared widgets ticket 07 located — next up: VM Manager main window.
- Console-viewer implementation specifics (per-channel breakdown of the
  pure-Rust-first / gir-fallback approach decided in ticket 05, fallback
  triggers) — needs its own ticket once GUI-screen work starts.
- `virtinst-core` module/crate boundaries (domain/storage/network/nodedev
  XML layer per ticket 06, device-default logic, the `libvirt-glib` event
  binding per tickets 02/09, where the shared async/progress-modal wrapper
  lives) — needs its own ticket to lay out the crate structure.
- Per-driver device-default behavior (QEMU/KVM vs Xen vs LXC vs bhyve
  differences in add-hardware/create-VM options).
- Packaging/release process.

## Out of scope

- Non-Linux platforms (Windows/macOS) — virt-manager's real usage is
  managing local/remote libvirt hosts from Linux; a non-Linux build would
  only ever do remote connections, a different product shape. Decided
  2026-08-04.
