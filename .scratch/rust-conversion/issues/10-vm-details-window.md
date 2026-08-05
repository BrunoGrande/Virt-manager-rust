---
id: 10
title: VM details window
type: grilling
status: resolved
blocked_by: [05, 06, 07, 09]
claimed_by: null
---

## Question

Ticket 07 flagged this as the biggest screen (`details/details.py`, 2522
LOC — the single biggest controller in the app — plus `details/snapshots.py`
at 853 LOC). Its structure: a dynamic `hw-list` sidebar (fixed rows for
General/OS/Stats/CPU/Memory/Boot, plus one row per actual device on the
VM) whose selection swaps a detail pane, wired up against the shared
sub-widget layer ticket 07 catalogued (`fsdetails`, `gfxdetails`,
`tpmdetails`, `vsockdetails`, `netlist`, `addstorage`, `mediacombo`,
`xmleditor`). Editing accumulates `EDIT_*` flags on the currently-selected
row into `_active_edits`; an Apply button gates on that; switching rows
with pending edits prompts confirm-or-discard.

Unlike ticket 08's Create VM wizard (fresh document, no DOM needed), this
is the canonical *editing* surface ticket 06's order-preserving DOM was
built for. What's the `gtk4-rs` shape of the `hw-list` sidebar, the
edit/Apply architecture, and how much of this ticket's scope should include
Snapshot management and the live/persistent (`AFFECT_LIVE`/
`AFFECT_CONFIG`) config duality?

## Resolution

Grilled across 2 rounds, grounded directly in `details.py`: traced
`_apply_vcpus` (CPU page can have `EDIT_VCPUS`/`EDIT_CPU`/`EDIT_TOPOLOGY`
simultaneously pending, merged into one `kwargs` dict for one
`define_cpu()` call), `_has_unapplied_changes`/`_hw_changed_cb` (row-switch
confirm-or-discard), `_config_remove`/`_config_remove_clicked_cb` (Remove
Device is its own immediate action, not routed through the edit/Apply
system at all), and confirmed the Stats/Performance tab has no local
`GLib.timeout_add` — it rides on `engine.py`'s shared "Tick thread," i.e.
ticket 09's already-decided async model, not a new mechanism.

**Scope.** Ticket 10 covers the `hw-list`-driven shell — General/OS/Stats/
CPU/Memory/Boot tabs, per-device edit panels, the apply/dirty-tracking
architecture. Two things explicitly carved out:
- **Snapshot management** (853 LOC, a real feature — create/delete/revert,
  not just another field-editing panel) gets its own ticket, same
  size/complexity bar that got it flagged separately in ticket 07's
  inventory.
- **Live vs. persistent config** (`AFFECT_LIVE`/`AFFECT_CONFIG`): not
  visible at the GUI-dispatcher layer in upstream either — it's decided
  underneath `vm.define()`, presumably in the domain object. Ticket 10
  treats "apply this edit" as an opaque call into `virtinst-core`; the
  live/persistent split is that not-yet-opened ticket's problem, not this
  screen's.

Console viewer and XML editor tab were already settled elsewhere (ticket
05's deferred console-viewer ticket; ticket 07's shared-widget catalog,
respectively) — not re-litigated here.

**`hw-list` widget.** Plain `GtkListBox` with directly-constructed row
widgets, not `GtkListView`/`GtkColumnView` + factory. This list is a dozen
or so rows (six fixed + per-device), not the thousands-of-rows virtualized
case `ListView`'s factory machinery pays for itself on.

**Edit/Apply model.** Direct port of upstream's architecture — pending
edits scoped to the currently-selected row, an Apply button gated on dirty
state, confirm-or-discard on row-switch. This is deliberate product
behavior (batching related field changes into one atomic define call), not
incidental Python plumbing; changing it would be a product decision this
ticket has no mandate to make.

**Pending-edit data shape.** A per-`HW_LIST_TYPE` struct with `Option<T>`
per editable field (e.g. `CpuPending { vcpus: Option<u32>, model:
Option<String>, topology: Option<TopologyPending> }`), held as one
`Option<PendingEdit>` (an enum over each type's pending-struct) on the
window's state — the same "plain struct, `Option` fields" shape ticket 08
used for `WizardState`, scoped to one row's lifetime instead of the whole
wizard's. Chosen over a single enum-of-structs (ticket 08's `InstallMethod`
pattern doesn't fit — those variants were mutually exclusive; a row's
fields aren't, per the CPU-page finding above) and over a flat
`Vec<EditFlag>` (would just recreate upstream's "flags checked ad hoc"
pattern this project has already moved past once).

**Remove Device.** Settled by inspection, not a fork: it's an immediate
action with its own confirmation, entirely separate from the edit/Apply
system — routes straight through ticket 09's shared async-job wrapper
(spawn the removal, show the progress modal). No new mechanism needed.

Per-widget layout detail (boot-order drag-reorder, exact per-tab field
arrangement) is left to implementation, same restraint ticket 08 used —
this ticket settles architecture, not pixels.
