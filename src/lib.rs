#![allow(unknown_lints, clippy::useless_format)]
#![deny(
    missing_docs,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    rustdoc::missing_doc_code_examples,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links,
    rustdoc::missing_crate_level_docs,
    rustdoc::invalid_codeblock_attributes,
    rustdoc::bare_urls
)]

//! TODO: doc

mod error;
use error::*;

use convert_case::{Case, Casing};
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Ident;

mod sections {
    use super::DataContainer;
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::Ident;

    pub mod additions;
    pub mod bool_ops;
    pub mod entry_ops;
    pub mod get_contained;
    pub mod iters;
    pub mod misc;
    pub mod querying;
    pub mod ref_adapters;
    pub mod transformers;
}

mod external {
    use super::DataContainer;
    use proc_macro2::TokenStream;
    use quote::quote;

    pub mod impls;
    pub mod traits;
}

pub(crate) struct DataContainer {
    name: Ident,
    full_name: TokenStream,
    full_name_string: String,
    some_ident: Ident,
    none_ident: Ident,
    some_snake: String,
    none_snake: String,
    none_pattern: TokenStream,
    some_ty: syn::Type,
    some_field_ident: Option<Ident>,
    some_ty_name: String,
    is_generic: bool,
    imp: TokenStream,
    wheres: TokenStream,
    #[allow(dead_code)]
    where_clause: TokenStream,

    some_x: TokenStream,
    some_ref_x: TokenStream,
    some_ref_mut_x: TokenStream,
    some__: TokenStream,
    some_y: TokenStream,
    some_xy: TokenStream,

    func: TokenStream,
    c_func: TokenStream,
    opt: TokenStream,
}

impl DataContainer {
    fn some(&self, pattern: TokenStream) -> TokenStream {
        let name = &self.name;
        let some_ident = &self.some_ident;
        if let Some(ident) = self.some_field_ident.as_ref() {
            quote! {#name::#some_ident { #ident: #pattern }}
        } else {
            quote! {#name::#some_ident(#pattern)}
        }
    }
}

/// TODO: doc
/// TODO: talk about `T` as placeholder for the contained type
///
/// ## Added Methods
///
/// Symbols:
/// - (G): only added if the enum is generic over the contained type
/// - (U): not yet added because it's unstable
///
/// #### Querying the contained values
/// - `is_<some>` (where `<some>` is the snake_case version of the `Some`-like variant)
/// - ~~`is_<some>_and`~~ (U)
/// - `is_<none>`
///
/// #### Adapter for working with references
/// - `as_ref` (G)
/// - `as_mut` (G)
/// - `as_pin_ref` (G)
/// - `as_pin_mut` (G)
///
/// #### Getting to contained values
/// - `expect`
/// - `unwrap`
/// - `unwrap_or`
/// - `unwrap_or_else`
/// - `unwrap_or_default`
/// - `unwrap_unchecked`
///
/// #### Transforming contained values
/// - `map` (G)
/// - ~~`inspect`~~ (U)
/// - `map_or` (G)
/// - `map_or_else` (G)
/// - `ok_or`
/// - `ok_or_else`
/// - `as_deref` (G)
/// - `as_deref_mut` (G)
///
/// #### Iterator constructors
/// - `iter`
/// - `iter_mut`
///
/// #### Boolean operations on the values, eager and lazy
/// - `and`
/// - `and_then`
/// - `filter`
/// - `or`
/// - `or_else`
/// - `xor`
///
/// #### Entry-like operations to insert a value and return a reference
/// - `insert`
/// - `get_or_insert`
/// - `get_or_insert_default`
/// - `get_or_insert_with`
///
/// #### Misc
/// - `take`
/// - `replace`
/// - `contains`
/// - `zip` (G)
/// - `zip_with`
///
/// ## Additional Methods not in `Option`
/// - `as_option_ref`: Converts `&Self` to `Option<&inner>`, similar to `as_ref`
///   but swapping `Self` with `Option`
/// - `as_option_mut`: Converts `&mut Self` to `Option<&mut inner>`, similar to `as_mut`
///   but swapping `Self` with `Option`
///
/// ## Traits
/// - `From<T> for Self`
/// - `From<Option<T>> for Self`
/// - `From<Self> for Option<T>`
/// - `Self: Default`
///
/// ## Things that were **not** added
/// - unstable or nightly-only methods and traits
///   - this sadly includes the try (`?`) operator
#[proc_macro_derive(Optional)]
pub fn optional(input: TokenStream1) -> TokenStream1 {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    match optional_internal(input) {
        Ok(s) => s.into(),
        Err(e) => e.into(),
    }
}

fn optional_internal(input: syn::DeriveInput) -> Result<TokenStream> {
    let name = input.ident;

    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let full_name = quote! {#name #ty_generics};
    let full_name_string = full_name.to_string().replace(' ', "");
    let imp = quote! {impl #impl_generics};
    let wheres = where_clause
        .map(|c| c.to_token_stream())
        .unwrap_or_else(TokenStream::new);
    let where_clause = match where_clause {
        Some(c) => {
            let mut s = c.to_token_stream();
            // check if there is a trailing `,` and add one if not
            if !s.is_empty() && !s.to_string().trim().ends_with(',') {
                s.extend(quote! {,});
            }
            s
        }
        None => quote! {where },
    };

    let data = match input.data {
        syn::Data::Enum(data) => Ok(data),
        syn::Data::Struct(data) => Err(data.struct_token.span),
        syn::Data::Union(data) => Err(data.union_token.span),
    }
    .map_err(|span| Error::new(span, "Optional can only be used on enums"))?;

    let variants = data.variants;
    if variants.len() != 2 {
        let msg = "Optional only works when there are exactly 2 enum variants";
        return Error::err_spanned(variants, msg);
    }

    let (some_variant, none_variant) = {
        let mut iter = variants.into_iter();
        let a = iter.next().unwrap(); // unwrap ok because we checked len == 2
        let b = iter.next().unwrap();

        let a_data = !matches!(a.fields, syn::Fields::Unit);
        let b_data = !matches!(b.fields, syn::Fields::Unit);
        match (a_data, b_data) {
            (false, false) => {
                let msg = "Optional needs exactly one variant with data (the `Some(T)` equivalent)";
                return Error::builder()
                    .with_spanned(a, msg)
                    .with_spanned(b, msg)
                    .build_err();
            }
            (true, true) => {
                let msg = "Optional needs exactly one unit variant (the `None` equivalent)";
                return Error::builder()
                    .with_spanned(a, msg)
                    .with_spanned(b, msg)
                    .build_err();
            }
            (true, false) => (a, b),
            (false, true) => (b, a),
        }
    };

    if some_variant.fields.len() != 1 {
        let msg = "Optional currently only supports one type in the variant";
        return Error::err_spanned(some_variant.fields, msg);
    }

    let some_ident = some_variant.ident;
    let none_ident = none_variant.ident;
    let none_pattern = quote! {#name::#none_ident};

    let some_snake = some_ident.to_string().to_case(Case::Snake);
    let none_snake = none_ident.to_string().to_case(Case::Snake);

    let some_field = some_variant.fields.into_iter().next().unwrap();
    let some_ty = some_field.ty;
    let some_field_ident = some_field.ident;
    let some_ty_name = some_ty.to_token_stream().to_string();

    let is_generic = generics.params.iter().any(|param| {
        if let syn::GenericParam::Type(ty) = param {
            ty.ident == some_ty_name
        } else {
            false
        }
    });

    let make_some = |pattern: TokenStream| {
        if let Some(ident) = some_field_ident.as_ref() {
            quote! {#name::#some_ident { #ident: #pattern }}
        } else {
            quote! {#name::#some_ident(#pattern)}
        }
    };
    let some_x = make_some(quote! {x});
    let some_ref_x = make_some(quote! {ref x});
    let some_ref_mut_x = make_some(quote! {ref mut x});
    let some__ = make_some(quote! {..});
    let some_y = make_some(quote! {y});
    let some_xy = make_some(quote! {(x, y)});

    let opt = quote! {::std::option::Option};
    let func = quote! {#[inline] pub fn};
    let c_func = quote! {#[inline] pub const fn};

    let container = DataContainer {
        name,
        full_name,
        full_name_string,
        some_ident,
        none_ident,
        some_snake,
        none_snake,
        none_pattern,
        some_ty,
        some_field_ident,
        some_ty_name,
        is_generic,
        imp,
        wheres,
        where_clause,

        some_x,
        some_ref_x,
        some_ref_mut_x,
        some__,
        some_y,
        some_xy,

        func,
        c_func,
        opt,
    };

    let mut impl_block = TokenStream::new();
    let mut additional_impls = TokenStream::new();

    sections::additions::add_section(&container, &mut impl_block);
    sections::bool_ops::add_section(&container, &mut impl_block);
    sections::entry_ops::add_section(&container, &mut impl_block);
    sections::get_contained::add_section(&container, &mut impl_block);
    sections::iters::add_section(&container, &mut impl_block);
    sections::misc::add_section(&container, &mut impl_block);
    sections::querying::add_section(&container, &mut impl_block);
    sections::ref_adapters::add_section(&container, &mut impl_block);
    sections::transformers::add_section(&container, &mut impl_block);

    external::traits::add_external(&container, &mut additional_impls);
    external::impls::add_external(&container, &mut additional_impls);

    let DataContainer {
        full_name,
        imp,
        wheres,
        ..
    } = container;

    Ok(quote! {
        #imp #full_name #wheres {
            #impl_block
        }

        #additional_impls
    })
}
