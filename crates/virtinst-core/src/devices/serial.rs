//! `<serial>` — upstream's `devices/char.py`, specifically `DeviceSerial`
//! (`_DeviceChar` base class, `XML_NAME = "serial"`). Grounded in the
//! real `XMLProperty` declarations, read carefully rather than at a
//! glance: `_DeviceChar` itself only has `type` and `target/*` as
//! straightforward self/nested attributes; `host`/`service`/`path`/
//! `mode`/`tls` all live on a single nested `<source>` child
//! (`CharSource`, `is_single=True` — one `<source>`, not a list), also
//! a plain forward path.
//!
//! **Real gap, narrow and non-blocking:** `CharSource` also declares
//! `connect_host`/`bind_host`/`protocol`/`log_file` via `..` (parent
//! axis) and `[@mode='connect']` (attribute-predicate) XPath —
//! `./../source[@mode='connect']/@host`. Neither parent traversal nor
//! predicate filtering exists in this crate's `path` resolver (plain
//! forward `find()` by tag name only). Not modeled here — but these are
//! upstream's own *redundant convenience duplicates* for
//! cross-referencing a sibling `<source>` when more than one exists
//! (e.g. separate bind/connect sources for TCP); `source_mode`/
//! `source_host`/`source_service` below already reach the same data
//! for this struct's own single `<source>`, so nothing is actually
//! unreachable, just the shortcut spelling.
//!
//! Upstream shares this exact field set across four tags via
//! `_DeviceChar` (`DeviceConsole`="console", `DeviceSerial`="serial",
//! `DeviceParallel`="parallel", `DeviceChannel`="channel") — no Rust
//! inheritance to port it through (ticket 06 already rejected porting
//! upstream's dynamic class machinery), so each becomes its own
//! `#[xml(tag = "...")]` struct. `DeviceSerial` first as the
//! representative case; the other three are the same fields under a
//! different tag, added when needed.

use virtinst_xml::XmlBound;

/// ```xml
/// <serial type="pty">
///   <target type="isa-serial" port="0">
///     <model name="isa-serial"/>
///   </target>
/// </serial>
/// ```
/// or
/// ```xml
/// <serial type="tcp">
///   <source mode="bind" host="127.0.0.1" service="2445"/>
///   <target type="isa-serial" port="0"/>
/// </serial>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, XmlBound)]
#[xml(tag = "serial")]
pub struct DeviceSerial {
    #[xml(attribute = "type")]
    pub char_type: Option<String>,

    #[xml(path = "source", attribute = "mode")]
    pub source_mode: Option<String>,

    #[xml(path = "source", attribute = "host")]
    pub source_host: Option<String>,

    #[xml(path = "source", attribute = "service")]
    pub source_service: Option<u32>,

    #[xml(path = "source", attribute = "path")]
    pub source_path: Option<String>,

    #[xml(path = "source", attribute = "tls")]
    pub source_tls: Option<bool>,

    #[xml(path = "target", attribute = "type")]
    pub target_type: Option<String>,

    #[xml(path = "target", attribute = "port")]
    pub target_port: Option<u32>,

    #[xml(path = "target/model", attribute = "name")]
    pub target_model_name: Option<String>,
}
