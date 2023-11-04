use super::*;

pub(crate) fn add_section(container: &DataContainer, impl_block: &mut TokenStream) {
    let DataContainer {
        ref name,
        ref full_name,
        ref full_name_string,
        ref some_ident,
        ref none_ident,
        ref some_snake,
        ref none_snake,
        ref none_pattern,
        ref some_ty,
        ref some_field_ident,
        ref some_ty_name,
        is_generic,
        ref func,
        ref c_func,
        ..
    } = *container;

    let some_x = container.some(quote! {x});
    let some_ref_x = container.some(quote! {ref x});
    let some_ref_mut_x = container.some(quote! {ref mut x});
    let some__ = container.some(quote! {..});
    let some_y = container.some(quote! {y});
    let some_xy = container.some(quote! {(x, y)});
}
