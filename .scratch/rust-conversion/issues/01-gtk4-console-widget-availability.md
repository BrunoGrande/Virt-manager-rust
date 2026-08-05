---
id: 01
title: GTK4 console-widget availability (gtk-vnc / spice-gtk)
type: research
status: resolved
blocked_by: []
claimed_by: null
---

## Question

Do `gtk-vnc` and `spice-gtk` ship GTK4-compatible widgets today (as opposed
to only GTK3), and are those GTK4 versions actually packaged and available
on current mainstream Linux distros (Debian/Ubuntu, Fedora)? Include version
numbers and which distro releases carry them.

This feeds ticket 05 (GUI toolkit choice) — if GTK4-native console widgets
are real and available, that changes the cost/benefit of choosing `gtk4-rs`
over `egui` given the close-GTK-match fidelity requirement.

## Resolution

No. As of 2026-08-04, neither `gtk-vnc` (latest release 1.5.0) nor `spice-gtk`
(latest release in the 0.42–0.43 line) has a GTK4 build — both upstream
`master` branches depend exclusively on `gtk+-3.0 (>= 3.22)`, with no GTK4
option, dependency, or changelog entry in either project. Consequently
Debian (stable through unstable), Ubuntu 24.04 LTS, and Fedora 43/44/Rawhide
all package only the GTK3-linked widgets (`gtk-vnc`/`gtk-vnc2` and
`spice-gtk`/`spice-gtk3`) — there is no GTK4 variant to package. This closes
off the "GTK4-native console widgets are already available" premise for
ticket 05: a `gtk4-rs` UI would need a GTK3-embedding shim or a
non-upstream-widget approach to render VNC/SPICE consoles today. Full
findings and sources: [docs/research/gtk4-console-widgets.md](../../../docs/research/gtk4-console-widgets.md).
