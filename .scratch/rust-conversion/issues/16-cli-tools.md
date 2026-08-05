---
id: 16
title: CLI tools (virt-install, virt-clone, virt-xml)
type: grilling
status: resolved
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

Grilled across 2 rounds, grounded in `cli.py` (5379 LOC, mostly one
engine), `virtinstall.py`, `virtclone.py`, and `virtxml.py` directly.

**CLI option-string syntax: preserved exactly, via a derive-macro
extension, not a redesign.** Traced `VirtCLIParser`/`add_arg` — upstream's
`--disk path=/foo,bus=virtio` isn't the same runtime dynamic-dispatch
engine ticket 06 already rejected (`xmlbuilder.py`); it's a hand-curated,
declarative cliname→propname mapping, one parser class per device/domain
area, evaluated at class-definition time. That maps cleanly onto
extending ticket 06's derive macro to also emit a CLI sub-key parser per
typed struct (`#[cli(key = "path")]`-style), preserving upstream's exact
flag surface — a genuine user-facing compatibility commitment (existing
scripts, docs, muscle memory), not new complexity for its own sake, since
it's the same declarative-mapping shape upstream already uses, just
compile-time-checked instead of a runtime registry.

**One `virtinst-core`, thin CLI-shell on top — confirms ticket 15's call
was right.** No GTK dependency in core; CLI gets stdout progress and
`y/n` prompts instead of a modal and confirm dialogs, sitting on the exact
same core the GUI uses. Nothing found while researching this ticket
needed core itself to change.

**`virt-xml`'s action model: direct port.** `--edit` (index/`all`/
selector-string), `--add-device`, `--remove-device`, `--build-xml`
(confirmed it works standalone with no connection at all — pure
XML-fragment construction), plus live-vs-persistent handling
(`start_domain_transient` for hotplug-only, `update_changes` for
live-and-persist) — the same `attach_device()`-then-`add_device()`
pattern ticket 12 already found and ticket 10 deferred to. Nothing here
is GUI-specific; it's the same `virtinst-core` operations already
settled, invoked from argument parsing instead of button clicks.

**`--edit` is the first end-to-end validation of ticket 06's DOM, and it
holds.** Ticket 08 (Create VM) bypassed the order-preserving DOM entirely
— fresh document, nothing to preserve. `virt-xml --edit` is the exact
opposite case: parse an existing (possibly hand-crafted, foreign-namespace-
laden) document once per process invocation, apply one edit via the typed
struct's XPath binding, serialize, exit. This "cold start, one edit, done"
pattern is actually the *simpler* case relative to what ticket 06's DOM
was already designed for — ticket 10's in-process, multi-edit Apply
session is the harder one. No wrinkle found.

**`virt-install` reuses `InstallMethod` as-is — with two real additions
found by checking its actual argument groups, not assumed.**
- `--unattended`/`--cloud-init` are **orthogonal modifiers layered on
  top of** a base install method (`--location URL --unattended` = URL
  install *plus* answer-file injection), not separate mutually-exclusive
  methods — they thread through as extra fields on whichever
  `InstallMethod` variants support them (e.g. `Url { location,
  extra_args, unattended: Option<UnattendedConfig> }`), rather than
  becoming their own variants or forking the enum CLI-vs-GUI.
- `--reinstall DOMAIN` has **no GUI-wizard equivalent at all** — it
  re-runs install-method construction against an *existing* domain,
  ignoring all its other config. Modeled as its own distinct CLI-only
  operation, not bent to fit `WizardState`. Closer in shape to `virt-xml
  --edit` (mutate an existing domain's install-relevant config) than to
  ticket 08's fresh-`Guest`-only wizard.

**`virt-clone` — settled by precedent, no fresh fork.** Thin wrapper
(247 LOC) around `cloner.py`'s `Cloner`: parse source VM, override
MAC/disk paths, validate, define new domain. Same "fresh document, no
DOM-preservation needed" shape as ticket 08. Worth noting `cloner.py`
was missing from ticket 15's module table — minor addition, not a design
change.
