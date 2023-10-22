use super::*;

pub(crate) fn add_section(container: &DataContainer, impl_block: &mut TokenStream) {
    let DataContainer {
        ref name,
        ref some_ident,
        ref none_ident,
        ref some_snake,
        ref none_snake,
        ref none_pattern,
        ref c_func,
        ..
    } = *container;

    let some__ = container.some(quote! {..});

    /////////////////////////////////////////////////////////////////////////
    // Querying the contained values
    /////////////////////////////////////////////////////////////////////////

    // is_some
    {
        let is_some = Ident::new(&format!("is_{}", some_snake), some_ident.span());
        let doc = format!(
            "Returns `true` if the `{}` is a `{}` value. Equivalent to `Option::is_some`.",
            name, some_ident,
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
            "Returns `true` if the `{}` is a `{}` value. Equivalent to `Option::is_none`.",
            name, none_ident,
        );
        impl_block.extend(quote! {
            #[doc = #doc]
            #c_func #is_none(&self) -> bool {
                matches!(*self, #none_pattern)
            }
        });
    }
}
