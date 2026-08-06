//! `<devices>` — the container wrapping every device element on a
//! domain. First real exercise of `#[xml(list)]`: `<disk>` can appear
//! any number of times, unlike every field modeled so far.
//!
//! Every one of ticket 12's 17 `NewDevice` variants now has both a
//! typed struct ([`super::DeviceDisk`], [`super::DeviceNetwork`],
//! [`super::DeviceGraphics`], [`super::DeviceController`],
//! [`super::DeviceInput`], [`super::DeviceSound`],
//! [`super::DeviceFilesystem`], [`super::DeviceHostdev`],
//! [`super::DeviceSerial`], [`super::DeviceTpm`], [`super::DeviceRng`],
//! [`super::DevicePanic`], [`super::DeviceVsock`],
//! [`super::DeviceWatchdog`], [`super::DeviceSmartcard`],
//! [`super::DeviceRedirdev`], [`super::DeviceVideo`]) and a `list` field
//! here.
//!
//! Every `list` field is populated by
//! [`virtinst_xml::XmlBound::from_element`] but not written back through
//! `write_to` — adding/removing a device goes through
//! `virtinst_xml::list_add`/`list_remove` against a specific element
//! instead. See that module's docs for why: nothing in this project ever
//! needs "take a whole new list, diff it against the document" — ticket
//! 12's Add Hardware and ticket 10's Remove Device are both single-item
//! operations against a live DOM.

use super::{
    DeviceController, DeviceDisk, DeviceFilesystem, DeviceGraphics, DeviceHostdev, DeviceInput,
    DeviceNetwork, DevicePanic, DeviceRedirdev, DeviceRng, DeviceSerial, DeviceSmartcard,
    DeviceSound, DeviceTpm, DeviceVideo, DeviceVsock, DeviceWatchdog,
};
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

    #[xml(list)]
    pub filesystems: Vec<DeviceFilesystem>,

    #[xml(list)]
    pub hostdevs: Vec<DeviceHostdev>,

    #[xml(list)]
    pub serials: Vec<DeviceSerial>,

    #[xml(list)]
    pub tpms: Vec<DeviceTpm>,

    #[xml(list)]
    pub rngs: Vec<DeviceRng>,

    #[xml(list)]
    pub panics: Vec<DevicePanic>,

    #[xml(list)]
    pub vsocks: Vec<DeviceVsock>,

    #[xml(list)]
    pub watchdogs: Vec<DeviceWatchdog>,

    #[xml(list)]
    pub smartcards: Vec<DeviceSmartcard>,

    #[xml(list)]
    pub redirdevs: Vec<DeviceRedirdev>,

    #[xml(list)]
    pub videos: Vec<DeviceVideo>,
}
