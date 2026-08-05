//! The root domain-XML aggregate type — upstream's `guest.py`.
//!
//! Aggregates [`crate::devices`] and [`crate::domain`], bound to their
//! XPaths via the derive macro decided in ticket 06. Two distinct
//! construction paths, not one:
//!
//! - **Fresh creation** (Create VM wizard, ticket 08; `virt-install`,
//!   ticket 16): build the struct directly and serialize fresh. No
//!   `edit-xml` DOM involved — there's no existing document to preserve.
//! - **Editing an existing domain** (VM details window, ticket 10;
//!   `virt-xml --edit`, ticket 16 — the first ticket to walk this path
//!   end to end): parse through the order-preserving `edit-xml` DOM,
//!   mutate via the typed struct's XPath binding, serialize back. This
//!   is the path that has to satisfy ticket 03's acceptance bar —
//!   byte-exact preservation of untouched XML, including foreign
//!   `xmlns:qemu` elements.
//!
//! `description`/`title` are the first real fields on this struct — a
//! `path` + `text` binding (`<domain><description>…</description>` is a
//! child element's text content, not an attribute on `<domain>` itself)
//! against the most natural real home for it. The rest — devices,
//! per-domain-subsystem structs, `<devices>`/`<name>`/etc. — follows the
//! same pattern as each of those gets implemented.

use virtinst_xml::XmlBound;

#[derive(Debug, Clone, Default, PartialEq, Eq, XmlBound)]
#[xml(tag = "domain")]
pub struct Guest {
    #[xml(path = "description", text)]
    pub description: Option<String>,

    #[xml(path = "title", text)]
    pub title: Option<String>,
    // TODO: name (text), uuid (text), devices: DeviceList, and the
    // domain-subsystem structs (Clock, CurrentMemory, ...) as each
    // gets its own struct.
}
