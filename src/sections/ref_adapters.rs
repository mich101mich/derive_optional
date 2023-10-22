use super::*;

pub(crate) fn add_section(container: &DataContainer, impl_block: &mut TokenStream) {
    let DataContainer {
        ref name,
        ref none_pattern,
        ref some_ty,
        ref some_ty_name,
        is_generic,
        ref func,
        ref c_func,
        ..
    } = *container;

    let some_x = container.some(quote! {x});
    let some_ref_x = container.some(quote! {ref x});
    let some_ref_mut_x = container.some(quote! {ref mut x});

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
                    #some_ref_x => #some_x,
                    _ => #none_pattern,
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
                    #some_ref_mut_x => #some_x,
                    _ => #none_pattern,
                }
            }
        });
    }

    // as_pin_ref
    if is_generic {
        let value = container.some(quote! {::std::pin::Pin::new_unchecked(x)});
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
                    #some_x => unsafe { #value },
                    _ => #none_pattern,
                }
            }
        });
    }

    // as_pin_mut
    if is_generic {
        let value = container.some(quote! {::std::pin::Pin::new_unchecked(x)});
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
                        #some_x => #value,
                        _ => #none_pattern,
                    }
                }
            }
        });
    }
}
