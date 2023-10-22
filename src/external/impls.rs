use super::*;

pub(crate) fn add_external(_container: &DataContainer, _additional_impls: &mut TokenStream) {
    // TODO: unzip on #name<(#some_ty, U)>
    // TODO: copied on #name<&#some_ty> where #some_ty: Copy
    // TODO: cloned on #name<&#some_ty> where #some_ty: Clone
    // TODO: copied on #name<&mut #some_ty> where #some_ty: Copy
    // TODO: cloned on #name<&mut #some_ty> where #some_ty: Clone
    // TODO: transpose on #name<Result<#some_ty, E>>
    // TODO: transpose on #name<Option<#some_ty>>
    // TODO: transpose on Option<#name<#some_ty>>
    // TODO: flatten on #name<#name<#some_ty>>
    // TODO: flatten on #name<Option<#some_ty>> -> #name<#some_ty>
    // TODO: flatten on Option<#name<#some_ty>> -> #name<#some_ty>
}
