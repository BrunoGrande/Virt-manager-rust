//! `<input>` — upstream's `devices/input.py`. Small device, grounded in
//! the real `XMLProperty` declarations same as every other struct.
//! `source/@repeat` uses the `on`/`off` boolean spelling (`is_onoff`) —
//! left out for now, same flagged gap as `DeviceController`'s
//! `target/@hotplug`.

use virtinst_xml::XmlBound;

/// ```xml
/// <input type="tablet" bus="usb"/>
/// ```
/// or
/// ```xml
/// <input type="evdev" bus="virtio">
///   <source dev="/dev/input/event1" grab="all"/>
/// </input>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, XmlBound)]
#[xml(tag = "input")]
pub struct DeviceInput {
    #[xml(attribute = "type")]
    pub input_type: Option<String>,

    #[xml(attribute = "bus")]
    pub bus: Option<String>,

    #[xml(attribute = "model")]
    pub model: Option<String>,

    #[xml(path = "source", attribute = "evdev")]
    pub source_evdev: Option<String>,

    #[xml(path = "source", attribute = "dev")]
    pub source_dev: Option<String>,

    #[xml(path = "source", attribute = "grab")]
    pub source_grab: Option<String>,
}
