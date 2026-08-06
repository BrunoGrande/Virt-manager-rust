//! `<redirdev>` — upstream's `devices/redirdev.py`. Grounded in the
//! real `XMLProperty`/`XMLChildProperty` declarations: two self
//! attributes plus the same `CharSource` `<source>` subset already
//! modeled for `DeviceSerial` and `DeviceSmartcard`. No gaps.

use virtinst_xml::XmlBound;

/// ```xml
/// <redirdev bus="usb" type="spicevmc"/>
/// ```
/// or
/// ```xml
/// <redirdev bus="usb" type="tcp">
///   <source mode="connect" host="localhost" service="4000"/>
/// </redirdev>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, XmlBound)]
#[xml(tag = "redirdev")]
pub struct DeviceRedirdev {
    #[xml(attribute = "bus")]
    pub bus: Option<String>,

    #[xml(attribute = "type")]
    pub redirdev_type: Option<String>,

    #[xml(path = "source", attribute = "mode")]
    pub source_mode: Option<String>,

    #[xml(path = "source", attribute = "host")]
    pub source_host: Option<String>,

    #[xml(path = "source", attribute = "service")]
    pub source_service: Option<u32>,
}
