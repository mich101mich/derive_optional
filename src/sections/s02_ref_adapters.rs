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
    // Adapter for working with references
    /////////////////////////////////////////////////////////////////////////

    // as_ref
    if is_generic {
        let doc = format!(
            "Converts from `&{name}<{ty}>` to `{name}<&{ty}>`. Equivalent to `Option::as_ref`.",
            name = name,
            ty = some_ty_name,
        );
        let where_clause = container.where_clause_for(quote! {&'a #some_ty});
        impl_block.extend(quote! {
            #[doc = #doc]
            #c_func as_ref<'a>(&'a self) -> #name<&'a #some_ty> #where_clause {
                match *self {
                    #some(ref x) => #some(x),
                    _ => #none,
                }
            }
        });
    }

    // as_mut
    if is_generic {
        let doc = format!(
            "Converts from `&mut {name}<{ty}>` to `{name}<&mut {ty}>`. Equivalent to `Option::as_mut`.",
            name = name,
            ty = some_ty_name,
        );
        let where_clause = container.where_clause_for(quote! {&'a mut #some_ty});
        // can't be c_func right now because of &mut: see https://github.com/rust-lang/rust/issues/57349
        impl_block.extend(quote! {
            #[doc = #doc]
            #func as_mut<'a>(&'a mut self) -> #name<&'a mut #some_ty> #where_clause {
                match *self {
                    #some(ref mut x) => #some(x),
                    _ => #none,
                }
            }
        });
    }

    // as_pin_ref
    if is_generic {
        let doc = format!(
            "Converts from `Pin<&{name}<{ty}>>` to `{name}<Pin<&{ty}>>`. Equivalent to `Option::as_pin_ref`.",
            name = name,
            ty = some_ty_name,
        );
        let ret_inner = quote! {::std::pin::Pin<&'a #some_ty>};
        let where_clause = container.where_clause_for(&ret_inner);
        // can't be c_func right now because of Pin::<&'a T>::get_ref (https://github.com/rust-lang/rust/issues/76654)
        impl_block.extend(quote! {
            #[doc = #doc]
            #func as_pin_ref<'a>(self: ::std::pin::Pin<&'a Self>) -> #name<#ret_inner> #where_clause {
                match ::std::pin::Pin::get_ref(self) {
                    // SAFETY: `x` is guaranteed to be pinned because it comes from `self`
                    // which is pinned.
                    #some(ref x) => unsafe { #some(::std::pin::Pin::new_unchecked(x)) },
                    _ => #none,
                }
            }
        });
    }

    // as_pin_mut
    if is_generic {
        let doc = format!(
            "Converts from `Pin<&mut {name}<{ty}>>` to `{name}<Pin<&mut {ty}>>`. Equivalent to `Option::as_pin_mut`.",
            name = name,
            ty = some_ty_name,
        );
        let ret_inner = quote! {::std::pin::Pin<&'a mut #some_ty>};
        let where_clause = container.where_clause_for(&ret_inner);
        // can't be c_func right now because of Pin::<&'a mut T>::get_unchecked_mut (https://github.com/rust-lang/rust/issues/76654)
        // and &mut (https://github.com/rust-lang/rust/issues/57349)
        impl_block.extend(quote! {
            #[doc = #doc]
            #func as_pin_mut<'a>(self: ::std::pin::Pin<&'a mut Self>) -> #name<#ret_inner> #where_clause {
                // SAFETY: `get_unchecked_mut` is never used to move the `Option` inside `self`.
                // `x` is guaranteed to be pinned because it comes from `self` which is pinned.
                unsafe {
                    match ::std::pin::Pin::get_unchecked_mut(self) {
                        #some(ref mut x) => #some(::std::pin::Pin::new_unchecked(x)),
                        _ => #none,
                    }
                }
            }
        });
    }

    // as_slice
    {
        let doc = format!(
            "Returns a slice of the contained value, if any. Equivalent to `Option::as_slice`."
        );
        // can't be c_func right now because from_ref is not yet const in our MSRV
        impl_block.extend(quote! {
            #[doc = #doc]
            #func as_slice(&self) -> &[#some_ty] {
                match *self {
                    #some(ref x) => ::std::slice::from_ref(x),
                    _ => &[],
                }
            }
        });
    }

    // as_mut_slice
    {
        let doc = format!(
            "Returns a mutable slice of the contained value, if any. Equivalent to `Option::as_mut_slice`."
        );
        // can't be c_func right now because from_mut is not const yet
        impl_block.extend(quote! {
            #[doc = #doc]
            #func as_mut_slice(&mut self) -> &mut [#some_ty] {
                match *self {
                    #some(ref mut x) => ::std::slice::from_mut(x),
                    _ => &mut [],
                }
            }
        });
    }
}
