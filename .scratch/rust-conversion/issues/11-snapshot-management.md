---
id: 11
title: Snapshot management
type: grilling
status: resolved
blocked_by: [06, 07, 09, 10]
claimed_by: null
---

## Question

Spun out of ticket 10: `details/snapshots.py` (853 LOC) is a real feature —
create/delete/revert VM snapshots, browsable list/tree of existing
snapshots with their own metadata (description, timestamp, VM state at
capture) — living as a tab inside the VM details window but substantial
enough on its own (bigger than several entire other screens per ticket
07's inventory) to need its own architectural pass rather than being
folded into ticket 10.

What's the snapshot list/tree widget, how does create/revert/delete route
through ticket 09's async-job wrapper, and how does snapshot XML
(external vs internal snapshots, disk-only vs full-system) interact with
ticket 06's XML modeling approach?

## Resolution

Resolved directly against precedent rather than a fresh grilling round —
almost everything here is already settled by tickets 06/07/08/09/10/12;
traced `snapshots.py` to confirm each decision actually transfers rather
than assuming it does, per the mistake caught before writing this up (see
below).

**List widget: `GtkListBox`, but verify-checked, not precedent-copied
blind.** First read of the model (`Gtk.ListStore(str, str, str, str, str,
bool)`, six columns) looked like a `GtkColumnView`-shaped tabular grid,
which would NOT have transferred from ticket 10's `GtkListBox` reasoning
(that was justified by "single small list," not "single-column"). Reading
the actual `TreeViewColumn` setup corrected that: it's **one** rendered
column (icon + markup-formatted label + a "current snapshot" emblem,
packed together) backed by six data fields (name, label, tooltip,
icon-name, sort-key, current-flag) — not six visual columns. So it's a
short, single-rich-content-per-row list after all, and `GtkListBox` with a
custom row widget (icon + label + current-badge) holds, for the right
reason this time. Also verified directly: **no parent/child snapshot
hierarchy is displayed** (grepped for `parent`/`GetParent` — the only hits
are GTK dialog-transient-for parents, nothing snapshot-tree-related), so
this is genuinely flat, confirming a list widget over a tree one.
Row-separator grouping (`set_row_separator_func`, splitting
internal/external snapshot groups) has no direct `GtkListBox` equivalent
in the same form — implementation detail (insert a separator row/widget
manually), not a design fork.

**Create/revert/delete: already on ticket 09's rails.** All three route
through `vmmAsyncJob`/`simple_async` in upstream — direct port to the
shared async-job wrapper from ticket 09, no new mechanism. Revert
("Start snapshot") is single-selection only, with two different
confirm-dialog messages depending on whether the VM is active (disk-state
discarded) or not (disk+config state discarded) — same confirm-then-async
pattern already used for Add Hardware's hotplug-failure fallback. Delete
supports multi-select — a real capability, not an architectural fork
(batch through the same wrapper per-item or as one job, implementation
detail).

**Snapshot creation modes: `enum NewSnapshot`.** Internal / external /
disk-only are mutually exclusive (`_get_mode()`, gated by
`supports_externalSnapshot()` capability and derived as disk-only when
`not vm.is_active() and mode == external`) — same compile-time-exhaustive
enum-of-structs pattern as tickets 08/12's `InstallMethod`/`NewDevice`.
The "mixing internal and external snapshots" check is a soft warning
comparing the new snapshot's mode against only the *current* (most
recent) snapshot's mode, not a full-set scan — a yes/no confirm the user
can override, same shape as every other confirm-then-proceed dialog
already ported elsewhere.

**Snapshot XML ↔ ticket 06.** Opaque call-through, same deferral as
device-defaults in ticket 12: snapshot XML construction is a
`virtinst-core` concern (build the typed struct, let ticket 06's
machinery serialize it), not something this GUI-screen ticket needs to
design. No new interaction to resolve.

**No open forks left for this ticket.** Everything above was either
directly verified against source or is a direct, checked application of
an already-established pattern — nothing here needed a fresh decision the
existing map didn't already make.
