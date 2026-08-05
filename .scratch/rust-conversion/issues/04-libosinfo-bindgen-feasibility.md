---
id: 04
title: libosinfo bindgen feasibility
type: research
status: open
blocked_by: []
claimed_by: null
---

## Question

Is `bindgen` against `libosinfo`'s C API (not the GObject-Introspection
`.gir`/`.typelib` path) actually clean in practice for full OS-detection/
osinfo-db parity? Cover: what GLib/GObject dependency footprint it pulls in
(and whether that's avoidable without pulling GTK itself), what
initialization the C API requires, and how osinfo-db's XML data files are
distributed/versioned on a target system (system package vs bundled copy) —
since virt-manager's OS-defaults logic depends on that data being current.

This feeds the (currently fog) OS-detection/create-VM-wizard work, once
tickets 05 and 06 land.
