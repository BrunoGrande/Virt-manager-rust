//! `<rng>` — upstream's `devices/rng.py`. Grounded in the real
//! `XMLProperty` declarations. `device = XMLProperty("./backend[@model='random']")`
//! looks predicate-filtered like `char.py`'s skipped fields, but it
//! isn't actually inaccessible here: it has no `@attr`, so it's just
//! `<backend>`'s own text content, and `backend_model`/`backend_type`
//! already bind that same element directly via plain `./backend/@...`.
//! The predicate in upstream is a defensive/scoping detail, not a
//! different element — modeled as `path = "backend", text` with no gap.

use virtinst_xml::XmlBound;

/// ```xml
/// <rng model="virtio">
///   <backend model="random">/dev/urandom</backend>
/// </rng>
/// ```
/// or
/// ```xml
/// <rng model="virtio">
///   <backend model="egd" type="udp"/>
///   <rate bytes="1024" period="2000"/>
/// </rng>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, XmlBound)]
#[xml(tag = "rng")]
pub struct DeviceRng {
    #[xml(attribute = "model")]
    pub model: Option<String>,

    #[xml(path = "backend", attribute = "model")]
    pub backend_model: Option<String>,

    #[xml(path = "backend", attribute = "type")]
    pub backend_type: Option<String>,

    /// `<backend>`'s own text content — the `/dev/urandom`-style device
    /// path for a `random`-model backend.
    #[xml(path = "backend", text)]
    pub backend_device: Option<String>,

    #[xml(path = "rate", attribute = "bytes")]
    pub rate_bytes: Option<u32>,

    #[xml(path = "rate", attribute = "period")]
    pub rate_period: Option<u32>,
}
