use super::*;

pub(crate) fn add_section(container: &DataContainer, impl_block: &mut TokenStream) {
    let DataContainer {
        ref name,
        ref some_ident,
        ref none_ident,
        ref none_pattern,
        ref some_ty,
        is_generic,
        ref func,
        ..
    } = *container;

    let some_x = container.some(quote! {x});
    let some_y = container.some(quote! {y});
    let some_xy = container.some(quote! {(x, y)});

    /////////////////////////////////////////////////////////////////////////
    // Misc
    /////////////////////////////////////////////////////////////////////////

    // take
    {
        let doc = format!(
            "Replaces the `{}` with `{}` and returns the old value, if any. Equivalent to `Option::take`.",
            name, none_ident,
        );
        impl_block.extend(quote! {
            #[doc = #doc]
            #func take(&mut self) -> Self {
                ::std::mem::take(self)
            }
        });
    }

    // replace
    {
        let doc = format!(
            "Replaces the actual value in the `{}` with the provided one, returning the old value, if any. Equivalent to `Option::replace`.",
            name,
        );
        impl_block.extend(quote! {
            #[doc = #doc]
            #func replace(&mut self, x: #some_ty) -> Self {
                ::std::mem::replace(self, #some_x)
            }
        });
    }

    // contains
    {
        let doc = format!(
            "Returns `true` if the `{}` contains the given value. Equivalent to `Option::contains`.",
            name,
        );
        impl_block.extend(quote! {
            #[doc = #doc]
            #func contains(&self, x: &#some_ty) -> bool
            where
                #some_ty: ::std::cmp::PartialEq,
            {
                match self {
                    #some_y => ::std::cmp::PartialEq::eq(x, y),
                    _ => false,
                }
            }
        });
    }

    // zip
    if is_generic {
        let doc = format!(
            "zips `self` with another `{}` and returns the pair of contained values if both are `{}`s. Equivalent to `Option::zip`.",
            name, some_ident,
        );
        impl_block.extend(quote! {
            #[doc = #doc]
            #func zip<U>(self, other: #name<U>) -> #name<(#some_ty, U)> {
                match (self, other) {
                    (#some_x, #some_y) => #some_xy,
                    _ => #none_pattern,
                }
            }
        });
    }

    // zip_with
    {
        let doc = format!(
            "zips `self` with another `{}` and returns the result of the provided function if both are `{}`s. Equivalent to `Option::zip_with`.",
            name, some_ident,
        );
        let pattern = container.some(quote! {f(x, y)});
        if is_generic {
            impl_block.extend(quote! {
                #[doc = #doc]
                #func zip_with<U, F, R>(self, other: #name<U>, f: F) -> #name<R>
                where
                    F: FnOnce(#some_ty, U) -> R,
                {
                    match (self, other) {
                        (#some_x, #some_y) => #pattern,
                        _ => #none_pattern,
                    }
                }
            });
        } else {
            impl_block.extend(quote! {
                #[doc = #doc]
                #func zip_with<F>(self, other: Self, f: F) -> Self
                where
                    F: FnOnce(#some_ty, #some_ty) -> #some_ty,
                {
                    match (self, other) {
                        (#some_x, #some_y) => #pattern,
                        _ => #none_pattern,
                    }
                }
            });
        }
    }
}
