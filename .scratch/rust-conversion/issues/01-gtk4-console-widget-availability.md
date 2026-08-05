---
id: 01
title: GTK4 console-widget availability (gtk-vnc / spice-gtk)
type: research
status: open
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
