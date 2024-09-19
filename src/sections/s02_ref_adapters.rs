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
        impl_block.extend(quote! {
            #[doc = #doc]
            #c_func as_ref(&self) -> #name<&#some_ty> {
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
        // can't be c_func right now because of &mut: see https://github.com/rust-lang/rust/issues/57349
        impl_block.extend(quote! {
            #[doc = #doc]
            #func as_mut(&mut self) -> #name<&mut #some_ty> {
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
        // can't be c_func right now because of Pin::<&'a T>::get_ref (https://github.com/rust-lang/rust/issues/76654)
        impl_block.extend(quote! {
            #[doc = #doc]
            #func as_pin_ref(self: ::std::pin::Pin<&Self>) -> #name<::std::pin::Pin<&#some_ty>> {
                match ::std::pin::Pin::get_ref(self).as_ref() {
                    // SAFETY: `x` is guaranteed to be pinned because it comes from `self`
                    // which is pinned.
                    #some(x) => unsafe { #some(::std::pin::Pin::new_unchecked(x)) },
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
        // can't be c_func right now because of Pin::<&'a mut T>::get_unchecked_mut (https://github.com/rust-lang/rust/issues/76654)
        // and &mut (https://github.com/rust-lang/rust/issues/57349)
        impl_block.extend(quote! {
            #[doc = #doc]
            #func as_pin_mut(self: ::std::pin::Pin<&mut Self>) -> #name<::std::pin::Pin<&mut #some_ty>> {
                // SAFETY: `get_unchecked_mut` is never used to move the `Option` inside `self`.
                // `x` is guaranteed to be pinned because it comes from `self` which is pinned.
                unsafe {
                    match ::std::pin::Pin::get_unchecked_mut(self).as_mut() {
                        #some(x) => #some(::std::pin::Pin::new_unchecked(x)),
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
        impl_block.extend(quote! {
            #[doc = #doc]
            #c_func as_slice(&self) -> &[#some_ty] {
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
