use super::*;
use syn::Ident;

pub(crate) fn add_section(container: &DataContainer, impl_block: &mut TokenStream) {
    #[allow(unused_variables)]
    #[rustfmt::skip]
    let DataContainer {
        ref name, ref full_name, ref full_name_string, ref some, ref none, ref some_name, ref none_name,
        ref some_name_snake, ref none_name_snake, ref some_ty, ref some_ty_name, is_generic, ref bounds, ref imp,
        ref func, ref c_func, ref opt
    } = *container;

    /////////////////////////////////////////////////////////////////////////
    // Querying the contained values
    /////////////////////////////////////////////////////////////////////////

    // is_some
    {
        let is_some = Ident::new(
            &format!("is_{}", some_name_snake),
            some.clone().into_iter().last().unwrap().span(),
        );
        let doc = format!(
            "Returns `true` if the `{name}` is a `{some}` value. Equivalent to `Option::is_some`.",
            name = name,
            some = some_name,
        );
        // some_match might have {} or (), so match against none instead
        impl_block.extend(quote! {
            #[doc = #doc]
            #func #is_some(&self) -> bool {
                matches!(*self, #some(_))
            }
        });
    }

    // is_some_and
    {
        let is_some_and = Ident::new(
            &format!("is_{}_and", some_name_snake),
            some.clone().into_iter().last().unwrap().span(),
        );
        let doc = format!(
            "Returns `true` if the `{name}` is a `{some}` value and the predicate `f` returns `true`.",
            name = name,
            some = some_name,
        );
        impl_block.extend(quote! {
            #[doc = #doc]
            #func #is_some_and<F>(self, f: F) -> bool
            where
                F: FnOnce(#some_ty) -> bool,
            {
                match self {
                    #some(x) => f(x),
                    _ => false,
                }
            }
        });
    }

    // is_none
    {
        let is_none = Ident::new(
            &format!("is_{}", none_name_snake),
            none.clone().into_iter().last().unwrap().span(),
        );
        let doc = format!(
            "Returns `true` if the `{name}` is a `{none}` value. Equivalent to `Option::is_none`.",
            name = name,
            none = none_name,
        );
        impl_block.extend(quote! {
            #[doc = #doc]
            #func #is_none(&self) -> bool {
                matches!(*self, #none)
            }
        });
    }

    // is_none_or
    {
        let is_none_or = Ident::new(
            &format!("is_{}_or", none_name_snake),
            none.clone().into_iter().last().unwrap().span(),
        );
        let doc = format!(
            "Returns `true` if the `{name}` is a `{none}` value or the predicate `f` returns `true`.",
            name = name,
            none = none_name,
        );
        impl_block.extend(quote! {
            #[doc = #doc]
            #func #is_none_or<F>(self, f: F) -> bool
            where
                F: FnOnce(#some_ty) -> bool,
            {
                match self {
                    #none => true,
                    #some(x) => f(x),
                }
            }
        });
    }
}
