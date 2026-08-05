//! Wraps gir-bound `libosinfo` (ticket 04) — upstream's `osdict.py`.
//! Pure GObject C API, no GTK in the link chain; `osinfo-db` is a
//! separately-versioned, distro-installed data package with its own
//! update tooling, not something this crate vendors.

// TODO: pub struct OsDb { ... }
// TODO: pub struct OsInfo { ... }
