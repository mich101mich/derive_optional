use super::*;

pub(crate) fn add_section(container: &DataContainer, impl_block: &mut TokenStream) {
    #[allow(unused_variables)]
    #[rustfmt::skip]
    let DataContainer {
        ref name, ref full_name, ref full_name_string,
        ref some_ident, ref none_ident, ref some_snake, ref none_snake, ref none_pattern,
        ref some_ty, ref some_field_ident, ref some_ty_name, is_generic,
        ref imp, ref wheres, ref where_clause,
        ref some_x, ref some_ref_x, ref some_ref_mut_x, ref some__, ref some_y, ref some_xy,
        ref func, ref c_func, ref opt,
        ..
    } = *container;

    /////////////////////////////////////////////////////////////////////////
    // Querying the contained values
    /////////////////////////////////////////////////////////////////////////

    // is_some
    {
        let is_some = Ident::new(&format!("is_{}", some_snake), some_ident.span());
        let doc = format!(
            "Returns `true` if the `{name}` is a `{some}` value. Equivalent to `Option::is_some`.",
            name = name,
            some = some_ident,
        );
        // some_match might have {} or (), so match against none instead
        impl_block.extend(quote! {
            #[doc = #doc]
            #c_func #is_some(&self) -> bool {
                matches!(*self, #some__)
            }
        });
    }

    // is_some_and
    // unstable

    // is_none
    {
        let is_none = Ident::new(&format!("is_{}", none_snake), none_ident.span());
        let doc = format!(
            "Returns `true` if the `{name}` is a `{none}` value. Equivalent to `Option::is_none`.",
            name = name,
            none = none_ident,
        );
        impl_block.extend(quote! {
            #[doc = #doc]
            #c_func #is_none(&self) -> bool {
                matches!(*self, #none_pattern)
            }
        });
    }
}
