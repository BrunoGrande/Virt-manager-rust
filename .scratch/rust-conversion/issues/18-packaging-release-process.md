---
id: 18
title: Packaging/release process
type: grilling
status: resolved
blocked_by: []
claimed_by: null
---

## Question

The map's Destination is silent on license, distribution channel, and
release/CI scope. Before this map can call itself hand-off-ready: what
license, what distribution channel(s), and what CI/build scope for v1?

## Resolution

Checked what actually exists before asking anything — confirmed this
repo has no `Cargo.toml`, no `LICENSE`, no README, blank GitHub
description, and the only CI is Claude Code review/PR-assistant
workflows (no build/release pipeline). For reference (not as a decision),
upstream's own packaging shape: `.desktop` entry, hicolor icon set, man
page, AppStream `.appdata.xml` metainfo, GSettings schema — standard
Linux desktop-app packaging.

**License: GPLv2-or-later, matching upstream.** This was a hard blocker
— no `LICENSE` file existed at all. Copied the standard, unmodified FSF
GPL-2 text (from `/usr/share/common-licenses/GPL-2`, the same text
Debian's own `virt-manager` package references rather than shipping its
own copy) directly into the repo as `LICENSE`.

**Distribution: traditional distro packages (`.deb`/`.rpm`) only, for
now.** Flatpak explicitly not pursued yet — its sandbox model is in real
tension with what this app needs to do (`qemu:///system` access, SSH
keys for console tunneling per ticket 14, raw device/socket access), and
nothing has demonstrated that tension is worth solving before the port
even exists. Revisit once there's a working build to actually sandbox.

**CI/build scope: x86_64 Linux only for v1.** "Linux only" was already
locked (map's Out of scope section); aarch64 desktop Linux is real but
nothing has shown demand for it yet on a project that doesn't have a
first release. Add it when someone actually needs it, not speculatively.

**Deliberately not decided here**: release cadence/versioning scheme,
actual CI pipeline contents (build matrix, test running, artifact
publishing), packaging metadata (desktop file, AppStream metainfo,
icons) — none of these can be meaningfully designed before there's a
`Cargo.toml` and working code to package. This ticket unblocks starting
the actual build (license and distribution shape are settled), it
doesn't design a release pipeline for code that doesn't exist yet.
