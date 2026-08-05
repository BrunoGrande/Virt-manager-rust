//! One typed struct per domain-XML subsystem — upstream's `domain/*.py`
//! (18 files: cpu, clock, os, features, seclabel, sysinfo, vcpus,
//! xmlnsqemu foreign-namespace preservation, etc.).
//!
//! Bound to XPaths via the derive macro from ticket 06, read/written
//! through the `edit-xml`-backed order-preserving DOM. `xmlnsqemu.py`'s
//! foreign-namespace preservation is exactly what makes the DOM
//! approach necessary in the first place (ticket 03's acceptance bar).

// TODO: one module per domain subsystem (cpu, os, clock, features,
// seclabel, sysinfo, vcpus, pm, numatune, memtune, ...).
