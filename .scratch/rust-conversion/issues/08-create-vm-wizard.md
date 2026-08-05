---
id: 08
title: Create VM wizard
type: grilling
status: open
blocked_by: [04, 05, 06, 07]
claimed_by: null
---

## Question

Ticket 07 flagged this as the single biggest/riskiest screen (`createvm.py`,
2106 LOC). Its structure: a `GtkNotebook` state machine of 5 top-level pages
(Name → Install → Memory/CPU → Storage → Finish) where the Install page
itself branches into 7 install-method sub-flows (ISO, URL, PXE/manual,
import-existing-disk, container-app, container-OS, Virtuozzo template), plus
conditional page-skip logic (e.g. the Storage page is skipped entirely for
container installs). It pulls in most of the shared sub-widget layer from
ticket 07 (OS list, storage config, network device panel, media combo,
storage-volume browser) and both `virtinst-core` (ticket 02's XML/domain
layer) and `libosinfo` (ticket 04) to build and validate the final `Guest`
before calling `domain create`.

What's the wizard's state-machine model in `gtk4-rs` — an enum-driven
`GtkStack`/`GtkAssistant`-equivalent with explicit page-skip transitions
mirroring `createvm.py`'s `_get_next_pagenum`/`_should_skip_disk_page`
logic, or something else — and how does per-install-method page validity
(which fields are required/hidden per branch) get represented so it's
exhaustive at compile time rather than the runtime `if instmethod ==`
branching upstream uses?

## Resolution

*(not yet resolved)*
