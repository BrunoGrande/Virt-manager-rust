//! `<disk>` — upstream's `devices/disk.py`. First real exercise of
//! ticket 06's derive macro against a device with several distinct
//! nested-element attribute groups (`<driver>`, `<source>`, `<target>`),
//! not just flat attributes-on-self.
//!
//! Per-driver default resolution (`_default_bus` in upstream, ticket
//! 17's finding) isn't modeled yet — this struct is XML *binding* only
//! so far, not the default-resolution chain.

use virtinst_xml::XmlBound;

/// ```xml
/// <disk type="file" device="disk">
///   <driver name="qemu" type="qcow2"/>
///   <source file="/var/lib/libvirt/images/vm.qcow2"/>
///   <target dev="vda" bus="virtio"/>
/// </disk>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, XmlBound)]
#[xml(tag = "disk")]
pub struct DeviceDisk {
    #[xml(attribute = "type")]
    pub disk_type: Option<String>,

    #[xml(attribute = "device")]
    pub device: Option<String>,

    #[xml(path = "driver", attribute = "name")]
    pub driver_name: Option<String>,

    #[xml(path = "driver", attribute = "type")]
    pub driver_type: Option<String>,

    #[xml(path = "source", attribute = "file")]
    pub source_file: Option<String>,

    #[xml(path = "target", attribute = "dev")]
    pub target_dev: Option<String>,

    #[xml(path = "target", attribute = "bus")]
    pub target_bus: Option<String>,
}
