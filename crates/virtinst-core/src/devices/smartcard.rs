//! `<smartcard>` — upstream's `devices/smartcard.py`. Grounded in the
//! real `XMLProperty`/`XMLChildProperty` declarations.
//!
//! `source = XMLChildProperty(CharSource, is_single=True)` reuses the
//! same nested-`<source>` shape already modeled for `DeviceSerial`
//! (`char.py`) — same field subset here. `database = XMLProperty("./database")`
//! (no `@attr`) is `<database>`'s own text content, same
//! `path = "...", text` shape `DeviceRng` already proved.
//!
//! `certificates = XMLChildProperty(_Certificate)` — critically, **no**
//! `is_single=True` — is the first `#[xml(list)]` field living *inside*
//! a device struct rather than at [`super::DeviceList`]'s top level.
//! Each `<certificate>` is a plain text-only element
//! (`value = XMLProperty("./.")`), the first struct whose only field is
//! its own `#[xml(text)]` (path is empty — text content of the element
//! `from_element`/`write_to` are already given, same as `CurrentMemory`'s
//! `value` field, just without a sibling attribute this time).

use virtinst_xml::XmlBound;

/// `<certificate>base64-encoded-cert-data</certificate>`
#[derive(Debug, Clone, Default, PartialEq, Eq, XmlBound)]
#[xml(tag = "certificate")]
pub struct Certificate {
    #[xml(text)]
    pub value: Option<String>,
}

/// ```xml
/// <smartcard mode="host-certificates">
///   <certificate>cert1base64</certificate>
///   <certificate>cert2base64</certificate>
///   <database>/etc/pki/nssdb</database>
/// </smartcard>
/// ```
/// or
/// ```xml
/// <smartcard mode="passthrough" type="tcp">
///   <source mode="bind" host="127.0.0.1" service="2001"/>
/// </smartcard>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, XmlBound)]
#[xml(tag = "smartcard")]
pub struct DeviceSmartcard {
    #[xml(attribute = "mode")]
    pub mode: Option<String>,

    #[xml(attribute = "type")]
    pub smartcard_type: Option<String>,

    #[xml(path = "source", attribute = "mode")]
    pub source_mode: Option<String>,

    #[xml(path = "source", attribute = "host")]
    pub source_host: Option<String>,

    #[xml(path = "source", attribute = "service")]
    pub source_service: Option<u32>,

    #[xml(path = "database", text)]
    pub database: Option<String>,

    #[xml(list)]
    pub certificates: Vec<Certificate>,
}
