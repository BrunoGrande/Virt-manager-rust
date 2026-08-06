//! `<interface>` — upstream's `devices/interface.py` (`DeviceInterface`).
//! Field subset chosen to match `DeviceDisk`'s scope: the common
//! `type='network'` shape, checked against upstream's actual
//! `XMLProperty` XPath declarations rather than assumed from memory —
//! `network = XMLProperty("./source/@network")`,
//! `macaddr = XMLProperty("./mac/@address")`, etc.

use virtinst_xml::XmlBound;

/// ```xml
/// <interface type="network">
///   <mac address="52:54:00:12:34:56"/>
///   <source network="default"/>
///   <model type="virtio"/>
///   <target dev="vnet0"/>
///   <link state="up"/>
/// </interface>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, XmlBound)]
#[xml(tag = "interface")]
pub struct DeviceNetwork {
    #[xml(attribute = "type")]
    pub interface_type: Option<String>,

    #[xml(path = "mac", attribute = "address")]
    pub macaddr: Option<String>,

    /// For `type="network"`.
    #[xml(path = "source", attribute = "network")]
    pub source_network: Option<String>,

    /// For `type="bridge"`.
    #[xml(path = "source", attribute = "bridge")]
    pub source_bridge: Option<String>,

    #[xml(path = "model", attribute = "type")]
    pub model: Option<String>,

    #[xml(path = "target", attribute = "dev")]
    pub target_dev: Option<String>,

    #[xml(path = "link", attribute = "state")]
    pub link_state: Option<String>,
}
