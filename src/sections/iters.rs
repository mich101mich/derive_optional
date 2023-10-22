use super::*;

pub(crate) fn add_section(container: &DataContainer, impl_block: &mut TokenStream) {
    let DataContainer {
        ref some_ty,
        ref func,
        ..
    } = *container;

    /////////////////////////////////////////////////////////////////////////
    // Iterator constructors
    /////////////////////////////////////////////////////////////////////////

    // iter
    {
        let doc = format!(
            "Returns an iterator over the possibly contained value. Equivalent to `Option::iter`.",
        );
        impl_block.extend(quote! {
            #[doc = #doc]
            #func iter(&self) -> ::std::option::IntoIter<&#some_ty> {
                self.as_option_ref().into_iter()
            }
        });
    }

    // iter_mut
    {
        let doc = format!(
            "Returns a mutable iterator over the possibly contained value. Equivalent to `Option::iter_mut`.",
        );
        impl_block.extend(quote! {
            #[doc = #doc]
            #func iter_mut(&mut self) -> ::std::option::IntoIter<&mut #some_ty> {
                self.as_option_mut().into_iter()
            }
        });
    }
}
