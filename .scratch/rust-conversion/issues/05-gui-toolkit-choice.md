---
id: 05
title: GUI toolkit choice (egui/eframe vs gtk4-rs)
type: grilling
status: resolved
blocked_by: [01]
claimed_by: null
---

## Question

Given the close-GTK-UI-match fidelity requirement (destination decision),
should the spec commit to `egui`/`eframe` (pure Rust, no GObject-Introspection
binding risk, but requires hand-building GTK-alike chrome and a from-scratch
console renderer) or `gtk4-rs` (real widget/theming parity by construction,
but reopens binding-generation risk for `GtkVnc`/`SpiceClientGtk`/
`Libosinfo`/`AyatanaAppIndicator` — see ticket 01's findings)?

## Resolution

**`gtk4-rs`.** Ticket 01 already showed neither toolkit gets a free upstream
console widget (no GTK4 build of `gtk-vnc`/`spice-gtk` exists), so the
console pane needs custom work either way — that neutralizes the original
"embed the real widget for free" argument for `gtk4-rs` but also neutralizes
the "avoid `GtkVnc`/`SpiceClientGtk` binding risk" argument for `egui`,
since neither design embeds those specific C widgets anymore. What's left
is the rest of the app (menus, dialogs, wizards, treeviews, headerbars,
system theming, HIG conformance, AT-SPI accessibility) — `gtk4-rs` gets that
by construction against mature, actively-maintained GI bindings; `egui`
would mean hand-building GTK-alike chrome with visible fidelity gaps and no
free accessibility stack. Given the destination's hard close-GTK-match
requirement, `gtk4-rs` wins decisively on the ~95% of the app that isn't the
console pane.

Console-rendering approach (the resulting sub-question, not a separate
ticket): both `gtk-vnc` (`Gvnc`/`GVncConnection`) and `spice-gtk`
(`SpiceClientGlib`/`SpiceSession`) split cleanly into a GTK-independent
protocol/session core plus a separate GTK3 widget layer, so a
GTK3-embedding shim is not the only non-reimplementation option. Decided
approach: start with pure-Rust protocol crates (`vnc-rs`/`rust-vnc` for VNC,
`spice-client` for SPICE) painting decoded frames into a native GTK4 canvas
— no GTK3 widget dependency either way. Escalate a given channel to
gir-generated bindings against the GTK-independent cores (`libgvnc`,
`libspice-client-glib`) only if/when the pure-Rust crate can't deliver a
required parity feature for that channel (e.g. SPICE audio, USB
redirection, clipboard sync, folder sharing) — not an upfront wholesale
commitment to FFI/gir binding work. Full console-viewer implementation
specifics (per-channel breakdown, fallback triggers) are deferred to the
console-viewer GUI-screen ticket, where real constraints will be known.

**Amendment (from ticket 14):** the real constraints came in and both
channels escalate immediately rather than starting pure-Rust. `vnc-rs`
turned out to be `tokio`-based (conflicts with ticket 09's no-async-
runtime decision) and `whitequark/rust-vnc` admits no auth/encryption at
all in its own README; `spice-client`'s audio/clipboard/USB-redirect are
confirmed unimplemented against features upstream genuinely uses. Both
channels bind `libgvnc`/`libspice-client-glib` via `gir` from the start —
the GTK-independent-core structure this ticket found still holds (that's
*why* gir-binding them doesn't reopen GTK3-in-GTK4 risk), just without
the pure-Rust-first attempt actually paying off in practice.
