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
