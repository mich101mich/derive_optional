use super::*;

pub(crate) fn add_section(container: &DataContainer, impl_block: &mut TokenStream) {
    #[allow(unused_variables)]
    #[rustfmt::skip]
    let DataContainer {
        ref name, ref full_name, ref full_name_string, ref some, ref none, ref some_name, ref none_name,
        ref some_name_snake, ref none_name_snake, ref some_ty, ref some_ty_name, is_generic, ref bounds, ref imp,
        ref func, ref c_func, ref opt
    } = *container;

    /////////////////////////////////////////////////////////////////////////
    // Transforming contained values
    /////////////////////////////////////////////////////////////////////////

    // map
    if is_generic {
        let doc = format!(
            "Maps an `{name}<{ty}>` to `{name}<U>` by applying a function to a contained value. Equivalent to `Option::map`.",
            name = name, ty = some_ty_name,
        );
        // can't be c_func right now because of destructors (https://github.com/rust-lang/rust/issues/67792)
        impl_block.extend(quote! {
            #[doc = #doc]
            #func map<U, F>(self, f: F) -> #name<U>
            where
                F: FnOnce(#some_ty) -> U,
            {
                match self {
                    #some(x) => #some(f(x)),
                    _ => #none,
                }
            }
        });
    }

    // inspect
    // unstable

    // map_or
    if is_generic {
        let doc = format!(
            "Applies a function to the contained value (if any), or returns the provided default (if not). Equivalent to `Option::map_or`.",
        );
        // can't be c_func right now because of destructors (https://github.com/rust-lang/rust/issues/67792)
        impl_block.extend(quote! {
            #[doc = #doc]
            #func map_or<U, F>(self, default: U, f: F) -> U
            where
                F: FnOnce(#some_ty) -> U,
            {
                match self {
                    #some(x) => f(x),
                    _ => default,
                }
            }
        });
    }

    // map_or_else
    if is_generic {
        let doc = format!(
            "Applies a function to the contained value (if any), or calls the provided function to compute a default (if not). Equivalent to `Option::map_or_else`.",
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
                    #some(x) => f(x),
                    _ => default(),
                }
            }
        });
    }

    // ok_or
    {
        let doc = format!(
            "Transforms the `{name}` into a `Result<{ty}, E>`, mapping `{some}(x)` to `Ok(x)` and `{none}` to `Err(err)`. Equivalent to `Option::ok_or`.",
            name = full_name_string, ty = some_ty_name, some = some_name, none = none_name,
        );
        // can't be c_func right now because of destructors (https://github.com/rust-lang/rust/issues/67792)
        impl_block.extend(quote! {
            #[doc = #doc]
            #func ok_or<E>(self, err: E) -> ::std::result::Result<#some_ty, E> {
                match self {
                    #some(x) => ::std::result::Result::Ok(x),
                    _ => ::std::result::Result::Err(err),
                }
            }
        });
    }

    // ok_or_else
    {
        let doc = format!(
            "Transforms the `{name}` into a `Result<{ty}, E>`, mapping `{some}(x)` to `Ok(x)` and `{none}` to `Err(err())`. Equivalent to `Option::ok_or_else`.",
            name = full_name_string, ty = some_ty_name, some = some_name, none = none_name,
        );
        // can't be c_func right now because of destructors (https://github.com/rust-lang/rust/issues/67792)
        impl_block.extend(quote! {
            #[doc = #doc]
            #func ok_or_else<E, F>(self, err: F) -> ::std::result::Result<#some_ty, E>
            where
                F: FnOnce() -> E,
            {
                match self {
                    #some(x) => ::std::result::Result::Ok(x),
                    _ => ::std::result::Result::Err(err()),
                }
            }
        });
    }

    // as_deref
    if is_generic {
        let doc = format!(
            "Creates a `{name}<&{ty}::Target>` from an `&{name}<{ty}>`. Equivalent to `Option::as_deref`.",
            name = name, ty = some_ty_name,
        );
        // can't be c_func right now because of trait bounds (https://github.com/rust-lang/rust/issues/67792)
        impl_block.extend(quote! {
            #[doc = #doc]
            #func as_deref(&self) -> #name<&<#some_ty as ::std::ops::Deref>::Target>
            where
                #some_ty: ::std::ops::Deref,
            {
                match self.as_ref() {
                    #some(x) => #some(x.deref()),
                    _ => #none,
                }
            }
        });
    }

    // as_deref_mut
    if is_generic {
        let doc = format!(
            "Creates a `{name}<&mut {ty}::Target>` from an `&mut {name}<{ty}>`. Equivalent to `Option::as_deref_mut`.",
            name = name, ty = some_ty_name,
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
                    #some(x) => #some(x.deref_mut()),
                    _ => #none,
                }
            }
        });
    }
}
