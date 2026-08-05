---
id: 13
title: VM Manager main window
type: grilling
status: resolved
blocked_by: [07, 09, 10]
claimed_by: null
---

## Question

Last of ticket 07's four biggest/riskiest screens (`manager.py`, 1026
LOC) — this is the app's front door: a `Gtk.TreeStore`-backed two-level
list (connections as parent rows, VMs as child rows under each), a `Name`
column plus toggleable live-stats columns rendered with a *custom*
`CellRendererSparkline` (`lib/graphwidgets.py`, not a stock GTK cell
renderer), and a `row_activated` dispatch that opens ticket 10's VM
details for a VM row, reconnects a disconnected connection, or opens
Host details (`host.py` — cataloged by ticket 07, not yet its own ticket)
for a connected one.

## Resolution

Most of this is already settled by precedent, verified against source
rather than assumed:

- **Row-activation dispatch**: a direct port of `row_activated`'s
  three-way logic (VM row → ticket 10's details window; disconnected
  connection → reconnect; connected connection → Host details) — no new
  decision, just wiring into screens this map already owns or has
  cataloged.
- **Live stats feed**: rides ticket 09's already-decided model (OS
  thread + `glib::MainContext::channel()`, fed by the same Tick-thread-
  equivalent polling ticket 10's Performance tab already relies on) — not
  a new mechanism.
- **Delete/context-menu actions** (`do_delete`, connection context menu):
  route through ticket 09's shared async-job wrapper, same as every other
  long-running action ported so far.

**The one real fork — not resolved here on purpose.** Checked GTK4's
actual typelib on this machine (`gir1.2-gtk-4.0` isn't installed by
default, pulled it from the GNOME 46 snap's bundled typelib to verify
directly rather than assume): `Gtk.TreeView`/`Gtk.TreeStore`/
`Gtk.CellRendererText` **are still present** in GTK4 — deprecated since
4.10, but shipped and functional, not removed the way I'd have guessed.
That means this isn't a forced migration; there's a genuine choice with no
precedent-covered answer:

1. Port `TreeStore`/`TreeView`/`CellRendererSparkline` close to 1:1 —
   smallest diff from upstream's actual mechanism, but deprecated API for
   a Rust codebase starting fresh in 2026.
2. **Migrate to `GtkListView` + `GtkTreeListModel` + `GtkTreeExpander`.**
   **Chosen.** Not deprecated, matches the project's own general lean
   (ticket 05 picked `gtk4-rs` specifically for close-fidelity/long-term
   fit, not smallest-diff-from-Python) at the cost of new machinery no
   other ticket has needed yet.

**Shape.** `GtkTreeListModel` wraps a root `GListModel` of connections;
each connection row supplies a child-model factory returning a
`GListModel` of its VMs (empty for a connection with no VMs, `None` for a
VM row — `TreeListModel` uses that to know a row is a leaf). One
`GtkListItemFactory` binds each row: a `GtkTreeExpander` provides the
disclosure triangle and indentation `TreeView`'s `set_level_indentation`
did by hand, wrapping a row-content widget that switches between
"connection row" (name + connection-state text) and "VM row" (name +
status icon + OS-inspection icon + the sparkline) based on which the
bound `GtkTreeListRow`'s item actually is. The sparkline itself becomes a
small custom widget (`GtkWidget` subclass drawing via `Snapshot`/`cairo`
in its own `snapshot()` vfunc) standing in for `CellRendererSparkline` —
same rendering logic ported, different embedding mechanism (a real child
widget in the factory's bound row, not a cell renderer attribute).
Toggleable stat columns become additional `GtkColumnView` columns if the
per-column toggle behavior is worth keeping, or folded into one row
widget if not — left as an implementation call, not an architectural one.
