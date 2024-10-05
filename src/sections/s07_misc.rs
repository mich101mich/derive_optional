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
    // Misc
    /////////////////////////////////////////////////////////////////////////

    // take
    {
        let doc = format!(
            "Takes the actual value out of the `{name}`, leaving a `{none}` in its place. Equivalent to `Option::take`.",
            name = name, none = none_name,
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
            "Replaces the actual value in the `{name}` with the provided one, returning the old value, if any. Equivalent to `Option::replace`.",
            name = name,
        );
        impl_block.extend(quote! {
            #[doc = #doc]
            #func replace(&mut self, x: #some_ty) -> Self {
                ::std::mem::replace(self, #some(x))
            }
        });
    }

    // contains
    {
        let doc = format!(
            "Returns `true` if the `{name}` contains the given value. Equivalent to `Option::contains`.",
            name = name,
        );
        impl_block.extend(quote! {
            #[doc = #doc]
            #func contains(&self, x: &#some_ty) -> bool
            where
                #some_ty: ::std::cmp::PartialEq,
            {
                match self {
                    #some(y) => ::std::cmp::PartialEq::eq(x, y),
                    _ => false,
                }
            }
        });
    }

    // zip
    if is_generic {
        let doc = format!(
            "zips `self` with another `{name}` and returns the pair of contained values if both are `{some}`s. Equivalent to `Option::zip`.",
            name = name, some = some_name,
        );
        let u_where = container.where_clause_for(quote! {U});
        let tuple_bounds = container.bounds_for(quote! {(#some_ty, U)});
        impl_block.extend(quote! {
            #[doc = #doc]
            #func zip<U>(self, other: #name<U>) -> #name<(#some_ty, U)> #u_where #tuple_bounds {
                match (self, other) {
                    (#some(x), #some(y)) => #some((x, y)),
                    _ => #none,
                }
            }
        });
    }

    // zip_with
    {
        if is_generic {
            let doc = format!(
                "zips `self` with another `{name}` and returns the result of the provided function if both are `{some}`s. Equivalent to `Option::zip_with`.",
                name = name, some = some_name,
            );
            let u_bounds = container.bounds_for(quote! {U});
            let r_bounds = container.bounds_for(quote! {R});
            impl_block.extend(quote! {
                #[doc = #doc]
                #func zip_with<U, F, R>(self, other: #name<U>, f: F) -> #name<R>
                where
                    F: FnOnce(#some_ty, U) -> R,
                    #u_bounds
                    #r_bounds
                {
                    match (self, other) {
                        (#some(x), #some(y)) => #some(f(x, y)),
                        _ => #none,
                    }
                }
            });
        } else {
            let doc = format!(
                "zips `self` with another `{name}` and returns the result of the provided function if both are `{some}`s. Note that, since `{name}` is not generic over its inner type, the function is required to return `{ty}`. Equivalent to `Option::zip_with`.",
                name = name, some = some_name, ty = some_ty_name,
            );
            impl_block.extend(quote! {
                #[doc = #doc]
                #func zip_with<F>(self, other: Self, f: F) -> Self
                where
                    F: FnOnce(#some_ty, #some_ty) -> #some_ty,
                {
                    match (self, other) {
                        (#some(x), #some(y)) => #some(f(x, y)),
                        _ => #none,
                    }
                }
            });
        }
    }

    // functions moved to external/impls.rs:
    // unzip
    // copied
    // cloned
    // transpose
    // flatten
}
