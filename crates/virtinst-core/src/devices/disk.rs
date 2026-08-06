//! `<disk>` — upstream's `devices/disk.py`. First real exercise of
//! ticket 06's derive macro against a device with several distinct
//! nested-element attribute groups (`<driver>`, `<source>`, `<target>`),
//! not just flat attributes-on-self.
//!
//! `read_only`/`shareable`/`transient` were left out when this struct
//! was first written — `<disk><readonly/></disk>` binds a child
//! element's *presence*, not an attribute or text value, and that field
//! kind (`#[xml(present)]`) didn't exist in the macro yet. Added now
//! that `DeviceFilesystem` needed the same pattern and it got built
//! properly. Confirmed against `disk.py`: all three really are
//! `is_bool=True`, same shape.
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
///   <readonly/>
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

    #[xml(path = "readonly", present)]
    pub read_only: bool,

    #[xml(path = "shareable", present)]
    pub shareable: bool,

    #[xml(path = "transient", present)]
    pub transient: bool,
}
