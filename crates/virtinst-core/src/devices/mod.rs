//! One typed struct per device type — upstream's `devices/*.py` (20 files).
//!
//! Two enums assemble these for different flows, both compile-time
//! exhaustive over the same per-device structs here:
//! - [`NewDevice`](crate::devices) (ticket 12, Add Hardware wizard):
//!   17 mutually-exclusive variants, one per device type.
//! - `InstallMethod`'s device-adjacent fields (ticket 08, Create VM).
//!
//! Per-driver default resolution (ticket 17) lives as a method on each
//! type here, taking [`crate::capabilities`] — a direct-ported ordered
//! decision chain per field (upstream's `set_defaults`/`_default_bus`
//! shape), not a driver-strategy trait. Only ~9 of these files actually
//! branch on driver family upstream; this isn't a pervasive concern.

pub mod disk;
pub use disk::DeviceDisk;

pub mod network;
pub use network::DeviceNetwork;

pub mod graphics;
pub use graphics::DeviceGraphics;

pub mod list;
pub use list::DeviceList;

// TODO: the remaining 14 variants (see docs/research/gui-screen-inventory.md
// and ticket 12's NewDevice enum): Controller, Input, Sound, Hostdev,
// Char, Video, Watchdog, Filesystem, Smartcard, UsbRedir, Tpm, Rng,
// Panic, Vsock.
