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
#![cfg_attr(feature = "try_op", allow(unstable_features))]
#![cfg_attr(feature = "try_op", feature(try_trait_v2))]

//! TODO: doc
//!
//! ## Features
//!
//! ### `try_op`
//! Enables the unstable `std::ops::Try` implementation. **Requires nightly**.  
//! This feature is disabled by default.
//!
//! Note that this allows the `?` operator for your enum type and conversions to and from `Option`:
//!
//! ```rust
//! # #![cfg_attr(feature = "try_op", feature(try_trait_v2))]
//! # use derive_optional::Optional;
//! #
//! #[derive(Optional, PartialEq, Debug)]
//! enum MyType {
//!     SomeVariant(u8),
//!     NoneVariant,
//! }
//!
//! #[cfg(feature = "try_op")]
//! {
//!     fn returns_my_type() -> MyType {
//!         assert_eq!(MyType::SomeVariant(42)?, 42);
//!
//!         MyType::NoneVariant?; // early return
//!         unreachable!()
//!     }
//!     fn returns_option() -> Option<u8> {
//!         assert_eq!(MyType::SomeVariant(42)?, 42);
//!
//!         MyType::NoneVariant?; // early return
//!         unreachable!()
//!     }
//!     fn returns_my_type_from_option() -> MyType {
//!         assert_eq!(Some(42)?, 42);
//!
//!         None?; // early return
//!         unreachable!()
//!     }
//!
//!     assert_eq!(returns_my_type(), MyType::NoneVariant);
//!     assert_eq!(returns_option(), None);
//!     assert_eq!(returns_my_type_from_option(), MyType::NoneVariant);
//! }
//! ```
//!

mod utils;
use utils::*;

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
///
/// ## Generics
///
/// `derive(Optional)` can be done on types with or without generics:
///
/// ```
/// # #![cfg_attr(feature = "try_op", feature(try_trait_v2))]
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
/// # #![cfg_attr(feature = "try_op", feature(try_trait_v2))]
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
/// - `is_<some>` (where `<some>` is the snake_case version of the `'some'`-like variant)
/// - `is_<some>_and` (see above)
/// - `is_<none>` (same as above, but for the `none`-like variant)
/// - `is_<none>_or` (see above)
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
/// - `map_or`
/// - `map_or_else`
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
/// - `zip_with` [^1]
/// - `unzip` (G)
/// - `copied` (G)
/// - `cloned` (G)
/// - `transpose` (G)
/// - `flatten` (G)
///
/// ## Additional Methods not in `Option`
/// - `as_option_ref`: Converts `&Self` to `Option<&inner>`, similar to `as_ref`
///   but swapping `Self` with `Option`
/// - `as_option_mut`: Converts `&mut Self` to `Option<&mut inner>`, similar to `as_mut`
///   but swapping `Self` with `Option`
/// - `flatten` on `Self<Option<inner>>`: Flattens a nested `Self<Option<inner>>` into `Self<inner>`.
///   This is similar to `Option::flatten` (and `Self::flatten`), but with mixed `Option` and `Self`.
///   - Note that the corresponding `Option<Self>` is not implemented, as it would require extending
///     the existing `Option` type.
///
/// ## Traits
/// - `From<inner type> for Self`
/// - `From<Option<inner type>> for Self`
/// - `From<Self> for Option<inner type>`
/// - `Self: Default` (returns the `none` variant, does not require `Default` for the inner type)
///
/// ## Things that were **not** added
/// - unstable or nightly-only methods and traits
///   - this sadly includes the try (`?`) operator, though it can be used on nightly with the [`try_op`](index.html#try_op) feature
///
/// [^1]: `zip_with` is in its original form only available for generic enums. For non-generic types there is a slightly
///       modified version where the zipping function is required to return the contained type of the enum, rather than
///       an arbitrary generic type.
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

    let (some_variant, none_variant, some_field) = parse_variants(data.variants, &name)?;

    let some_ident = some_variant.ident;
    let none_ident = none_variant.ident;
    let some = quote! {#name::#some_ident};
    let none = quote! {#name::#none_ident};
    let some_name = some_ident.to_string();
    let none_name = none_ident.to_string();
    let some_name_snake = some_name.to_case(Case::Snake);
    let none_name_snake = none_name.to_case(Case::Snake);

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

fn parse_variants(
    variants: syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
    enum_name: &syn::Ident,
) -> Result<(syn::Variant, syn::Variant, syn::Field)> {
    // find the two variants
    let mut iter = variants.into_iter();
    let (mut a, mut b) = if let Some((a, b)) = iter.next().zip(iter.next()) {
        // can't use let-else yet due to msrv
        (a, b)
    } else {
        let msg = "Optional needs exactly two variants";
        return Error::err_spanned(enum_name, msg);
    };
    if let Some(after) = iter.next() {
        // check for more variants
        let msg = "Optional only works when there are exactly 2 enum variants";
        return Error::builder()
            .with_spanned(after, msg)
            .with_spans(iter, msg) // add all other variants
            .build_err();
    }

    // figure out which one is the 'some' variant and which one is the 'none' variant
    use syn::Fields::*;
    let (fields, should_swap) = match (&a.fields, &b.fields) {
        // the correct cases
        (Unnamed(fields), Unit) => (fields.clone(), false),
        (Unit, Unnamed(fields)) => (fields.clone(), true),

        // the slightly wrong cases
        (Named(f), Unit) | (Unit, Named(f)) => {
            let wrong_ident = &if matches!(b.fields, Unit) { &a } else { &b }.ident;
            let hint = match f.named.len() {
                0 => format!("`{}(...)`", wrong_ident),
                1 => format!(
                    "`{}({})`",
                    wrong_ident,
                    f.named.first().unwrap().ty.to_token_stream()
                ),
                _ => format!(
                    "`{0}({1}{0})`
      with `struct {1}{0} {{ {2} }}`", // if this ever becomes a code suggestion, remember to add the generics from the enum
                    wrong_ident,
                    enum_name,
                    f.named.to_token_stream().to_string().replace(" : ", ": ")
                ),
            };
            let msg = format!(
                "Optional currently does not support named fields in the 'some' variant, only tuple variants.
Hint: Change this variant to for example {}",
                hint
            );
            return Error::err_spanned(f, msg);
        }
        (Unnamed(_), Unnamed(_)) => {
            let msg = "Optional requires exactly one of its variants (the 'none' variant) to have no fields";
            return Error::err_from_spans([a, b], msg);
        }
        (Unit, Unit) => {
            let msg = "Optional requires exactly one of its variants (the 'some' variant) to contain data";
            return Error::err_from_spans([a, b], msg);
        }

        // everything else
        _ => {
            let msg = "Optional needs one unit variant (the 'none' variant) and one variant with data (the 'some' variant)";
            return Error::err_from_spans([a, b], msg);
        }
    };
    if should_swap {
        std::mem::swap(&mut a, &mut b);
    }

    // find the field in the 'some' variant
    let mut fields = fields.unnamed.into_iter();
    let field = if let Some(field) = fields.next() {
        // can't use let-else yet due to msrv
        field
    } else {
        let msg = "Optional needs exactly one field in the 'some' variant";
        return Error::err_spanned(a, msg);
    };
    if let Some(after) = fields.next() {
        let msg = "Optional only works when there is exactly one field in the 'some' variant";
        return Error::builder()
            .with_spanned(after, msg)
            .with_spans(fields, msg) // add all other fields
            .build_err();
    }

    // check that the field is in the correct format
    if let Some(ident) = field.ident.as_ref() {
        // note that this is technically an error in syn, since we only take syn::Fields::Unnamed
        let msg = format!(
            "Optional currently only supports tuple variants for the 'some' variant like `{}({})`",
            ident,
            field.ty.to_token_stream()
        );
        return Error::err_spanned(ident, msg);
    }

    Ok((a, b, field))
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
            let msg = "The generic type must be the same as the type in the 'some' variant";
            return Error::err_spanned(ty, msg);
        }

        Ok(Some(out_bounds))
    } else {
        Ok(None)
    }
}
