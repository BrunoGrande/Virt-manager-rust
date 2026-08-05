---
id: 14
title: Console-viewer implementation specifics
type: research
status: open
blocked_by: [05, 09]
claimed_by: null
---

## Question

Ticket 05 deferred per-channel specifics: pure-Rust protocol crates first
(`vnc-rs`/`rust-vnc` for VNC, `spice-client` for SPICE), escalating to
gir-bound `libgvnc`/`libspice-client-glib` only if a crate can't deliver a
required parity feature for that channel. What does that escalation
actually look like per channel, given what the crates really support
today — and does the SSH-tunnel transport (`sshtunnels.py`) need its own
design pass?

## Resolution (partial — real research done, one conflict needs your call)

**SSH tunnel transport — settled, no crate needed.** Traced
`sshtunnels.py`'s `_Tunnel.open()`: it's `os.fork()` +
`os.execlp("ssh", "ssh", host)` with the socket fd `dup2`'d onto the
child's stdin/stdout — literally shelling out to the system `ssh` binary
as a netcat-mode transport, not using any SSH library. Direct Rust
equivalent: `std::process::Command` piping a socket, no SSH crate in the
dependency tree at all.

**VNC — a real conflict with ticket 09, not a clean pick.**
- `vnc-rs` (checked crates.io directly: v0.5.3, released Feb 2026, active):
  supports Tight/ZRLE/Raw encodings, tested against TightVNC and macOS's
  VNC server — but it's **built on `tokio`**, an async runtime. Ticket 09
  decided *no async runtime anywhere in this app*, specifically because
  nothing needed one; this crate would be the first exception.
- `whitequark/rust-vnc`: sync, no tokio — but its own README states **"No
  encryption or authentication"** outright, and it's a thin, 35-commit
  project. Likely can't even do basic VNC password auth against a real
  libvirt/QEMU VNC display, let alone anything TLS-based.
- Neither crate's VeNCrypt/TLS or clipboard (cut-text) support is
  confirmed either way — didn't chase this down to source level, it's a
  real gap in current knowledge, not a settled "good enough."

**SPICE — ticket 05's escalation condition is already triggered, not
hypothetical.** `spice-client` (pure-Rust, the crate ticket 05 named):
display channel + input work; **audio, clipboard, and USB redirection are
explicitly in-progress/planned, not implemented** (checked directly).
Upstream's actual `SpiceViewer` class (`viewers.py`) uses exactly those
three features — `SpiceClientGLib.Audio`, `SpiceClientGtk.UsbDeviceWidget`,
clipboard sync via the main channel. So the SPICE channel needs the
gir-bound `libspice-client-glib` escalation *today*, for real parity, not
as a someday-maybe fallback.

## Open forks — need your call, not a guess

1. **VNC crate + the tokio question.** Three real options, not
   precedent-covered by anything already on the map: (a) use `vnc-rs`,
   accept `tokio` — but confined to a single isolated worker thread
   (spin up a single-threaded tokio runtime inside the same dedicated OS
   thread ticket 09 already uses for blocking-call offload, so it never
   touches the main GLib loop), not a blanket reversal of ticket 09;
   (b) use `whitequark/rust-vnc` despite the missing-auth gap, and build
   auth/TLS on top by hand; (c) skip the pure-Rust-first approach
   entirely for VNC specifically and go straight to gir-bound `libgvnc`,
   same as SPICE is about to need anyway.
2. **How much more research before deciding #1.** VeNCrypt/TLS and
   clipboard support for both crates is still unconfirmed — worth 15
   minutes reading actual source/tests before locking in, or is "no
   confirmed TLS support in either" itself enough to just pick option
   (c) above and stop chasing pure-Rust VNC further?

Not deciding either of these — they're judgment calls about how much
new-dependency risk (tokio-in-a-box) versus escalation-by-default this
project wants, and the map has no precedent that settles it either way.
