//! `<hostdev>` — upstream's `devices/hostdev.py`. A union-of-shapes
//! device (USB vendor/product-ID-based, PCI domain/bus/slot/function-
//! based, SCSI adapter-based, …) — grounded in the real `XMLProperty`
//! declarations, first real exercise of a genuine multi-segment `path`
//! (`vendor = XMLProperty("./source/vendor/@id")` → `path = "source/vendor"`)
//! and of `text` under a multi-segment path
//! (`net_interface = XMLProperty("./source/interface")`, no `@attr` —
//! it's the element's own text content).
//!
//! Modeled: mode/type/managed (self), USB vendor/product IDs, PCI
//! address fields, and the network-interface text case. Not modeled
//! yet: SCSI adapter fields, `rom_bar` (`is_onoff`, same flagged gap as
//! other devices), `uuid`, `misc_char`/`storage_block` (same
//! text-under-path shape as `net_interface`, omitted for scope not for
//! a design reason).

use virtinst_xml::XmlBound;

/// USB, by vendor/product ID:
/// ```xml
/// <hostdev mode="subsystem" type="usb" managed="yes">
///   <source>
///     <vendor id="0x1234"/>
///     <product id="0x5678"/>
///   </source>
/// </hostdev>
/// ```
/// PCI, by address:
/// ```xml
/// <hostdev mode="subsystem" type="pci" managed="yes">
///   <source>
///     <address domain="0x0000" bus="0x00" slot="0x02" function="0x0"/>
///   </source>
/// </hostdev>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, XmlBound)]
#[xml(tag = "hostdev")]
pub struct DeviceHostdev {
    #[xml(attribute = "mode")]
    pub mode: Option<String>,

    #[xml(attribute = "type")]
    pub hostdev_type: Option<String>,

    #[xml(attribute = "managed")]
    pub managed: Option<bool>,

    #[xml(path = "source/vendor", attribute = "id")]
    pub vendor: Option<String>,

    #[xml(path = "source/product", attribute = "id")]
    pub product: Option<String>,

    #[xml(path = "source/address", attribute = "domain")]
    pub pci_domain: Option<String>,

    #[xml(path = "source/address", attribute = "bus")]
    pub pci_bus: Option<String>,

    #[xml(path = "source/address", attribute = "slot")]
    pub pci_slot: Option<String>,

    #[xml(path = "source/address", attribute = "function")]
    pub pci_function: Option<String>,

    #[xml(path = "source/interface", text)]
    pub net_interface: Option<String>,
}
