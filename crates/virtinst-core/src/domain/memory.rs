//! `<currentMemory>` — upstream's `domain/memory.py`. First real
//! exercise of `#[xml(text)]`, and of `attribute` and `text` binding to
//! two different fields on the *same* element (no `path` needed for
//! either — libvirt puts the unit as an attribute and the value as the
//! element's own text content).

use virtinst_xml::XmlBound;

/// ```xml
/// <currentMemory unit="KiB">1048576</currentMemory>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, XmlBound)]
#[xml(tag = "currentMemory")]
pub struct CurrentMemory {
    #[xml(attribute = "unit")]
    pub unit: Option<String>,

    #[xml(text)]
    pub value: Option<String>,
}
