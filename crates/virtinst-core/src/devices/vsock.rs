//! `<vsock>` — upstream's `devices/vsock.py`. Grounded in the real
//! `XMLProperty` declarations. Clean, no gaps this time: three fields,
//! all straightforward forward paths.

use virtinst_xml::XmlBound;

/// ```xml
/// <vsock model="virtio">
///   <cid auto="no" address="3"/>
/// </vsock>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, XmlBound)]
#[xml(tag = "vsock")]
pub struct DeviceVsock {
    #[xml(attribute = "model")]
    pub model: Option<String>,

    #[xml(path = "cid", attribute = "auto")]
    pub auto_cid: Option<bool>,

    #[xml(path = "cid", attribute = "address")]
    pub cid: Option<u32>,
}
