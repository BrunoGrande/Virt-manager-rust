//! Runtime support for ticket 06's XPath-binding approach. See
//! `virtinst-xml-derive` for the `#[derive(XmlBound)]` macro that
//! generates code against the traits and helpers here.

pub use edit_xml::{Document, EditXMLError, Element, ReadOptions, Result};
pub use virtinst_xml_derive::XmlBound;

/// `Document::parse_str`, but relaxed to not require an XML declaration
/// — libvirt's own `virDomainGetXMLDesc`-style output never has one, so
/// requiring it (`edit_xml`'s default) would reject every real document
/// this crate is actually meant to read.
pub fn parse_libvirt_xml(xml: &str) -> Result<Document> {
    Document::parse_str_with_opts(xml, ReadOptions::relaxed())
}

/// Implemented by every `#[derive(XmlBound)]` struct: read/write itself
/// against one `edit_xml` element, touching only its own bound fields
/// and never anything else already in the document (ticket 03's
/// round-trip/preservation acceptance bar — see the crate-level test in
/// `virtinst-core` for a concrete check of this).
pub trait XmlBound: Sized {
    /// The XML tag name this struct binds to, e.g. `"disk"`.
    const TAG: &'static str;

    fn from_element(doc: &Document, el: Element) -> Self;
    fn write_to(&self, doc: &mut Document, el: Element);
}

/// How one field's value converts to/from an XML attribute string.
/// `to_attr() -> None` means "don't write this attribute at all" —
/// the mechanism `Option<T>` fields use to stay absent from the XML
/// rather than round-tripping as an empty string.
pub trait XmlAttrValue: Sized {
    fn from_attr(s: Option<&str>) -> Self;
    fn to_attr(&self) -> Option<String>;
}

impl XmlAttrValue for Option<String> {
    fn from_attr(s: Option<&str>) -> Self {
        s.map(str::to_string)
    }
    fn to_attr(&self) -> Option<String> {
        self.clone()
    }
}

impl XmlAttrValue for String {
    fn from_attr(s: Option<&str>) -> Self {
        s.unwrap_or_default().to_string()
    }
    fn to_attr(&self) -> Option<String> {
        Some(self.clone())
    }
}

/// libvirt XML's usual boolean spelling: `yes`/`no`, not `true`/`false`.
impl XmlAttrValue for bool {
    fn from_attr(s: Option<&str>) -> Self {
        s == Some("yes")
    }
    fn to_attr(&self) -> Option<String> {
        Some(if *self { "yes" } else { "no" }.to_string())
    }
}

/// Walks a `/`-separated chain of child-element names from `current`,
/// using `find()` at each step. An empty `segments` (a field bound
/// directly on its own element, no `path`) returns `current` unchanged.
/// A missing segment anywhere in the chain returns `None` — this is how
/// an unset optional nested field reads back as absent rather than
/// panicking.
pub fn resolve_path(doc: &Document, mut current: Element, segments: &[&str]) -> Option<Element> {
    for seg in segments {
        current = current.find(doc, seg)?;
    }
    Some(current)
}

/// Same walk as [`resolve_path`], but creates each missing segment as a
/// new child element instead of returning `None`. Only ever adds the
/// exact chain asked for — never touches sibling content, which is the
/// entire preservation guarantee this macro exists for.
pub fn resolve_or_create_path(
    doc: &mut Document,
    mut current: Element,
    segments: &[&str],
) -> Element {
    for seg in segments {
        current = match current.find(doc, seg) {
            Some(existing) => existing,
            None => {
                let created = Element::new(doc, *seg);
                current
                    .push_child(doc, created)
                    .expect("a freshly-created detached element always attaches cleanly");
                created
            }
        };
    }
    current
}
