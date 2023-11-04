use super::*;

pub(crate) fn add_section(container: &DataContainer, impl_block: &mut TokenStream) {
    let DataContainer {
        ref full_name_string,
        ref some_ty,
        ref some_ty_name,
        ref func,
        ref c_func,
        ..
    } = *container;

    let some_ref_x = container.some(quote! {ref x});
    let some_ref_mut_x = container.some(quote! {ref mut x});

    // as_option_ref
    {
        let doc = format!(
            "Converts from `&{name}` to `Option<&{ty}>`.",
            name = full_name_string,
            ty = some_ty_name,
        );
        impl_block.extend(quote! {
            #[doc = #doc]
            #c_func as_option_ref(&self) -> ::std::option::Option<&#some_ty> {
                match *self {
                    #some_ref_x => ::std::option::Option::Some(x),
                    _ => ::std::option::Option::None,
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
            #func as_option_mut(&mut self) -> ::std::option::Option<&mut #some_ty> {
                match *self {
                    #some_ref_mut_x => ::std::option::Option::Some(x),
                    _ => ::std::option::Option::None,
                }
            }
        });
    }
}
