//! `<devices>` — the container wrapping every device element on a
//! domain. First real exercise of `#[xml(list)]`: `<disk>` can appear
//! any number of times, unlike every field modeled so far.
//!
//! `disks`/`interfaces`/`graphics`/`controllers`/`inputs`/`sounds` are
//! modeled so far, matching how many device types have their own struct
//! ([`super::DeviceDisk`], [`super::DeviceNetwork`],
//! [`super::DeviceGraphics`], [`super::DeviceController`],
//! [`super::DeviceInput`], [`super::DeviceSound`]) — the remaining
//! `NewDevice` variants (ticket 12) get their own `Vec<T>` field here as
//! each one is implemented, same pattern.
//!
//! Every `list` field is populated by
//! [`virtinst_xml::XmlBound::from_element`] but not written back through
//! `write_to` — adding/removing a device goes through
//! `virtinst_xml::list_add`/`list_remove` against a specific element
//! instead. See that module's docs for why: nothing in this project ever
//! needs "take a whole new list, diff it against the document" — ticket
//! 12's Add Hardware and ticket 10's Remove Device are both single-item
//! operations against a live DOM.

use super::{DeviceController, DeviceDisk, DeviceGraphics, DeviceInput, DeviceNetwork, DeviceSound};
use virtinst_xml::XmlBound;

#[derive(Debug, Clone, Default, PartialEq, Eq, XmlBound)]
#[xml(tag = "devices")]
pub struct DeviceList {
    #[xml(list)]
    pub disks: Vec<DeviceDisk>,

    #[xml(list)]
    pub interfaces: Vec<DeviceNetwork>,

    #[xml(list)]
    pub graphics: Vec<DeviceGraphics>,

    #[xml(list)]
    pub controllers: Vec<DeviceController>,

    #[xml(list)]
    pub inputs: Vec<DeviceInput>,

    #[xml(list)]
    pub sounds: Vec<DeviceSound>,
}
