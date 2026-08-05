---
id: 07
title: GUI screen inventory
type: research
status: resolved
blocked_by: [05, 06]
claimed_by: null
---

## Question

The map's "Not yet specified" section lists 15 GUI screens by name but
doesn't locate them in the upstream source or size them. Before writing a
per-screen spec ticket for any of them, catalog every `.ui` file against its
Python controller, with LOC as a rough complexity signal, so screen tickets
can be prioritized and scoped instead of guessed at.

## Resolution

Inventoried all 31 `.ui` files under `/usr/share/virt-manager/ui/` against
their controllers. The map's 15-screen list maps onto 17 top-level screens
(systray and the VM right-click menu were one map item, are two controllers;
"connection manager" turned out to just be the VM Manager main window, no
separate screen exists) plus a **13-item shared sub-widget layer the map
didn't call out** — storage/network/graphics/filesystem/TPM/vsock device
panels, the OS-list, storage-volume browser, async-job modal, and three
create-* wizards (network/pool/volume) that are reused across multiple
top-level screens rather than owned by one. Total: ~23.6k LOC of controller
code. Full table with file:line-equivalent detail:
`docs/research/gui-screen-inventory.md`.

Biggest screens by controller LOC, i.e. where per-screen tickets carry the
most risk: Create VM wizard (2106), VM details window (2522, before even
counting its snapshots tab at 853), Add Hardware wizard (1622), VM Manager
main window (1026).

This doesn't spec any screen — it's the map for where to open screen
tickets next, deliberately kept to inventory depth (ponytail: no per-screen
design work here until a ticket actually needs it).
