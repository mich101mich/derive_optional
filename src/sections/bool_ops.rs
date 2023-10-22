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
    let some__ = container.some(quote! {..});

    /////////////////////////////////////////////////////////////////////////
    // Boolean operations on the values, eager and lazy
    /////////////////////////////////////////////////////////////////////////

    // and
    {
        let doc = format!(
            "Returns `{1}` if the `{0}` is a `{1}`, otherwise returns `optb`. Equivalent to `Option::and`.",
            name, none_ident,
        );
        if is_generic {
            impl_block.extend(quote! {
                #[doc = #doc]
                #func and<U>(self, optb: #name<U>) -> #name<U> {
                    match self {
                        #some__ => optb,
                        _ => #none_pattern,
                    }
                }
            });
        } else {
            impl_block.extend(quote! {
                #[doc = #doc]
                #func and(self, optb: Self) -> Self {
                    match self {
                        #some__ => optb,
                        _ => #none_pattern,
                    }
                }
            });
        }
    }

    // and_then
    {
        let doc = format!(
            "Returns `{1}` if the `{0}` is a `{1}`, otherwise calls `f` and returns the result. Equivalent to `Option::and_then`.",
            name, none_ident,
        );
        if is_generic {
            impl_block.extend(quote! {
                #[doc = #doc]
                #func and_then<U, F>(self, f: F) -> #name<U>
                where
                    F: FnOnce(#some_ty) -> #name<U>,
                {
                    match self {
                        #some_x => f(x),
                        _ => #none_pattern,
                    }
                }
            });
        } else {
            impl_block.extend(quote! {
                #[doc = #doc]
                #func and_then<F>(self, f: F) -> Self
                where
                    F: FnOnce(#some_ty) -> Self,
                {
                    match self {
                        #some_x => f(x),
                        _ => #none_pattern,
                    }
                }
            });
        }
    }

    // filter
    {
        let doc = format!(
            "Returns a `{2}` if the `{0}` is a `{2}` and the contained value satisfies the predicate `pred`, otherwise returns `{1}`. Equivalent to `Option::filter`.",
            name, none_ident, some_ident,
        );
        impl_block.extend(quote! {
            #[doc = #doc]
            #func filter<P>(self, pred: P) -> Self
            where
                P: FnOnce(&#some_ty) -> bool,
            {
                match self {
                    #some_x if pred(&x) => #some_x,
                    _ => #none_pattern,
                }
            }
        });
    }

    // or
    {
        let doc = format!(
            "Returns the `{0}` if it is a `{1}`, otherwise returns `optb`. Equivalent to `Option::or`.",
            name, some_ident,
        );
        impl_block.extend(quote! {
            #[doc = #doc]
            #func or(self, optb: Self) -> Self {
                match self {
                    #some_x => #some_x,
                    _ => optb,
                }
            }
        });
    }

    // or_else
    {
        let doc = format!(
            "Returns the `{0}` if it is a `{1}`, otherwise calls `f` and returns the result. Equivalent to `Option::or_else`.",
            name, some_ident,
        );
        impl_block.extend(quote! {
            #[doc = #doc]
            #func or_else<F>(self, f: F) -> Self
            where
                F: FnOnce() -> Self,
            {
                match self {
                    #some_x => #some_x,
                    _ => f(),
                }
            }
        });
    }

    // xor
    {
        let doc = format!(
            "Returns `{1}` if exactly one of `self` or `optb` is a `{1}`, otherwise returns `{0}`. Equivalent to `Option::xor`.",
            none_ident, some_ident,
        );
        impl_block.extend(quote! {
            #[doc = #doc]
            #func xor(self, optb: Self) -> Self {
                match (self, optb) {
                    (#some_x, #none_pattern) | (#none_pattern, #some_x) => #some_x,
                    _ => #none_pattern,
                }
            }
        });
    }
}
