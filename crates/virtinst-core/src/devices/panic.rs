//! `<panic>` — upstream's `devices/panic.py`. Genuinely a one-field
//! device once traced: `model = XMLProperty("./@model")` is the only
//! real data.
//!
//! `set_stub = XMLProperty(".", is_bool=True)` is **not modeled, on
//! purpose** — it isn't a binding gap the way the `..`/`[@mode=...]`
//! XPath cases were. Its own XPath is `.` (the element itself), and
//! upstream only ever *sets* it (`self.set_stub = True` in
//! `set_defaults`, when no `model` was picked) to stop its dynamic
//! engine from pruning an otherwise-attribute-less `<panic/>` out of the
//! serialized document entirely. This crate's derive macro never prunes
//! elements it didn't create in the first place — ticket 06 already
//! decided not to port that engine — so there's nothing for a
//! `set_stub`-equivalent to signal here. A bare `<panic/>` (no `model`)
//! reads back as `model: None` and round-trips as `<panic/>` either way.

use virtinst_xml::XmlBound;

/// ```xml
/// <panic model="isa"/>
/// ```
/// or, the stub case telling libvirt to pick a default:
/// ```xml
/// <panic/>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, XmlBound)]
#[xml(tag = "panic")]
pub struct DevicePanic {
    #[xml(attribute = "model")]
    pub model: Option<String>,
}
