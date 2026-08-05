---
id: 05
title: GUI toolkit choice (egui/eframe vs gtk4-rs)
type: grilling
status: open
blocked_by: [01]
claimed_by: null
---

## Question

Given the close-GTK-UI-match fidelity requirement (destination decision),
should the spec commit to `egui`/`eframe` (pure Rust, no GObject-Introspection
binding risk, but requires hand-building GTK-alike chrome and a from-scratch
console renderer) or `gtk4-rs` (real widget/theming parity by construction,
but reopens binding-generation risk for `GtkVnc`/`SpiceClientGtk`/
`Libosinfo`/`AyatanaAppIndicator` — see ticket 01's findings)?

HITL — resolve via `/grilling` with the user once ticket 01's findings are in.
