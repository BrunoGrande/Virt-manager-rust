//! `<filesystem>` — upstream's `devices/filesystem.py`. The struct that
//! prompted adding `#[xml(present)]`: `readonly = XMLProperty("./readonly", is_bool=True)`
//! is a child element's presence, same shape as `DeviceDisk`'s
//! `read_only`/`shareable`/`transient` (now fixed to use it too).

use virtinst_xml::XmlBound;

/// ```xml
/// <filesystem type="mount" accessmode="mapped">
///   <source dir="/host/share"/>
///   <target dir="/mnt/share"/>
///   <readonly/>
/// </filesystem>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, XmlBound)]
#[xml(tag = "filesystem")]
pub struct DeviceFilesystem {
    #[xml(attribute = "type")]
    pub filesystem_type: Option<String>,

    #[xml(attribute = "accessmode")]
    pub accessmode: Option<String>,

    #[xml(path = "source", attribute = "dir")]
    pub source_dir: Option<String>,

    #[xml(path = "target", attribute = "dir")]
    pub target_dir: Option<String>,

    #[xml(path = "readonly", present)]
    pub readonly: bool,
}
