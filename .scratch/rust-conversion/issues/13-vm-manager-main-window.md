---
id: 13
title: VM Manager main window
type: grilling
status: open
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

## Resolution (partial — one fork below needs a call before this can close)

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

1. **Port `TreeStore`/`TreeView`/`CellRendererSparkline` close to 1:1** —
   smallest diff from upstream's actual mechanism, reuses a widget
   `gtk4-rs` still exposes, but it's a deprecated API surface for a
   Rust codebase starting fresh in 2026.
2. **Migrate to `GtkListView` + `GtkTreeListModel` + `GtkTreeExpander`**
   (GTK4's modern tree-list mechanism) with a custom
   `GtkListItemFactory`/widget standing in for the sparkline cell
   renderer — the "not deprecated" answer, but real new complexity
   (`TreeListModel`'s tree-flattening semantics, factory bind/unbind
   lifecycle) that no other ticket in this map has had to design yet,
   for the one screen in the whole app that's a genuine two-level tree
   rather than the flat lists tickets 10/11 already settled.

Not deciding this one by precedent-matching or unilaterally — flagging it
back rather than guessing at a call that's really about how much
deprecated-API debt this project wants to start with.
