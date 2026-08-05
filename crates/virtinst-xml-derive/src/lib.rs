//! `#[derive(XmlBound)]` — generates an `impl virtinst_xml::XmlBound`
//! that reads/writes struct fields as attributes on an `edit_xml`
//! element (ticket 06's "typed structs bound to XPaths via a derive
//! macro, operating over a mutable order-preserving DOM" decision).
//!
//! ```ignore
//! #[derive(XmlBound)]
//! #[xml(tag = "disk")]
//! struct DeviceDisk {
//!     #[xml(attribute = "type")]
//!     disk_type: Option<String>,
//!
//!     #[xml(path = "source", attribute = "file")]
//!     source_file: Option<String>,
//! }
//! ```
//!
//! `path` is a `/`-separated chain of child-element names, resolved with
//! `find()` on read (missing segment ⇒ the field reads as absent) and
//! created with `Element::new` + `push_child` on write if missing —
//! never touching anything else already in the document, which is the
//! entire point (ticket 03's round-trip/preservation acceptance bar).
//!
//! Scope of this first slice: `attribute` (optionally under `path`)
//! only. `#[xml(text)]` for text-content fields is designed for (see
//! ticket 06's issue file) but not implemented yet — an unrecognized
//! field attribute is a compile error, not a silent no-op, so that gap
//! is loud rather than a footgun.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, LitStr};

#[proc_macro_derive(XmlBound, attributes(xml))]
pub fn derive_xml_bound(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

struct FieldBinding {
    ident: syn::Ident,
    path: Vec<String>,
    attribute: String,
}

fn expand(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name = &input.ident;
    let tag = struct_tag(&input)?;

    let Data::Struct(data) = &input.data else {
        return Err(syn::Error::new_spanned(
            &input,
            "XmlBound can only be derived for structs",
        ));
    };
    let Fields::Named(fields) = &data.fields else {
        return Err(syn::Error::new_spanned(
            &input,
            "XmlBound requires named fields",
        ));
    };

    let bindings = fields
        .named
        .iter()
        .map(field_binding)
        .collect::<syn::Result<Vec<_>>>()?;

    let read_fields = bindings.iter().map(|b| {
        let ident = &b.ident;
        let segs = &b.path;
        let attr = &b.attribute;
        quote! {
            #ident: ::virtinst_xml::XmlAttrValue::from_attr(
                ::virtinst_xml::resolve_path(__doc, __el, &[#(#segs),*])
                    .and_then(|__target| __target.attribute(__doc, #attr))
            )
        }
    });

    let write_fields = bindings.iter().map(|b| {
        let ident = &b.ident;
        let segs = &b.path;
        let attr = &b.attribute;
        quote! {
            if let Some(__value) = ::virtinst_xml::XmlAttrValue::to_attr(&self.#ident) {
                let __target = ::virtinst_xml::resolve_or_create_path(__doc, __el, &[#(#segs),*]);
                __target.set_attribute(__doc, #attr, __value);
            }
        }
    });

    Ok(quote! {
        impl ::virtinst_xml::XmlBound for #struct_name {
            const TAG: &'static str = #tag;

            fn from_element(__doc: &::virtinst_xml::Document, __el: ::virtinst_xml::Element) -> Self {
                Self {
                    #(#read_fields),*
                }
            }

            fn write_to(&self, __doc: &mut ::virtinst_xml::Document, __el: ::virtinst_xml::Element) {
                #(#write_fields)*
            }
        }
    })
}

/// Pulls `#[xml(tag = "...")]` off the struct itself.
fn struct_tag(input: &DeriveInput) -> syn::Result<String> {
    for attr in &input.attrs {
        if !attr.path().is_ident("xml") {
            continue;
        }
        let mut tag = None;
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("tag") {
                let value: LitStr = meta.value()?.parse()?;
                tag = Some(value.value());
                Ok(())
            } else {
                Err(meta.error("expected `tag = \"...\"`"))
            }
        })?;
        if let Some(tag) = tag {
            return Ok(tag);
        }
    }
    Err(syn::Error::new_spanned(
        input,
        "XmlBound requires a struct-level #[xml(tag = \"...\")] attribute",
    ))
}

/// Pulls `#[xml(attribute = "...")]` / `#[xml(path = "...", attribute = "...")]`
/// off one field.
fn field_binding(field: &syn::Field) -> syn::Result<FieldBinding> {
    let ident = field
        .ident
        .clone()
        .ok_or_else(|| syn::Error::new_spanned(field, "XmlBound requires named fields"))?;

    let mut path = String::new();
    let mut attribute = None;

    for attr in &field.attrs {
        if !attr.path().is_ident("xml") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("attribute") {
                let value: LitStr = meta.value()?.parse()?;
                attribute = Some(value.value());
                Ok(())
            } else if meta.path.is_ident("path") {
                let value: LitStr = meta.value()?.parse()?;
                path = value.value();
                Ok(())
            } else if meta.path.is_ident("text") {
                Err(meta.error(
                    "#[xml(text)] is designed but not implemented yet — see \
                     virtinst-xml-derive's crate docs",
                ))
            } else {
                Err(meta.error("expected `attribute = \"...\"` and/or `path = \"...\"`"))
            }
        })?;
    }

    let attribute = attribute.ok_or_else(|| {
        syn::Error::new_spanned(
            &ident,
            "field needs an #[xml(attribute = \"...\")] binding (optionally with `path = \"...\"`)",
        )
    })?;

    let path = if path.is_empty() {
        Vec::new()
    } else {
        path.split('/').map(str::to_string).collect()
    };

    Ok(FieldBinding {
        ident,
        path,
        attribute,
    })
}
