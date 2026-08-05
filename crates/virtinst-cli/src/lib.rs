//! Shared CLI-shell code for `virt-install`/`virt-clone`/`virt-xml`
//! (ticket 16) — the thin, GTK-free counterpart to the GUI-shell layer
//! in `virt-manager`: stdout progress instead of a modal, `y/n` prompts
//! instead of confirm dialogs. Sits on the exact same `virtinst-core`
//! the GUI uses; nothing here reaches back into core.
//!
//! Also where the CLI option-string parser lives — the derive-macro
//! extension (ticket 16) that preserves upstream's exact
//! `--disk path=/foo,bus=virtio` syntax: a declarative cliname→propname
//! mapping per typed struct in `virtinst_core::devices`/`domain`, not a
//! redesigned flag surface.

// TODO: shared option-string parsing (cli_opt derive-macro output),
// progress reporting, confirm-prompt helpers.
