# libosinfo bindgen feasibility

Research for `.scratch/rust-conversion/issues/04-libosinfo-bindgen-feasibility.md`.

Question: is running `bindgen` directly against libosinfo's plain C headers
(instead of consuming the GObject-Introspection `.gir`/`.typelib` via
`gtk-rs`-style generated bindings) a practical path for full OS-detection /
osinfo-db parity? Specifically: GLib/GObject footprint without GTK, what the
C API needs for initialization, and how osinfo-db's XML data is distributed
and versioned.

Method: installed and inspected the real Ubuntu (resolute/24.10-ish rolling)
packages for the host — `libosinfo-1.0-0` 1.12.0-3build1, `osinfo-db`
0.20250606-1ubuntu2, and pulled `libosinfo-1.0-dev` 1.12.0-3build1 (headers,
`.pc` file, gtk-doc) with `apt-get download` + `dpkg-deb -x` since this
sandbox has no root to `apt install`. Cross-checked against the upstream
libosinfo GitLab source (`gitlab.com/libosinfo/{libosinfo,osinfo-db,
osinfo-db-tools}`, default branch `main`) and `libosinfo.org/download/`.

## Conclusion

**Practical.** libosinfo's public C API is plain GObject (not GTK), the
`.pc` file only requires `gobject-2.0`, and `ldd` on the installed `.so`
confirms no GTK/GDK/Pango/Cairo linkage — only glib/gobject/gio plus
libsoup, libxml2, libxslt as its own transitive deps. Initialization is a
handful of ordinary (non-async, non-main-loop-dependent) calls:
`osinfo_loader_new()` → `osinfo_loader_process_default_path()` →
`osinfo_loader_get_db()`. GObject's type system self-registers since GLib
2.36 (libosinfo requires GLib ≥ 2.44), so no `g_type_init()` call is needed.
osinfo-db is a separate, independently-versioned data package
(`/usr/share/osinfo/**/*.xml`, date-stamped `VERSION` file) installed by the
distro's `osinfo-db` package by default, with `osinfo-db-tools` providing
`--system`/`--local`/`--user` override paths that the loader's
`process_system_path`/`process_local_path`/`process_user_path` calls read —
this is exactly the mechanism virt-manager relies on to get newer OS
definitions than the distro package ships. The one real rough edge: the
`.pc` file's `Requires: gobject-2.0` is incomplete — `osinfo_db.h` directly
`#include`s `<gio/gio.h>`, so a `bindgen` wrapper header needs `gio-2.0`'s
cflags added by hand (or just `pkg-config --cflags gobject-2.0 gio-2.0` in
`build.rs`) even though the declared `Requires` field doesn't mention it.
No existing `-sys` crate for libosinfo was found on crates.io, so this would
be new work, but nothing in the C API shape makes it harder than any other
GObject-based `-sys` crate (e.g. hand-rolled parts of `gstreamer-sys`).

## Dependency footprint (GLib/GObject, no GTK)

`libosinfo-1.0.pc` (from the installed `libosinfo-1.0-dev` package):

```
Requires: gobject-2.0
Libs: -L${libdir} -losinfo-1.0
Cflags: -I${includedir}/libosinfo-1.0
```

`ldd` on the real installed `.so` (`/usr/lib/x86_64-linux-gnu/libosinfo-1.0.so.0.1012.0`):

```
libgobject-2.0.so.0
libglib-2.0.so.0
libgio-2.0.so.0
libsoup-3.0.so.0      (+ its own transitive deps: sqlite3, krb5, nghttp2, brotli, psl, idn2 …)
libxml2.so.16
libxslt.so.1
libgmodule-2.0.so.0
```

No `libgtk-*`, `libgdk-*`, `libpango-*`, `libcairo-*`, `libgdk_pixbuf-*`
anywhere in the link chain. `dpkg -s libosinfo-1.0-0` confirms the same at
the package-dependency level: `libglib2.0-0t64`, `libsoup-3.0-0`,
`libxml2-16`, `libxslt1.1`, plus `pci.ids`/`usb.ids` (device database data,
not code) and `osinfo-db` itself.

Upstream `meson.build` (`gitlab.com/libosinfo/libosinfo`, branch `main`)
declares as build deps: `glib-2.0 >= 2.44`, `gio-2.0 >= 2.44`,
`gobject-2.0 >= 2.44`, `libsoup-3.0` or `libsoup-2.4` (selectable), `libxml-2.0
>= 2.6.0`, `libxslt >= 1.0.0`, plus optional `hwdata`,
`g-ir-scanner` (for the `.gir`/`.typelib` this ticket is explicitly routing
around), and `gtkdoc-scan` (API docs, not the GTK toolkit). This matches the
runtime picture: **GObject + GLib + GIO + libsoup + libxml2/libxslt, no GTK
anywhere in the dependency graph.**

Practical implication for a `bindgen` `-sys` crate: link against
`glib-2.0`, `gobject-2.0`, `gio-2.0` (system dev packages — Debian/Ubuntu
`libglib2.0-dev`) and `libosinfo-1.0`. This is the same baseline footprint
any GObject-based Rust binding already carries (e.g. via `glib-sys`/
`gobject-sys`/`gio-sys` from the `gtk-rs` ecosystem, which can be reused
here as the base FFI types even while hand-writing the libosinfo bindings
with `bindgen` instead of `gir`).

## Initialization

Header inspection (`osinfo_loader.h`, `osinfo_db.h`) and the gtk-doc
reference (`Libosinfo-osinfo-loader.html`) show the C API's startup
sequence is small and synchronous — no GApplication, no GMainLoop
requirement to load the database:

```c
OsinfoLoader *loader = osinfo_loader_new();
GError *err = NULL;
osinfo_loader_process_default_path(loader, &err);   // or _system/_local/_user_path
OsinfoDb *db = osinfo_loader_get_db(loader);         // transfer-none, owned by loader
OsinfoOsList *oslist = osinfo_db_get_os_list(db);
```

- `osinfo_loader_process_default_path()` walks system → local → user paths
  in order (the individual `_process_system_path` / `_process_local_path` /
  `_process_user_path` calls exist separately if finer control is wanted).
- Errors are reported via the standard `GError **` out-param pattern, not
  exceptions or return codes — a `bindgen`-generated binding gets the raw
  `*mut GError` and needs a thin wrapper (already standard practice in
  `gtk-rs`-adjacent crates) to turn it into `Result`.
- No explicit `g_type_init()`/`g_type_init_with_debug_flags()` call is
  needed: that function was deprecated and became a no-op in GLib 2.36
  because `GType` self-registers via constructor attributes, and libosinfo
  requires GLib ≥ 2.44.
- Data access after loading is largely through a small number of *generic*
  accessors rather than one getter per XML field: `OsinfoEntity` (the base
  type for `OsinfoOs`, `OsinfoDevice`, `OsinfoMedia`, etc.) exposes
  `osinfo_entity_get_param_value(entity, key)` /
  `osinfo_entity_get_param_value_list()` / `_boolean` / `_int64` / `_enum`
  variants, and `OsinfoList` exposes `get_length`/`get_nth`/`find_by_id`/
  `get_elements`. This is good news for `bindgen` ergonomics: most
  osinfo-db field access doesn't need per-field FFI declarations, just this
  handful of `(entity, "key-string") -> value` functions, plus a modest set
  of typed getters for the few first-class fields (`osinfo_os_get_family`,
  `_get_distro`, `_get_release_status`, etc.) and the identify/match entry
  points (`osinfo_db_identify_media`, `osinfo_db_identify_tree`,
  `osinfo_db_identify_medialist`) that back virt-manager's OS-detection.
- Types are declared with GLib's standard `G_DECLARE_DERIVABLE_TYPE`-style
  macros (wrapped here as `OSINFO_DECLARE_TYPE_WITH_PRIVATE_AND_CLASS` in
  `osinfo_macros.h`). These expand to ordinary opaque structs and
  `*_get_type(void)` functions before `bindgen`'s clang parser ever sees
  them, so they bind the same way any other GObject type does — no special
  handling beyond what any GObject-based `-sys` crate already needs
  (reference counting via `g_object_ref`/`g_object_unref`, `GList`/`GSList`
  traversal, `GError` translation).

## osinfo-db distribution and versioning

Confirmed two independent ways, matching the loader's three-tier path
model:

1. **System package** (default/most common). On Debian/Ubuntu the `osinfo-db`
   package (separate from `libosinfo-1.0-0`, listed as a runtime `Depends:`
   of the library) installs XML under `/usr/share/osinfo/{os,platform,
   device,datamap,install-script,deployment,schema}/**/*.xml`, validated
   against `/usr/share/osinfo/schema/osinfo.rng`. Confirmed the compiled-in
   default system path is `/usr/share/osinfo` by grepping strings out of
   the actual linked `.so` (`strings libosinfo-1.0.so.0.1012.0 | grep
   /usr/share/osinfo`), which also shows a compiled-in local override path
   at `/etc/osinfo` (also seen as `/etc/libosinfo/db`). Note: the shipped
   `.pc` file's `system_db_dir`/`local_db_dir` variables
   (`/usr/data/libosinfo/db`, `/usr/etc/libosinfo/db`) do **not** match the
   actual compiled-in paths on this system — that's a stale/patched `.pc`
   quirk in the Ubuntu package, not something to trust; the binary's own
   strings are the ground truth.
   - A `VERSION` file ships at the root of the data tree
     (`/usr/share/osinfo/VERSION` → `20250712` on this host), independent
     from the `libosinfo-1.0-0` package version (`1.12.0-3build1`) and even
     from the `osinfo-db` package's own version
     (`0.20250606-1ubuntu2`) — confirming osinfo-db is versioned/released
     on its own date-based cadence, decoupled from the library.
   - `libosinfo.org/download/` states this explicitly: "The Osinfo database
     is updated frequently … individual releases are not listed [on the
     library's release page]" — i.e. osinfo-db has a much higher release
     cadence than `libosinfo`/`osinfo-db-tools`, which both track normal
     x.y.z versions (both 1.12.0 as of the last tagged releases seen).
2. **osinfo-db-tools overlay** (`osinfo-db-import`, `osinfo-db-export`,
   `osinfo-db-validate`, `osinfo-db-path`), a separate package
   (`osinfo-db-tools`, own deps: libarchive, glib, libjson-glib, libsoup,
   libxml2 — not linked into `libosinfo` itself). It installs newer
   `osinfo-db` tarball releases to one of three tiers without touching the
   distro package: `--system` (privileged, same dir the OS vendor's package
   uses), `--local` (admin override under `/etc`, doesn't conflict with the
   vendor package), or `--user` (per-user, under the XDG data dir). This is
   the mechanism that lets a host stay current with upstream osinfo-db
   between distro release cycles — directly relevant since the ticket notes
   virt-manager's OS-defaults logic depends on this data being current.
   These three tiers are exactly what `osinfo_loader_process_system_path` /
   `_process_local_path` / `_process_user_path` read back from, in that
   priority order, when `_process_default_path` is called.

Practical implication for the Rust side: a libosinfo binding should not
try to bundle or vendor osinfo-db data itself. It should call
`osinfo_loader_process_default_path()` (or replicate its
system→local→user precedence) and let the existing distro-package /
osinfo-db-tools split keep the data current, same as virt-manager does
today.

## Sources

- `libosinfo-1.0-dev` 1.12.0-3build1 (Ubuntu), downloaded via
  `apt-get download` and extracted with `dpkg-deb -x` on this host:
  - `usr/include/libosinfo-1.0/osinfo/*.h` (osinfo.h, osinfo_loader.h,
    osinfo_db.h, osinfo_entity.h, osinfo_list.h, osinfo_os.h,
    osinfo_macros.h)
  - `usr/lib/x86_64-linux-gnu/pkgconfig/libosinfo-1.0.pc`
  - `usr/share/gtk-doc/html/Libosinfo/Libosinfo-osinfo-loader.html`
- `ldd` / `dpkg -s` / `strings` against the installed
  `libosinfo-1.0-0` 1.12.0-3build1 runtime library and `osinfo-db`
  0.20250606-1ubuntu2 data package on this host, plus
  `/usr/share/osinfo/VERSION`.
- `gitlab.com/libosinfo/libosinfo` (`main` branch) — `meson.build` build
  dependency declarations.
- `libosinfo.org/download/` — component relationship and release-cadence
  description for libosinfo / osinfo-db-tools / osinfo-db.
- crates.io search for `libosinfo-sys`/`osinfo-sys` (no existing crate
  found as of this research).
