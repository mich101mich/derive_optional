use super::*;

pub(crate) fn add_section(container: &DataContainer, impl_block: &mut TokenStream) {
    let DataContainer {
        ref name,
        ref none_pattern,
        ref some_ty,
        ref some_ty_name,
        is_generic,
        ref func,
        ..
    } = *container;

    let some_x = container.some(quote! {x});

    /////////////////////////////////////////////////////////////////////////
    // Transforming contained values
    /////////////////////////////////////////////////////////////////////////

    // map
    if is_generic {
        let value = container.some(quote! {f(x)});
        let doc = format!(
            "Maps an `{0}<{1}>` to `{0}<U>` by applying a function to a contained value. Equivalent to `Option::map`.",
            name, some_ty_name,
        );
        // can't be c_func right now because of destructors (https://github.com/rust-lang/rust/issues/67792)
        impl_block.extend(quote! {
            #[doc = #doc]
            #func map<U, F>(self, f: F) -> #name<U>
            where
                F: FnOnce(#some_ty) -> U,
            {
                match self {
                    #some_x => #value,
                    _ => #none_pattern,
                }
            }
        });
    }

    // inspect
    // unstable

    // map_or
    if is_generic {
        let doc = format!(
            "Applies a function to the contained value (if any), or returns a default (if not). Equivalent to `Option::map_or`.",
        );
        // can't be c_func right now because of destructors (https://github.com/rust-lang/rust/issues/67792)
        impl_block.extend(quote! {
            #[doc = #doc]
            #func map_or<U, F>(self, default: U, f: F) -> U
            where
                F: FnOnce(#some_ty) -> U,
            {
                match self {
                    #some_x => f(x),
                    _ => default,
                }
            }
        });
    }

    // map_or_else
    if is_generic {
        let doc = format!(
            "Applies a function to the contained value (if any), or computes a default (if not). Equivalent to `Option::map_or_else`.",
        );
        // can't be c_func right now because of destructors (https://github.com/rust-lang/rust/issues/67792)
        impl_block.extend(quote! {
            #[doc = #doc]
            #func map_or_else<U, D, F>(self, default: D, f: F) -> U
            where
                D: FnOnce() -> U,
                F: FnOnce(#some_ty) -> U,
            {
                match self {
                    #some_x => f(x),
                    _ => default(),
                }
            }
        });
    }

    // ok_or
    {
        let doc = format!(
            "Transforms the `{}` into a `Result<{}, E>`, mapping `{}` to `Ok(x)` and `{}` to `Err(err)`. Equivalent to `Option::ok_or`.",
            name, some_ty_name, some_x, none_pattern,
        );
        // can't be c_func right now because of destructors (https://github.com/rust-lang/rust/issues/67792)
        impl_block.extend(quote! {
            #[doc = #doc]
            #func ok_or<E>(self, err: E) -> ::std::result::Result<#some_ty, E> {
                match self {
                    #some_x => ::std::result::Result::Ok(x),
                    _ => ::std::result::Result::Err(err),
                }
            }
        });
    }

    // ok_or_else
    {
        let doc = format!(
            "Transforms the `{}` into a `Result<{}, E>`, mapping `{}` to `Ok(x)` and `{}` to `Err(err())`. Equivalent to `Option::ok_or_else`.",
            name, some_ty_name, some_x, none_pattern,
        );
        // can't be c_func right now because of destructors (https://github.com/rust-lang/rust/issues/67792)
        impl_block.extend(quote! {
            #[doc = #doc]
            #func ok_or_else<E, F>(self, err: F) -> ::std::result::Result<#some_ty, E>
            where
                F: FnOnce() -> E,
            {
                match self {
                    #some_x => ::std::result::Result::Ok(x),
                    _ => ::std::result::Result::Err(err()),
                }
            }
        });
    }

    // as_deref
    if is_generic {
        let value = container.some(quote! {x.deref()});
        let doc = format!(
            "Creates a `{0}<&{1}::Target>` from an `&{0}<{1}>`. Equivalent to `Option::as_deref`.",
            name, some_ty_name,
        );
        // can't be c_func right now because of trait bounds (https://github.com/rust-lang/rust/issues/67792)
        impl_block.extend(quote! {
            #[doc = #doc]
            #func as_deref(&self) -> #name<&<#some_ty as ::std::ops::Deref>::Target>
            where
                #some_ty: ::std::ops::Deref,
            {
                match self.as_ref() {
                    #some_x => #value,
                    _ => #none_pattern,
                }
            }
        });
    }

    // as_deref_mut
    if is_generic {
        let value = container.some(quote! {x.deref_mut()});
        let doc = format!(
            "Creates a `{0}<&mut {1}::Target>` from an `&mut {0}<{1}>`. Equivalent to `Option::as_deref_mut`.",
            name, some_ty_name,
        );
        // can't be c_func right now because of trait bounds (https://github.com/rust-lang/rust/issues/67792)
        // and &mut (https://github.com/rust-lang/rust/issues/57349)
        impl_block.extend(quote! {
            #[doc = #doc]
            #func as_deref_mut(&mut self) -> #name<&mut <#some_ty as ::std::ops::Deref>::Target>
            where
                #some_ty: ::std::ops::DerefMut,
            {
                match self.as_mut() {
                    #some_x => #value,
                    _ => #none_pattern,
                }
            }
        });
    }
}
