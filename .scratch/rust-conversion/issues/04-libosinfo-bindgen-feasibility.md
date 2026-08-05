---
id: 04
title: libosinfo bindgen feasibility
type: research
status: resolved
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

## Resolution

`bindgen` against libosinfo's plain C API is practical. Verified against the
real installed packages plus upstream source: the public API is pure
GObject with `Requires: gobject-2.0` and no GTK anywhere in the link chain
(`ldd` confirms only glib/gobject/gio/libsoup/libxml2/libxslt). Init is a
tiny synchronous sequence (`osinfo_loader_new` → `process_default_path` →
`osinfo_loader_get_db`), no `g_type_init()` needed on modern GLib. osinfo-db
XML is a separate, independently date-versioned data package
(`/usr/share/osinfo`, own `VERSION` file) installed by the distro by
default, with `osinfo-db-tools`' `--system`/`--local`/`--user` tiers as the
update-without-a-new-distro-release path — matching the loader's own
three-tier path precedence. One gotcha: the `.pc` file's declared
`Requires` omits `gio-2.0` even though a public header needs it, so a
`build.rs` must add `gio-2.0` cflags explicitly. Full writeup:
[`docs/research/libosinfo-bindgen.md`](../../../docs/research/libosinfo-bindgen.md).
