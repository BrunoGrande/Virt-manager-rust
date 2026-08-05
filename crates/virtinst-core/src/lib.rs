//! Domain/device XML modeling, libvirt connection handling, and
//! install/clone logic — shared by the GUI (`virt-manager`) and CLI
//! tools (`virtinst-cli`). No GTK dependency: see
//! `.scratch/rust-conversion/issues/15-virtinst-core-crate-boundaries.md`
//! for why this is one crate with these module boundaries, and which
//! upstream `virtinst/*.py` file each one replaces.
//!
//! Deliberately *not* ported: upstream's `xmlbuilder.py`/`xmlapi.py`
//! runtime dynamic-dispatch descriptor engine (ticket 06) — replaced by
//! the typed-struct + XPath derive-macro + `edit-xml`-backed DOM
//! approach these modules use instead.

pub mod capabilities;
pub mod cloner;
pub mod connection;
pub mod devices;
pub mod domain;
pub mod install;
pub mod osinfo;

mod guest;
pub use guest::Guest;

#[cfg(test)]
mod xml_binding_tests;
