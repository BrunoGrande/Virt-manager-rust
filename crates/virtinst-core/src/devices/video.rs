//! `<video>` — upstream's `devices/video.py`. Grounded in the real
//! `XMLProperty` declarations. Upstream wraps `model` in a Python
//! getter/setter property (clearing dependent fields when the model
//! changes away from `qxl`) — that's Python-level convenience logic on
//! top of the underlying binding, not part of the XPath shape itself;
//! the real XPath is plain `./model/@type` like every sibling field
//! here. `blob = XMLProperty("./model/@blob", is_onoff=True)` is the
//! same flagged on/off-boolean gap as other devices — not modeled.
//!
//! Last of ticket 12's 17 `NewDevice` variants to get a real XML
//! binding.

use virtinst_xml::XmlBound;

/// ```xml
/// <video>
///   <model type="qxl" ram="65536" vram="65536" vgamem="16384" heads="1" primary="yes"/>
/// </video>
/// ```
/// or
/// ```xml
/// <video>
///   <model type="virtio" heads="1">
///     <acceleration accel3d="yes"/>
///   </model>
/// </video>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, XmlBound)]
#[xml(tag = "video")]
pub struct DeviceVideo {
    #[xml(path = "model", attribute = "type")]
    pub model: Option<String>,

    #[xml(path = "model", attribute = "vram")]
    pub vram: Option<u32>,

    #[xml(path = "model", attribute = "vram64")]
    pub vram64: Option<u32>,

    #[xml(path = "model", attribute = "ram")]
    pub ram: Option<u32>,

    #[xml(path = "model", attribute = "heads")]
    pub heads: Option<u32>,

    #[xml(path = "model", attribute = "vgamem")]
    pub vgamem: Option<u32>,

    #[xml(path = "model", attribute = "primary")]
    pub primary: Option<bool>,

    #[xml(path = "model/acceleration", attribute = "accel3d")]
    pub accel3d: Option<bool>,
}
