//! Capability queries — upstream's `capabilities.py`/`domcapabilities.py`.
//!
//! `DomainCapabilities` gets its own lightweight deserialize-only path,
//! deliberately *separate* from ticket 06's order-preserving DOM (ticket
//! 17): it's fetched fresh per-connection via
//! `virConnectGetDomainCapabilities`, cached per-`Guest`, and never
//! edited or serialized back — none of the DOM's preservation machinery
//! earns its keep on a read-only, cached path.

// TODO: pub struct DomCaps { ... } — plain XML→struct parse, no DOM.
