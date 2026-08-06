//! `<controller>` — upstream's `devices/controller.py`. Grounded in the
//! real `XMLProperty` declarations first: most of this device is
//! `is_int=True` attributes (`index`, `vectors`, `ports`, plus several
//! nested under `driver`/`target`), the first real exercise of
//! `Option<u32>` binding.
//!
//! This is the type ticket 12 named directly — a new `DeviceDisk`
//! needing virtio-scsi implicitly wants a companion `DeviceController`,
//! folded into that variant's data rather than a bolted-on side-channel
//! (`DiskConfig { companion_controller: Option<ControllerConfig> }`).
//! That `NewDevice`-enum-level wiring is still future work; this struct
//! is what such a field would reference once it exists.
//!
//! `target/@hotplug` uses libvirt's `on`/`off` boolean spelling, not
//! `yes`/`no` — not modeled yet, see `virtinst_xml`'s `Option<u32>` doc
//! comment for why that's a flagged gap rather than a silent one.

use virtinst_xml::XmlBound;

/// ```xml
/// <controller type="scsi" index="0" model="virtio-scsi">
///   <driver queues="4"/>
///   <target chassis="0"/>
/// </controller>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, XmlBound)]
#[xml(tag = "controller")]
pub struct DeviceController {
    #[xml(attribute = "type")]
    pub controller_type: Option<String>,

    #[xml(attribute = "model")]
    pub model: Option<String>,

    #[xml(attribute = "index")]
    pub index: Option<u32>,

    #[xml(path = "driver", attribute = "queues")]
    pub driver_queues: Option<u32>,

    #[xml(path = "target", attribute = "chassis")]
    pub target_chassis: Option<u32>,
}
