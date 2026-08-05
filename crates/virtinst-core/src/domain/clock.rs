//! `<clock>` — upstream's `domain/clock.py`. Simplest possible exercise
//! of the derive macro (a single flat attribute, no nested elements) —
//! one of ticket 17's 9 files that branches on driver family (Xen's
//! default `offset` differs from QEMU's); that default-resolution logic
//! isn't modeled here yet, this is XML binding only.

use virtinst_xml::XmlBound;

/// ```xml
/// <clock offset="utc"/>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, XmlBound)]
#[xml(tag = "clock")]
pub struct Clock {
    #[xml(attribute = "offset")]
    pub offset: Option<String>,
}
