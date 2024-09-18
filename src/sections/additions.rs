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

    // as_option_ref
    {
        let doc = format!(
            "Converts from `&{name}` to `Option<&{ty}>`.",
            name = full_name_string,
            ty = some_ty_name,
        );
        impl_block.extend(quote! {
            #[doc = #doc]
            #c_func as_option_ref(&self) -> #opt<&#some_ty> {
                match *self {
                    #some_ref_x => #opt::Some(x),
                    _ => #opt::None,
                }
            }
        });
    }

    // as_option_mut
    {
        let doc = format!(
            "Converts from `&mut {name}` to `Option<&mut {ty}>`.",
            name = full_name_string,
            ty = some_ty_name,
        );
        impl_block.extend(quote! {
            #[doc = #doc]
            #func as_option_mut(&mut self) -> #opt<&mut #some_ty> {
                match *self {
                    #some_ref_mut_x => #opt::Some(x),
                    _ => #opt::None,
                }
            }
        });
    }
}
