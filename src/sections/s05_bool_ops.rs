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
    // Boolean operations on the values, eager and lazy
    /////////////////////////////////////////////////////////////////////////

    // and
    {
        let doc = format!(
            "Returns `{none}` if the `{name}` is a `{none}`, otherwise returns `optb`. Equivalent to `Option::and`.",
            name = name, none = none_name,
        );
        if is_generic {
            impl_block.extend(quote! {
                #[doc = #doc]
                #func and<U>(self, optb: #name<U>) -> #name<U> {
                    match self {
                        #some(_) => optb,
                        _ => #none,
                    }
                }
            });
        } else {
            impl_block.extend(quote! {
                #[doc = #doc]
                #func and(self, optb: Self) -> Self {
                    match self {
                        #some(_) => optb,
                        _ => #none,
                    }
                }
            });
        }
    }

    // and_then
    {
        let doc = format!(
            "Returns `{name}` if the `{none}` is a `{name}`, otherwise calls `f` and returns the result. Equivalent to `Option::and_then`.",
            name = name, none = none_name,
        );
        if is_generic {
            impl_block.extend(quote! {
                #[doc = #doc]
                #func and_then<U, F>(self, f: F) -> #name<U>
                where
                    F: FnOnce(#some_ty) -> #name<U>,
                {
                    match self {
                        #some(x) => f(x),
                        _ => #none,
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
                        #some(x) => f(x),
                        _ => #none,
                    }
                }
            });
        }
    }

    // filter
    {
        let doc = format!(
            "Returns a `{some}` if the `{name}` is a `{some}` and the contained value satisfies the predicate `pred`, otherwise returns `{none}`. Equivalent to `Option::filter`.",
            name = name, none = none_name, some = some_name,
        );
        impl_block.extend(quote! {
            #[doc = #doc]
            #func filter<P>(self, pred: P) -> Self
            where
                P: FnOnce(&#some_ty) -> bool,
            {
                match self {
                    #some(x) if pred(&x) => #some(x),
                    _ => #none,
                }
            }
        });
    }

    // or
    {
        let doc = format!(
            "Returns the `{name}` if it is a `{some}`, otherwise returns `optb`. Equivalent to `Option::or`.",
            name = name, some = some_name,
        );
        impl_block.extend(quote! {
            #[doc = #doc]
            #func or(self, optb: Self) -> Self {
                match self {
                    #some(x) => #some(x),
                    _ => optb,
                }
            }
        });
    }

    // or_else
    {
        let doc = format!(
            "Returns the `{name}` if it is a `{some}`, otherwise calls `f` and returns the result. Equivalent to `Option::or_else`.",
            name = name, some = some_name,
        );
        impl_block.extend(quote! {
            #[doc = #doc]
            #func or_else<F>(self, f: F) -> Self
            where
                F: FnOnce() -> Self,
            {
                match self {
                    #some(x) => #some(x),
                    _ => f(),
                }
            }
        });
    }

    // xor
    {
        let doc = format!(
            "Returns `{some}` if exactly one of `self` or `optb` is a `{some}`, otherwise returns `{none}`. Equivalent to `Option::xor`.",
            none = none_name, some = some_name,
        );
        impl_block.extend(quote! {
            #[doc = #doc]
            #func xor(self, optb: Self) -> Self {
                match (self, optb) {
                    (#some(x), #none) | (#none, #some(x)) => #some(x),
                    _ => #none,
                }
            }
        });
    }
}
