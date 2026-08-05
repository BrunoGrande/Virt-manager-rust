//! Wraps the `virt` crate's `Connect` (libvirt bindings, ticket 02) plus
//! gir-bound `libvirt-glib` for event integration (ticket 09) — upstream's
//! `connection.py`.
//!
//! Also where two other tickets' concrete findings land:
//! - **Live + persistent duality** (ticket 10 deferred it, ticket 12
//!   found the concrete shape): `Domain::attach_device()` attempts live
//!   hotplug if the domain is active, degrading to persistent-only with
//!   a confirm on failure; `Domain::add_device()` always also updates
//!   the persistent config. Two explicit calls, not one
//!   `AFFECT_LIVE|AFFECT_CONFIG` flag.
//! - **Driver detection** (ticket 17): trivial URI-scheme-prefix checks
//!   — `is_qemu()`/`is_xen()`/`is_lxc()`/`is_bhyve()`/`is_vz()` — feeding
//!   the per-driver device-default chains in [`crate::devices`].

// TODO: pub struct Connection { inner: virt::connect::Connect, ... }
// TODO: pub struct Domain { ... attach_device(), add_device() (ticket 12) }
// TODO: libvirt-glib event-loop registration (ticket 09) — LibvirtGLib::init()
// + event_register() equivalent, once the gir binding exists.
