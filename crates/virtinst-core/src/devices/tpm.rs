//! `<tpm>` — upstream's `devices/tpm.py`. Grounded in the real
//! `XMLProperty`/`XMLChildProperty` declarations: `active_pcr_banks` is
//! `XMLChildProperty(_ActivePCRBanks, is_single=True, relative_xpath="./backend")`,
//! so `sha1`/`sha256`/etc. (each `is_bool=True`, upstream's own nested
//! helper class) sit three levels deep —
//! `backend/active_pcr_banks/sha1` — the first `#[xml(present)]` field
//! with a multi-segment path.

use virtinst_xml::XmlBound;

/// ```xml
/// <tpm model="tpm-crb">
///   <backend type="emulator" version="2.0" persistent_state="yes">
///     <active_pcr_banks>
///       <sha256/>
///     </active_pcr_banks>
///   </backend>
/// </tpm>
/// ```
/// or
/// ```xml
/// <tpm model="tpm-tis">
///   <backend type="passthrough">
///     <device path="/dev/tpm0"/>
///   </backend>
/// </tpm>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, XmlBound)]
#[xml(tag = "tpm")]
pub struct DeviceTpm {
    #[xml(attribute = "model")]
    pub model: Option<String>,

    #[xml(path = "backend", attribute = "type")]
    pub backend_type: Option<String>,

    #[xml(path = "backend", attribute = "version")]
    pub version: Option<String>,

    #[xml(path = "backend", attribute = "persistent_state")]
    pub persistent_state: Option<bool>,

    #[xml(path = "backend/device", attribute = "path")]
    pub device_path: Option<String>,

    #[xml(path = "backend/active_pcr_banks/sha1", present)]
    pub sha1: bool,

    #[xml(path = "backend/active_pcr_banks/sha256", present)]
    pub sha256: bool,
}
