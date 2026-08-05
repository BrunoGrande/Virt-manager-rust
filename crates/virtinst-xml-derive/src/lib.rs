//! `#[derive(XmlBound)]` — generates an `impl virtinst_xml::XmlBound`
//! that reads/writes struct fields against an `edit_xml` element
//! (ticket 06's "typed structs bound to XPaths via a derive macro,
//! operating over a mutable order-preserving DOM" decision).
//!
//! Two field kinds:
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
//!
//! #[derive(XmlBound)]
//! #[xml(tag = "devices")]
//! struct DeviceList {
//!     #[xml(list)]
//!     disks: Vec<DeviceDisk>,
//! }
//! ```
//!
//! `path` is a `/`-separated chain of child-element names, resolved with
//! `find()` on read (missing segment ⇒ the field reads as absent/empty)
//! and created with `Element::new` + `push_child` on write if missing —
//! never touching anything else already in the document, which is the
//! entire point (ticket 03's round-trip/preservation acceptance bar).
//! `list` fields can take a `path` too, for a container one level down
//! (e.g. `#[xml(path = "devices", list)]` if the `Vec` field lives
//! directly on `Guest` rather than a `DeviceList` wrapper struct).
//!
//! `list` fields are **read-only** through this trait: `from_element`
//! collects every `T::TAG` child into the `Vec`, but `write_to`
//! generates no code for them at all. Mutating a list goes through
//! `virtinst_xml::list_add`/`list_remove` against specific elements
//! instead — see that module's docs for why a whole-Vec reconcile-on-
//! write was deliberately not built.
//!
//! Scope of this slice: `attribute` (optionally under `path`) and
//! `list`. `#[xml(text)]` for text-content fields is designed for (see
//! ticket 06's issue file) but not implemented yet — an unrecognized
//! field attribute is a compile error, not a silent no-op, so that gap
//! is loud rather than a footgun.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, GenericArgument, LitStr, PathArguments, Type};

#[proc_macro_derive(XmlBound, attributes(xml))]
pub fn derive_xml_bound(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

enum FieldKind {
    Attribute { attribute: String },
    List { item_ty: Type },
}

struct FieldBinding {
    ident: syn::Ident,
    path: Vec<String>,
    kind: FieldKind,
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
        match &b.kind {
            FieldKind::Attribute { attribute } => quote! {
                #ident: ::virtinst_xml::XmlAttrValue::from_attr(
                    ::virtinst_xml::resolve_path(__doc, __el, &[#(#segs),*])
                        .and_then(|__target| __target.attribute(__doc, #attribute))
                )
            },
            FieldKind::List { item_ty } => quote! {
                #ident: ::virtinst_xml::list_read::<#item_ty>(__doc, __el, &[#(#segs),*])
            },
        }
    });

    let write_fields = bindings.iter().filter_map(|b| {
        let ident = &b.ident;
        let segs = &b.path;
        match &b.kind {
            FieldKind::Attribute { attribute } => Some(quote! {
                if let Some(__value) = ::virtinst_xml::XmlAttrValue::to_attr(&self.#ident) {
                    let __target = ::virtinst_xml::resolve_or_create_path(__doc, __el, &[#(#segs),*]);
                    __target.set_attribute(__doc, #attribute, __value);
                }
            }),
            // list fields are read-only through XmlBound — see the
            // crate-level docs for why. No code generated here at all.
            FieldKind::List { .. } => None,
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

/// Pulls `#[xml(attribute = "...")]` or `#[xml(list)]` — each optionally
/// with `path = "..."` — off one field.
fn field_binding(field: &syn::Field) -> syn::Result<FieldBinding> {
    let ident = field
        .ident
        .clone()
        .ok_or_else(|| syn::Error::new_spanned(field, "XmlBound requires named fields"))?;

    let mut path = String::new();
    let mut attribute = None;
    let mut is_list = false;

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
            } else if meta.path.is_ident("list") {
                is_list = true;
                Ok(())
            } else if meta.path.is_ident("text") {
                Err(meta.error(
                    "#[xml(text)] is designed but not implemented yet — see \
                     virtinst-xml-derive's crate docs",
                ))
            } else {
                Err(meta.error(
                    "expected `attribute = \"...\"`, `list`, and/or `path = \"...\"`",
                ))
            }
        })?;
    }

    let path = if path.is_empty() {
        Vec::new()
    } else {
        path.split('/').map(str::to_string).collect()
    };

    let kind = match (is_list, attribute) {
        (true, Some(_)) => {
            return Err(syn::Error::new_spanned(
                &ident,
                "a field can't be both `list` and `attribute` — pick one",
            ));
        }
        (true, None) => {
            let item_ty = vec_item_type(&field.ty).ok_or_else(|| {
                syn::Error::new_spanned(
                    &field.ty,
                    "#[xml(list)] requires a Vec<T> field, where T: XmlBound",
                )
            })?;
            FieldKind::List { item_ty }
        }
        (false, Some(attribute)) => FieldKind::Attribute { attribute },
        (false, None) => {
            return Err(syn::Error::new_spanned(
                &ident,
                "field needs an #[xml(attribute = \"...\")] or #[xml(list)] binding \
                 (optionally with `path = \"...\"`)",
            ));
        }
    };

    Ok(FieldBinding { ident, path, kind })
}

/// Syntactic `Vec<T>` detection — a proc-macro sees only the written
/// type, not its resolved identity, so this is a best-effort check on
/// the last path segment (the standard approach; the same one serde and
/// most other field-attribute derive macros use for this exact case).
fn vec_item_type(ty: &Type) -> Option<Type> {
    let Type::Path(type_path) = ty else {
        return None;
    };
    let segment = type_path.path.segments.last()?;
    if segment.ident != "Vec" {
        return None;
    }
    let PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };
    args.args.iter().find_map(|arg| match arg {
        GenericArgument::Type(inner) => Some(inner.clone()),
        _ => None,
    })
}
