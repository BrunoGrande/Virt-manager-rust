---
id: 16
title: CLI tools (virt-install, virt-clone, virt-xml)
type: grilling
status: open
blocked_by: [06, 15]
claimed_by: null
---

## Question

Flagged by ticket 15, not solved there: the map's own Destination
explicitly names `virt-install`/`virt-clone`/`virt-xml` as deliverables
alongside the GUI, and upstream's `cli.py`/`virtinstall.py`/`virtclone.py`/
`virtxml.py` are real, separate entrypoints on top of the same
`virtinst-core` — but no ticket in this session has touched them. Every
GUI-screen ticket so far has assumed a GTK event loop; these don't have
one.

What's the CLI argument-parsing approach, how much of `virtinst-core`
(devices/domain typed structs, the `Guest` builder, `connection`) do they
share with the GUI vs. need their own wrapper for, and — the one that
actually matters most — what does `virt-xml --edit` look like against
ticket 06's order-preserving DOM, given that's the exact acceptance-bar
scenario (ticket 03's `virt-xml --edit` preserving foreign `xmlns:qemu`
elements) the DOM machinery was built to satisfy and no ticket has
actually walked through it end to end yet?

## Resolution

*(not yet resolved)*
