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

mod sections {
    use super::DataContainer;
    use proc_macro2::TokenStream;
    use quote::quote;

    pub mod s01_querying;
    pub mod s02_ref_adapters;
    pub mod s03_get_contained;
    pub mod s04_transformers;
    pub mod s05_bool_ops;
    pub mod s05_iters;
    pub mod s06_entry_ops;
    pub mod s07_misc;
    pub mod s99_additions;
}

mod external {
    use super::DataContainer;
    use proc_macro2::TokenStream;
    use quote::quote;

    pub mod impls;
    pub mod traits;
}

type Bounds = syn::punctuated::Punctuated<syn::TypeParamBound, syn::token::Plus>;

pub(crate) struct DataContainer {
    /// The name of the enum
    name: syn::Ident,
    /// The full name of the enum, including generics if present
    full_name: TokenStream,
    /// String representation of full_name
    full_name_string: String,
    /// #name :: #some_variant
    some: TokenStream,
    /// #name :: #none_variant
    none: TokenStream,
    /// Name of #some_variant
    some_name: String,
    /// Name of #none_variant
    none_name: String,
    /// Name of #some_variant in snake_case
    some_name_snake: String,
    /// Name of #none_variant in snake_case
    none_name_snake: String,
    /// Type inside #some_variant
    some_ty: syn::Type,
    /// Name of #some_ty
    some_ty_name: String,
    /// Whether the enum is generic
    is_generic: bool,
    /// The traits on the generic type. Empty if not generic
    bounds: Option<Bounds>,
    /// impl #ty_generics
    imp: TokenStream,

    /// #inline pub fn
    func: TokenStream,
    /// #inline pub const fn
    c_func: TokenStream,
    /// ::std::option::Option
    opt: TokenStream,
}

impl DataContainer {
    fn where_clause_for(&self, ty: impl ToTokens) -> Option<TokenStream> {
        self.bounds_for(ty).map(|b| quote! { where #b })
    }
    fn bounds_for(&self, ty: impl ToTokens) -> Option<TokenStream> {
        self.bounds.as_ref().map(|b| quote! { #ty: #b, })
    }
}

/// TODO: doc
/// TODO: talk about `T` as placeholder for the contained type
///
/// ## Generics
///
/// `derive(Optional)` can be done on types with or without generics:
///
/// ```
/// # use derive_optional::Optional;
/// use std::fmt::Display;
/// #[derive(Optional)]
/// enum NoGenerics {
///     SomeVariantWithFixedType(usize),
///     EmptyNoneVariant,
/// }
///
/// #[derive(Optional)]
/// enum WithGenerics<T: Display> where T: Default {
///     SomeVariantWithGenericType(T),
///     StillEmptyNoneVariant,
/// }
/// ```
///
/// Weather or not the enum is generic will affect the available functions, since functions
/// like `map` or `as_ref` need to change the contained type. See [Added Methods](#added-methods)
/// for a list of functions that are affected by this.
///
/// **Important: Not all forms of generics and trait bounds are fully supported.**
///
/// Generics should be limited to only one generic type, and traits are only allowed on
/// the type itself, either directly or in a where clause, like in the example above.  
/// The following would be rejected:
///
/// ```compile_fail
/// # use std::convert::TryFrom;
/// # use derive_optional::Optional;
/// #[derive(Optional)]
/// enum TooManyGenerics<A, B> where A: TryFrom<B> {
///     SomeVariant(<A as TryFrom<B>>::Error),
///     NoneVariant,
/// }
/// ```
///
/// If derive_optional would want to add all methods, it would need to convert generics. Take e.g. the `map` function.
/// It would need to change the contained type from `<A as TryFrom<B>>::Error` to some other type. Would that be
/// `<U as TryFrom<B>>::Error`? Or would an alternative to `B` need to be introduced as well, so `<U as TryFrom<V>>::Error`?
/// Not only is it not clear even in this relatively simple example, it would also be a herculean task to find all
/// instances of the generic types in the arbitrary type-expressions, since procedural macros don't have type information.
/// It would need to rely on what is essentially string search-and-replace to change the types, which is not ideal.
///
/// Thus, at least for now, some methods are simply not added if the generic types or traits are too complex. Please be aware
/// that the detection for this is not ideal, so there may be errors in both directions where the methods aren't added in
/// places where they could be, or they are added and you get weird compiler errors.
///
/// ## Added Methods
///
/// Symbols:
/// - (G): only added if the enum is generic over the contained type, as described in [Generics](#generics)
/// - (U): not yet added because it's unstable
///
/// #### Querying the contained values
/// - `is_<some>` (where `<some>` is the snake_case version of the `Some`-like variant)
/// - `is_<some>_and`
/// - `is_<none>` (same as above, but for the `None`-like variant)
/// - ~~`is_<none>_or`~~ (U)
///
/// #### Adapter for working with references
/// - `as_ref` (G)
/// - `as_mut` (G)
/// - `as_pin_ref` (G)
/// - `as_pin_mut` (G)
/// - `as_slice`
/// - `as_mut_slice`
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
/// - `inspect`
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
/// - ~~`get_or_insert_default`~~ (U)
/// - `get_or_insert_with`
///
/// #### Misc
/// - `take`
/// - `take_if`
/// - `replace`
/// - `contains`
/// - `zip` (G)
/// - `zip_with`
/// - `unzip` (G)
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

    let some_ident = some_variant.ident;
    let none_ident = none_variant.ident;
    let some = quote! {#name::#some_ident};
    let none = quote! {#name::#none_ident};
    let some_name = some_ident.to_string();
    let none_name = none_ident.to_string();
    let some_name_snake = some_name.to_case(Case::Snake);
    let none_name_snake = none_name.to_case(Case::Snake);

    if some_variant.fields.len() != 1 {
        let msg = "Optional currently only supports one type in the variant";
        return Error::err_spanned(some_variant.fields, msg);
    }
    let some_field = some_variant.fields.into_iter().next().unwrap();
    if let Some(ident) = some_field.ident.as_ref() {
        let msg = "Optional currently only supports tuple variants for the `Some` variant";
        return Error::err_spanned(ident, msg);
    }
    let some_ty = some_field.ty;
    let some_ty_name = some_ty.to_token_stream().to_string();

    let (is_generic, bounds, imp, full_name);
    if let Some(in_bounds) = check_generics(input.generics, &some_ty_name)? {
        is_generic = true;
        if !in_bounds.is_empty() {
            imp = quote! {impl<#some_ty: #in_bounds>};
            bounds = Some(in_bounds);
        } else {
            imp = quote! {impl<#some_ty>};
            bounds = None;
        };
        full_name = quote! {#name<#some_ty>};
    } else {
        is_generic = false;
        bounds = None;
        imp = quote! {impl};
        full_name = quote! {#name};
    }
    let full_name_string = full_name.to_string().replace(' ', "");

    let opt = quote! {::std::option::Option};
    let func = quote! {#[inline] pub fn};
    let c_func = quote! {#[inline] pub const fn};

    let container = DataContainer {
        name,
        full_name,
        full_name_string,
        some,
        none,
        some_name,
        none_name,
        some_name_snake,
        none_name_snake,
        some_ty,
        some_ty_name,
        is_generic,
        bounds,
        imp,

        func,
        c_func,
        opt,
    };

    let mut impl_block = TokenStream::new();
    let mut additional_impls = TokenStream::new();

    sections::s99_additions::add_section(&container, &mut impl_block);
    sections::s05_bool_ops::add_section(&container, &mut impl_block);
    sections::s06_entry_ops::add_section(&container, &mut impl_block);
    sections::s03_get_contained::add_section(&container, &mut impl_block);
    sections::s05_iters::add_section(&container, &mut impl_block);
    sections::s07_misc::add_section(&container, &mut impl_block);
    sections::s01_querying::add_section(&container, &mut impl_block);
    sections::s02_ref_adapters::add_section(&container, &mut impl_block);
    sections::s04_transformers::add_section(&container, &mut impl_block);

    external::traits::add_external(&container, &mut additional_impls);
    external::impls::add_external(&container, &mut additional_impls);

    let DataContainer { full_name, imp, .. } = container;

    let tokens = quote! {
        #imp #full_name {
            #impl_block
        }

        #additional_impls
    };

    // println!("//////////////////////////////////////////////////");
    // println!("//////////////////////////////////////////////////");
    // println!("{}", tokens);
    // println!("//////////////////////////////////////////////////");
    // println!("//////////////////////////////////////////////////");

    Ok(tokens)
}

fn check_generics(generics: syn::Generics, some_ty_name: &str) -> Result<Option<Bounds>> {
    let mut generic_type = None;
    let mut out_bounds = Bounds::new();
    let mut error = Error::builder();
    for ty in generics.params.into_iter() {
        match ty {
            syn::GenericParam::Type(ty) => {
                if generic_type.is_some() {
                    let msg = "Optional currently only supports one generic type";
                    error.with_spanned(ty, msg);
                } else {
                    generic_type = Some(ty.ident);
                    out_bounds.extend(ty.bounds);
                }
            }
            _ => {
                let msg = "Optional currently only supports a generic type";
                error.with_spanned(ty, msg);
            }
        }
    }

    if let Some(where_clause) = generics.where_clause {
        for clause in where_clause.predicates.into_iter() {
            match clause {
                syn::WherePredicate::Type(syn::PredicateType {
                    bounded_ty: syn::Type::Path(p),
                    bounds,
                    ..
                }) if p.path.is_ident(some_ty_name) => {
                    out_bounds.extend(bounds);
                }
                _ => {
                    let msg = "Optional currently only supports where clauses on the generic type";
                    error.with_spanned(clause, msg);
                }
            }
        }
    }

    error.ok_or_build()?;

    if let Some(ty) = generic_type {
        if ty != *some_ty_name {
            let msg = "The generic type must be the same as the type in the `Some` variant";
            return Error::err_spanned(ty, msg);
        }

        Ok(Some(out_bounds))
    } else {
        Ok(None)
    }
}
