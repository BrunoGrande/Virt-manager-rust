---
id: 09
title: Async / background-task model
type: grilling
status: open
blocked_by: [05]
claimed_by: null
---

## Question

Spun out of ticket 08: multiple upstream screens run background work off
the GTK main thread and resume it on completion — `createvm.py`'s
OS-detection probe on Install→Next, `asyncjob.py` (the shared progress
modal used by any long-running libvirt call), `migrate.py`, `clone.py`.
None of this has a Rust equivalent decided yet.

What's the async/background-task model for a `gtk4-rs` app — a full async
runtime (`tokio`) bridged into the GLib main loop, or the idiomatic
`gtk4-rs` pattern of `glib::MainContext::spawn_local` + channels back to
widget-owning code, or plain OS threads + `glib::idle_add`/channels (closer
to what upstream's Python actually does — a real thread reporting progress
via `GLib.idle_add`)? And what's the shared "modal blocks input, shows
progress, is cancellable" widget wrapper (`asyncjob.ui` equivalent) that
every long-running call routes through?

## Resolution

*(not yet resolved)*
