//! `<graphics>` — upstream's `devices/graphics.py` (`DeviceGraphics`).
//! Grounded in the real `XMLProperty` declarations before writing
//! anything: unlike disk/network, most fields here are flat attributes
//! on `<graphics>` itself (`type`, `port`, `autoport`, `listen`,
//! `passwd` are all `./@...`) — the SPICE-specific channel config is
//! the nested exception (`image_compression = XMLProperty("./image/@compression")`),
//! included as one representative case rather than modeling every SPICE
//! sub-channel now. Console-rendering itself is gir-bound per tickets
//! 05/14 — this struct is XML binding only, same as every other device
//! struct so far.

use virtinst_xml::XmlBound;

/// ```xml
/// <graphics type="vnc" port="-1" autoport="yes" listen="127.0.0.1"/>
/// ```
/// or
/// ```xml
/// <graphics type="spice" autoport="yes">
///   <image compression="auto_glz"/>
/// </graphics>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, XmlBound)]
#[xml(tag = "graphics")]
pub struct DeviceGraphics {
    #[xml(attribute = "type")]
    pub graphics_type: Option<String>,

    #[xml(attribute = "port")]
    pub port: Option<String>,

    #[xml(attribute = "tlsPort")]
    pub tls_port: Option<String>,

    #[xml(attribute = "autoport")]
    pub autoport: Option<bool>,

    #[xml(attribute = "listen")]
    pub listen: Option<String>,

    #[xml(attribute = "passwd")]
    pub passwd: Option<String>,

    /// SPICE-only. The other SPICE sub-channels (`streaming`,
    /// `clipboard`, `mouse`, `filetransfer`, `gl`, `zlib`) follow the
    /// exact same `path = "...", attribute = "..."` shape — added as
    /// each is actually needed rather than modeled speculatively now.
    #[xml(path = "image", attribute = "compression")]
    pub image_compression: Option<String>,
}
