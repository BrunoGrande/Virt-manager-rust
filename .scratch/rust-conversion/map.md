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
- Reference project for Rust GUI conventions (predates GTK-fidelity decision,
  now contested by ticket 05): `~/Downloads/watermark_pro-main/` — eframe/egui,
  single App-struct-per-window, `rfd` for file dialogs, texture-blit pattern
  for image display.
- GitHub remote: https://github.com/BrunoGrande/Virt-manager-rust (Claude
  GitHub Actions integration installed). Tracker is local-markdown regardless
  (no `setup-matt-pocock-skills` config in this repo) — map and tickets live
  under `.scratch/rust-conversion/`, not GitHub Issues.

## Decisions so far

- [Destination](./map.md) — locked build-spec, Linux-only, close GTK UI
  match, full libvirt driver parity. (See Destination section above; this is
  the map's own founding decision, not a separate ticket.)

## Not yet specified

- Console-viewer implementation specifics (VNC/SPICE/serial) — depends on
  ticket 05 (GUI toolkit choice).
- Every individual GUI screen: connection manager, VM manager main window,
  VM details tabs, add-hardware wizard, create-VM wizard, clone dialog,
  migrate dialog, host storage tab, host network tab, snapshot management,
  delete-VM dialog, preferences, XML editor tab, systray, about dialog —
  depends on tickets 05 and 06.
- Per-driver device-default behavior (QEMU/KVM vs Xen vs LXC vs bhyve
  differences in add-hardware/create-VM options).
- Packaging/release process.

## Out of scope

- Non-Linux platforms (Windows/macOS) — virt-manager's real usage is
  managing local/remote libvirt hosts from Linux; a non-Linux build would
  only ever do remote connections, a different product shape. Decided
  2026-08-04.
