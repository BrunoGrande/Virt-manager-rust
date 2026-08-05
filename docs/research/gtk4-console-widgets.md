# GTK4 console-widget availability: gtk-vnc and spice-gtk

**Date:** 2026-08-04
**Question:** Do `gtk-vnc` and `spice-gtk` ship GTK4-compatible widgets today, and are those
GTK4 builds actually packaged in current Debian/Ubuntu and Fedora releases?
**Feeds:** `.scratch/rust-conversion/issues/01-gtk4-console-widget-availability.md` (ticket 01),
which in turn feeds ticket 05 (GUI toolkit choice).

## Short answer

**No.** As of 2026-08-04, neither `gtk-vnc` nor `spice-gtk` has a GTK4-linked widget in their
upstream `master` branch or in any released version. Both projects build exclusively against
`gtk+-3.0` (minimum version 3.22). No GTK4 build option, dependency, or changelog entry exists
in either repo. Consequently, no mainstream distro (Debian, Ubuntu, Fedora) packages a GTK4
build of either library — they all ship the GTK3 widgets only.

## gtk-vnc

Upstream repo: `https://gitlab.gnome.org/GNOME/gtk-vnc` (GNOME GitLab).

- **Build system targets GTK3 only.** `meson.build` on `master` (fetched 2026-08-04, latest
  commit dated 2026-08-04) sets `gtk_min_version = '3.22.0'` and declares
  `dependency('gtk+-3.0', version: '>= ' + gtk_min_version)`. There is no `gtk+-4.0` /
  `gtk4` dependency anywhere in the file, and no `gtk_api_version`-style option to select a
  GTK major version.
  Source: https://gitlab.gnome.org/GNOME/gtk-vnc/-/raw/master/meson.build
- **`meson_options.txt`** on `master` defines only `introspection`, `pulseaudio`, `sasl`,
  `with-coroutine`, `with-tls-priority`, `with-vala`, `gi-docs`, `valgrind` — nothing GTK4
  related.
  Source: https://gitlab.gnome.org/GNOME/gtk-vnc/-/raw/master/meson_options.txt
- **`NEWS`** (the project's changelog) has no mention of GTK4/"GTK 4" anywhere across its full
  history from 2007 through the latest entries.
  Source: https://gitlab.gnome.org/GNOME/gtk-vnc/-/raw/master/NEWS
- **Latest tagged releases:** `1.5.0` (2025-02-07), `1.4.0` (2025-01-06), `v1.3.1`
  (2022-07-14). All are GTK3-only.
  Source: https://gitlab.gnome.org/GNOME/gtk-vnc/-/tags
- **Naming caveat:** the library's soname/package name is `libgtk-vnc-2.0` and its GObject
  Introspection namespace is `Gvnc/GtkVnc-2.0`. That "2.0" is gtk-vnc's own library ABI
  version (bumped when the project moved from GTK2 to GTK3 around 2011) — it does **not**
  indicate the GTK major version it links against. It is easy to misread `libgtk-vnc-2.0` as
  "GTK 2.0 widget," but confirmed via meson.build the actual linked toolkit is GTK3.
- No open GitLab issue or merge request titled/labelled around a GTK4 port was found on the
  gtk-vnc tracker (checked `https://gitlab.gnome.org/GNOME/gtk-vnc/-/issues` and
  `/-/merge_requests`).

## spice-gtk

Upstream repo: `https://gitlab.freedesktop.org/spice/spice-gtk` (canonical; mirrored at
`https://gitlab.com/spice/spice-gtk`, which was used for fetches since freedesktop.org's
GitLab is behind an Anubis bot-challenge that blocks automated fetches).

- **Build system targets GTK3 only.** `meson.build` on `master` (via the `gitlab.com/spice/spice-gtk`
  mirror, fetched 2026-08-04) declares
  `dependency('gtk+-3.0', version : '>= @0@'.format(gtk_version_required))` with
  `gtk_version_required = '3.22'`, checks for `gtk-3.0/gdk/gdkwayland.h`, and its generated
  pkg-config file is described as "SPICE Client Gtk 3.0 library." No `gtk+-4.0` / `gtk4`
  dependency appears anywhere in the file.
  Source: https://gitlab.com/spice/spice-gtk/-/raw/master/meson.build
- **`meson_options.txt`** defines a generic `gtk` feature toggle (enable/disable the GTK
  widget entirely) but no GTK-major-version selector; no `gtk4` option exists.
  Source: https://gitlab.com/spice/spice-gtk/-/raw/master/meson_options.txt
- Package naming downstream (`spice-gtk3` in Fedora, `libspice-client-gtk-3.0-*` in
  Debian/Ubuntu) is consistent with this: it is explicitly the GTK3 widget, with no GTK4
  counterpart.

## Related upstream signal: virt-viewer

`virt-viewer` (the C/GTK VNC/SPICE console app that both these libraries exist to serve) has
an open, unresolved GitLab issue about a GTK4 port:

- **"Port virt-viewer to Gtk+ version 4" (#54)**, GitLab issue on
  `https://gitlab.com/virt-viewer/virt-viewer/-/issues/54`. The issue description itself
  frames the problem as unresolved — whether to support GTK3 and GTK4 in parallel or commit
  to one — citing GTK's own migration docs describing the two as significantly
  incompatible. This is consistent with gtk-vnc/spice-gtk (virt-viewer's console-widget
  dependencies) not yet having GTK4 builds to port against.

## Distro packaging check (2026-08-04)

### Debian

Source: Debian Package Tracker, `https://tracker.debian.org/pkg/gtk-vnc` and
`https://tracker.debian.org/pkg/spice-gtk`.

| Package | Suite | Version | Binaries |
|---|---|---|---|
| gtk-vnc | oldoldstable | 1.0.0-1 | GTK3-linked |
| gtk-vnc | oldstable | 1.3.1-1 | GTK3-linked |
| gtk-vnc | stable / testing / unstable | **1.5.0-1** | `gir1.2-gtk-vnc-2.0`, `gvncviewer`, `libgtk-vnc-2.0-0`, `libgtk-vnc-2.0-dev`, `libgvnc-1.0-0`, `libgvnc-1.0-dev` — all GTK3 |
| spice-gtk | oldoldstable | 0.39-1 | GTK3-linked |
| spice-gtk | oldstable | 0.42-1 | GTK3-linked |
| spice-gtk | stable | 0.42-3 | GTK3-linked |
| spice-gtk | testing / unstable | **0.42-4** | `libspice-client-gtk-3.0-5`, `libspice-client-gtk-3.0-dev`, `gir1.2-spiceclientgtk-3.0`, plus `libspice-client-glib-2.0-*` — all GTK3/GLib, no GTK4 binaries |

### Ubuntu

Source: Ubuntu Packages search (`packages.ubuntu.com`).

- `libgtk-vnc-2.0-0` (from source package `gtk-vnc`) in **noble (24.04 LTS)**: version
  **1.3.1-1**, universe repository, GTK3-linked widget. (Older than the 1.5.0 that's in
  Debian testing/unstable, but still GTK3 — Ubuntu just hasn't picked up the newer
  release.) Source: `https://packages.ubuntu.com/libgtk-vnc-2.0-0`,
  `https://packages.ubuntu.com/focal/libdevel/libgtk-vnc-2.0-dev` (older focal entry for
  cross-reference).
- `spice-gtk` in **noble (24.04 LTS)**: version **0.42-2ubuntu2**, universe repository.
  Binaries: `gir1.2-spiceclientgtk-3.0`, `libspice-client-gtk-3.0-5`,
  `libspice-client-gtk-3.0-dev`, `spice-client-gtk`, plus glib-only packages — all explicitly
  GTK3, no GTK4 binaries.
  Source: `https://packages.ubuntu.com/source/noble/spice-gtk`.

### Fedora

Source: rpmfind.net's mirrored Fedora repo metadata (used because
`packages.fedoraproject.org` / `apps.fedoraproject.org` are behind an Anubis bot-challenge
that blocked direct automated fetches at research time).

| Package | Fedora release | Version | Description |
|---|---|---|---|
| gtk-vnc2 | Rawhide (fc45) | 1.5.0-6 | "A GTK3 widget for VNC clients" |
| gtk-vnc2 | Fedora 44 | 1.5.0-4 | "A GTK3 widget for VNC clients" |
| gtk-vnc2 | Fedora 43 | 1.5.0-3 | "A GTK3 widget for VNC clients" |
| spice-gtk3 | Rawhide (fc45) | 0.43-4 | "A GTK3 widget for SPICE clients" |
| spice-gtk3 | Fedora 44 | 0.42-8 | "A GTK3 widget for SPICE clients" |
| spice-gtk3 | Fedora 43 | 0.42-7 | "A GTK3 widget for SPICE clients" |

Fedora's own package names/descriptions (`gtk-vnc2` = "A GTK3 widget…", `spice-gtk3` = "A
GTK3 widget…") make the GTK3-only status explicit at the packaging level, independent of the
upstream source check above.

## Conclusion

1. **Upstream:** Both `gtk-vnc` (latest release 1.5.0, 2025-02-07) and `spice-gtk` (latest
   packaged release in the 0.42–0.43 line) build exclusively against GTK3
   (`gtk+-3.0 >= 3.22`) as of their current `master` branches (checked 2026-08-04). Neither
   project has a GTK4 dependency, build option, or any GTK4 mention in its changelog. No
   GTK4 widget exists to package.
2. **Downstream:** Debian (stable through unstable), Ubuntu 24.04 LTS, and Fedora 43/44/Rawhide
   all ship only the GTK3-linked builds of both libraries — `gtk-vnc`/`gtk-vnc2` 1.5.0 (Debian
   unstable, Fedora) or 1.3.1 (Ubuntu noble), and `spice-gtk`/`spice-gtk3` 0.42–0.43. There is
   no GTK4 package variant on any of these distros because there is no GTK4 build upstream to
   package.
3. **Implication for ticket 05 (GUI toolkit choice):** the premise that GTK4-native
   `gtk-vnc`/`spice-gtk` widgets exist and are available on current distros is **false** today.
   A `gtk4-rs`-based virt-manager-rs cannot embed the upstream VNC/SPICE console widgets
   as GTK4 widgets without either (a) upstream completing a GTK4 port (no evidence of one in
   progress as of 2026-08-04, and virt-viewer's own GTK4 port issue #54 remains open/unresolved
   for the same underlying reason), (b) using the GTK3 widgets via a compatibility shim/embedding
   layer, or (c) reimplementing VNC/SPICE rendering directly (e.g. via a Rust crate) rather than
   embedding the C widget.

## Sources

- gtk-vnc `meson.build` (master): https://gitlab.gnome.org/GNOME/gtk-vnc/-/raw/master/meson.build
- gtk-vnc `meson_options.txt` (master): https://gitlab.gnome.org/GNOME/gtk-vnc/-/raw/master/meson_options.txt
- gtk-vnc `NEWS` (master): https://gitlab.gnome.org/GNOME/gtk-vnc/-/raw/master/NEWS
- gtk-vnc tags: https://gitlab.gnome.org/GNOME/gtk-vnc/-/tags
- spice-gtk `meson.build` (master, via gitlab.com mirror): https://gitlab.com/spice/spice-gtk/-/raw/master/meson.build
- spice-gtk `meson_options.txt` (master, via gitlab.com mirror): https://gitlab.com/spice/spice-gtk/-/raw/master/meson_options.txt
- spice-gtk canonical repo (freedesktop.org GitLab): https://gitlab.freedesktop.org/spice/spice-gtk
- virt-viewer GTK4 port issue #54: https://gitlab.com/virt-viewer/virt-viewer/-/issues/54
- Debian package tracker — gtk-vnc: https://tracker.debian.org/pkg/gtk-vnc
- Debian package tracker — spice-gtk: https://tracker.debian.org/pkg/spice-gtk
- Ubuntu packages — libgtk-vnc-2.0-0: https://packages.ubuntu.com/libgtk-vnc-2.0-0
- Ubuntu packages — spice-gtk (noble): https://packages.ubuntu.com/source/noble/spice-gtk
- Fedora package metadata (via rpmfind.net mirror of Fedora repos) — gtk-vnc2:
  https://rpmfind.net/linux/rpm2html/search.php?query=gtk-vnc2
- Fedora package metadata (via rpmfind.net mirror of Fedora repos) — spice-gtk3:
  https://rpmfind.net/linux/rpm2html/search.php?query=spice-gtk3
