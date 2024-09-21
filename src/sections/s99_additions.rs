use super::*;

pub(crate) fn add_section(container: &DataContainer, impl_block: &mut TokenStream) {
    #[allow(unused_variables)]
    #[rustfmt::skip]
    let DataContainer {
        ref name, ref full_name, ref full_name_string, ref some, ref none, ref some_name, ref none_name,
        ref some_name_snake, ref none_name_snake, ref some_ty, ref some_ty_name, is_generic, ref bounds, ref imp,
        ref func, ref c_func, ref opt
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
            #func as_option_ref(&self) -> #opt<&#some_ty> {
                match *self {
                    #some(ref x) => #opt::Some(x),
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
                    #some(ref mut x) => #opt::Some(x),
                    _ => #opt::None,
                }
            }
        });
    }
}
