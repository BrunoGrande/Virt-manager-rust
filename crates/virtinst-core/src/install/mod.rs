//! Backing logic for `InstallMethod`'s variants (ticket 08's Create VM
//! wizard enum, reused as-is by `virt-install` per ticket 16) — upstream's
//! `install/*.py`: cloud-init, unattended-install answer files, URL
//! install-tree detection/fetching.
//!
//! `--unattended`/`--cloud-init` are orthogonal modifiers threaded
//! through as extra fields on whichever `InstallMethod` variants support
//! them (ticket 16) — not separate variants of their own.

// TODO: pub struct CloudInit { ... }
// TODO: pub struct Unattended { ... }
// TODO: pub mod url_detect;
