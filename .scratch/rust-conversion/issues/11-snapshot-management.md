---
id: 11
title: Snapshot management
type: grilling
status: open
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

*(not yet resolved)*
