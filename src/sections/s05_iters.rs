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
