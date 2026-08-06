//! `<watchdog>` — upstream's `devices/watchdog.py`. Grounded in the
//! real `XMLProperty` declarations: the simplest device yet, two flat
//! self-attributes and nothing else.

use virtinst_xml::XmlBound;

/// ```xml
/// <watchdog model="i6300esb" action="reset"/>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, XmlBound)]
#[xml(tag = "watchdog")]
pub struct DeviceWatchdog {
    #[xml(attribute = "model")]
    pub model: Option<String>,

    #[xml(attribute = "action")]
    pub action: Option<String>,
}
