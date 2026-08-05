---
id: 03
title: Upstream test suite inventory
type: research
status: open
blocked_by: []
claimed_by: null
---

## Question

Pull virt-manager's upstream test suite from
https://github.com/virt-manager/virt-manager (`tests/` directory — not
shipped in the Debian-installed package at `/usr/share/virt-manager` on this
machine) and catalog what it actually covers: how many XML-comparison/golden
tests exist, which drivers/OS variants/device combinations they exercise,
and how the test harness invokes `virtinst`. This becomes the acceptance bar
for "converted correctly" — a Rust port that reproduces these XML outputs
byte-for-semantic-equivalent has real evidence of parity, not just a
plausible-looking implementation.

This feeds ticket 06 (XML modeling approach).
