#![allow(unknown_lints)]
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

/// TODO: doc
#[proc_macro_derive(Optional)]
pub fn optional(input: TokenStream1) -> TokenStream1 {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    match optional_internal(input) {
        Ok(s) => {
            println!("{}", s.to_string());
            s.into()
        }
        Err(e) => e.into(),
    }
}

fn optional_internal(input: syn::DeriveInput) -> Result<TokenStream> {
    let name = input.ident;

    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let full_name = quote!(#name #ty_generics);
    let imp = quote!(impl #impl_generics);
    let wheres = where_clause
        .map(|c| c.to_token_stream())
        .unwrap_or_else(TokenStream::new);

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
    let some_snake = some_ident.to_string().to_case(Case::Snake);
    let none_snake = none_ident.to_string().to_case(Case::Snake);
    let none_pattern = quote!(#name::#none_ident);

    if some_variant.fields.len() != 1 {
        let msg = "Optional currently only supports one type in the variant";
        return Error::err_spanned(some_variant.fields, msg);
    }

    let some_field = some_variant.fields.iter().next().unwrap();
    let some_ty = &some_field.ty;
    let some_ty_name = some_ty.to_token_stream().to_string();

    let some_pattern = |pat: TokenStream| {
        if let Some(ident) = some_field.ident.as_ref() {
            quote!(#name::#some_ident { #ident: #pat })
        } else {
            quote!(#name::#some_ident(#pat))
        }
    };

    let is_generic = generics.params.iter().any(|param| {
        if let syn::GenericParam::Type(ty) = param {
            ty.ident == some_ty_name
        } else {
            false
        }
    });

    let func = quote!(#[inline] pub fn);
    let c_func = quote!(#[inline] pub const fn);

    let mut impl_block = TokenStream::new();

    /////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////////
    // Querying the contained values
    /////////////////////////////////////////////////////////////////////////

    // is_some
    {
        let is_some = Ident::new(&format!("is_{}", some_snake), some_ident.span());
        let doc = format!(
            "Returns `true` if the `{}` is a `{}` value. Equivalent to `Option::is_some`.",
            name, some_ident,
        );
        // some_match might have {} or (), so match against none instead
        impl_block.extend(quote!(
            #[doc = #doc]
            #c_func #is_some(&self) -> bool {
                !matches!(*self, #none_pattern)
            }
        ));
    }

    // is_some_and
    // TODO:

    // is_none
    {
        let is_none = Ident::new(&format!("is_{}", none_snake), none_ident.span());
        let doc = format!(
            "Returns `true` if the `{}` is a `{}` value. Equivalent to `Option::is_none`.",
            name, none_ident,
        );
        impl_block.extend(quote!(
            #[doc = #doc]
            #c_func #is_none(&self) -> bool {
                matches!(*self, #none_pattern)
            }
        ));
    }

    /////////////////////////////////////////////////////////////////////////
    // Adapter for working with references
    /////////////////////////////////////////////////////////////////////////

    // as_ref
    if is_generic {
        let pattern = some_pattern(quote!(ref x));
        let value = some_pattern(quote!(x));
        let doc = format!(
            "Converts from `&{name}<{ty}>` to `{name}<&{ty}>`. Equivalent to `Option::as_ref`.",
            name = name,
            ty = some_ty_name,
        );
        impl_block.extend(quote!(
            #[doc = #doc]
            #c_func as_ref(&self) -> #name<&#some_ty> {
                match *self {
                    #pattern => #value,
                    _ => #none_pattern,
                }
            }
        ));
    }

    // as_mut
    if is_generic {
        let pattern = some_pattern(quote!(ref mut x));
        let value = some_pattern(quote!(x));
        let doc = format!(
            "Converts from `&mut {name}<{ty}>` to `{name}<&mut {ty}>`. Equivalent to `Option::as_mut`.",
            name = name,
            ty = some_ty_name,
        );
        // can't be c_func right now because of &mut: see issue #57349 <https://github.com/rust-lang/rust/issues/57349>
        impl_block.extend(quote!(
            #[doc = #doc]
            #func as_mut(&mut self) -> #name<&mut #some_ty> {
                match *self {
                    #pattern => #value,
                    _ => #none_pattern,
                }
            }
        ));
    }

    // as_pin_ref
    if is_generic {
        let pattern = some_pattern(quote!(x));
        let value = some_pattern(quote!(::std::pin::Pin::new_unchecked(x)));
        let doc = format!(
            "Converts from `Pin<&{name}<{ty}>>` to `{name}<Pin<&{ty}>>`. Equivalent to `Option::as_pin_ref`.",
            name = name,
            ty = some_ty_name,
        );
        // can't be c_func right now because of Pin::<&'a T>::get_ref
        impl_block.extend(quote!(
            #[doc = #doc]
            #func as_pin_ref(self: ::std::pin::Pin<&Self>) -> #name<::std::pin::Pin<&#some_ty>> {
                match ::std::pin::Pin::get_ref(self).as_ref() {
                    // SAFETY: `x` is guaranteed to be pinned because it comes from `self`
                    // which is pinned.
                    #pattern => unsafe { #value },
                    _ => #none_pattern,
                }
            }
        ));
    }

    // as_pin_mut
    if is_generic {
        let pattern = some_pattern(quote!(x));
        let value = some_pattern(quote!(::std::pin::Pin::new_unchecked(x)));
        let doc = format!(
            "Converts from `Pin<&mut {name}<{ty}>>` to `{name}<Pin<&mut {ty}>>`. Equivalent to `Option::as_pin_mut`.",
            name = name,
            ty = some_ty_name,
        );
        // can't be c_func right now because of Pin::<&'a mut T>::get_unchecked_mut and &mut
        impl_block.extend(quote!(
            #[doc = #doc]
            #func as_pin_mut(self: ::std::pin::Pin<&mut Self>) -> #name<::std::pin::Pin<&mut #some_ty>> {
                // SAFETY: `get_unchecked_mut` is never used to move the `Option` inside `self`.
                // `x` is guaranteed to be pinned because it comes from `self` which is pinned.
                unsafe {
                    match ::std::pin::Pin::get_unchecked_mut(self).as_mut() {
                        #pattern => #value,
                        _ => #none_pattern,
                    }
                }
            }
        ));
    }

    /////////////////////////////////////////////////////////////////////////
    // Getting to contained values
    /////////////////////////////////////////////////////////////////////////

    // expect
    {
        let pattern = some_pattern(quote!(x));
        let doc = format!(
            "Returns the contained `{}` value, consuming `self`. Equivalent to `Option::expect`.

# Panics

Panics if the value is a `{}` with a custom panic message provided by `msg`.",
            name, none_ident,
        );
        // can't be c_func right now because of `panic`'s formatting. std uses nightly-only functions for this
        impl_block.extend(quote!(
            #[doc = #doc]
            #func expect(self, msg: &str) -> #some_ty {
                match self {
                    #pattern => x,
                    _ => panic!("{}", msg),
                }
            }
        ));
    }

    // unwrap
    {
        let pattern = some_pattern(quote!(x));
        let msg = format!("called `{}::unwrap()` on a `{}` value", name, none_ident);
        let doc = format!(
            "Returns the contained `{}` value, consuming `self`. Equivalent to `Option::unwrap`.

# Panics

Panics if the value is a `{}`.",
            name, none_ident,
        );
        // can't be c_func right now because of `panic`'s formatting. std uses nightly-only functions for this
        impl_block.extend(quote!(
            #[doc = #doc]
            #func unwrap(self) -> #some_ty {
                match self {
                    #pattern => x,
                    _ => panic!("{}", #msg),
                }
            }
        ));
    }

    /////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////

    let mut additional_impls = TokenStream::new();

    // Self: From<Option>
    {
        let value = some_pattern(quote!(x));
        additional_impls.extend(quote!(
            #imp ::std::convert::From<::std::option::Option<#some_ty>> for #full_name #wheres {
                fn from(src: ::std::option::Option<#some_ty>) -> Self {
                    match src {
                        ::std::option::Option::Some(x) => #value,
                        _ => #none_pattern,
                    }
                }
            }
        ));
    }
    // Option: From<Self>
    {
        let pattern = some_pattern(quote!(x));
        additional_impls.extend(quote!(
            #imp ::std::convert::From<#full_name> for ::std::option::Option<#some_ty> #wheres {
                fn from(src: #full_name) -> Self {
                    match src {
                        #pattern => ::std::option::Option::Some(x),
                        _ => ::std::option::Option::None,
                    }
                }
            }
        ));
    }

    Ok(quote!(
        #imp #full_name #wheres {
            #impl_block
        }

        #additional_impls
    ))
}
