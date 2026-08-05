//! `virt-install` — reuses `InstallMethod` (ticket 08) as-is.
//! `--unattended`/`--cloud-init` thread through as extra fields on
//! relevant variants; `--reinstall` is its own CLI-only operation with
//! no wizard equivalent, closer in shape to `virt-xml --edit` than to
//! Create VM (ticket 16).

fn main() {
    // TODO
}
