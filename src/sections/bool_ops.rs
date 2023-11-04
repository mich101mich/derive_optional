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
            "Returns `{none}` if the `{name}` is a `{none}`, otherwise returns `optb`. Equivalent to `Option::and`.",
            name = name, none = none_ident,
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
            "Returns `{name}` if the `{none}` is a `{name}`, otherwise calls `f` and returns the result. Equivalent to `Option::and_then`.",
            name = name, none = none_ident,
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
            "Returns a `{some}` if the `{name}` is a `{some}` and the contained value satisfies the predicate `pred`, otherwise returns `{none}`. Equivalent to `Option::filter`.",
            name = name, none = none_ident, some = some_ident,
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
            "Returns the `{name}` if it is a `{some}`, otherwise returns `optb`. Equivalent to `Option::or`.",
            name = name, some = some_ident,
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
            "Returns the `{name}` if it is a `{some}`, otherwise calls `f` and returns the result. Equivalent to `Option::or_else`.",
            name = name, some = some_ident,
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
            "Returns `{some}` if exactly one of `self` or `optb` is a `{some}`, otherwise returns `{none}`. Equivalent to `Option::xor`.",
            none = none_ident, some = some_ident,
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
