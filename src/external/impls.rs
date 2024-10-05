use super::*;

pub(crate) fn add_external(container: &DataContainer, additional_impls: &mut TokenStream) {
    #[allow(unused_variables)]
    #[rustfmt::skip]
    let DataContainer {
        ref name, ref full_name, ref full_name_string, ref some, ref none, ref some_name, ref none_name,
        ref some_name_snake, ref none_name_snake, ref some_ty, ref some_ty_name, is_generic, ref bounds, ref imp,
        ref func, ref c_func, ref opt
    } = *container;

    // unzip on #name<(T, U)>
    if is_generic {
        let doc = format!(
            "Converts a tuple of `{name}<(T, U)>` into a tuple of ({name}<T>, {name}<U>). Equivalent to `Option::unzip`.",
            name = name,
        );
        let t_where = container.where_clause_for(quote! {T});
        let u_bounds = container.bounds_for(quote! {U});
        let tuple_bounds = container.bounds_for(quote! {(T, U)});
        additional_impls.extend(quote! {
            #[automatically_derived]
            impl<T, U> #name<(T, U)> #t_where #u_bounds #tuple_bounds {
                #[doc = #doc]
                pub fn unzip(self) -> (#name<T>, #name<U>) {
                    match self {
                        #some((x, y)) => (#some(x), #some(y)),
                        _ => (#none, #none),
                    }
                }
            }
        });
    }

    // copied, cloned on #name<&#some_ty>
    if is_generic {
        let copy_doc = format!(
            "Copies the contained value into a new `{name}<#some_ty>` if it implements `Copy`. Equivalent to `Option::copied`.",
            name = name,
        );
        let clone_doc = format!(
            "Clones the contained value into a new `{name}<#some_ty>` if it implements `Clone`. Equivalent to `Option::cloned`.",
            name = name,
        );
        let where_clause = container.where_clause_for(quote! {#some_ty});
        let ref_bounds = container.bounds_for(quote! {&'a #some_ty});
        additional_impls.extend(quote! {
            impl<'a, #some_ty> #name<&'a #some_ty> #where_clause #ref_bounds {
                #[doc = #copy_doc]
                pub fn copied(&self) -> #name<#some_ty> where #some_ty: Copy {
                    match self {
                        #some(x) => #some(**x),
                        _ => #none,
                    }
                }

                #[doc = #clone_doc]
                pub fn cloned(&self) -> #name<#some_ty> where #some_ty: Clone {
                    match self {
                        #some(x) => #some((*x).clone()),
                        _ => #none,
                    }
                }
            }
        });
    }

    // copied, cloned on  #name<&mut #some_ty>
    if is_generic {
        let copy_doc = format!(
            "Copies the contained value into a new `{name}<#some_ty>` if it implements `Copy`. Equivalent to `Option::copied`.",
            name = name,
        );
        let clone_doc = format!(
            "Clones the contained value into a new `{name}<#some_ty>` if it implements `Clone`. Equivalent to `Option::cloned`.",
            name = name,
        );
        let where_clause = container.where_clause_for(quote! {#some_ty});
        let ref_mut_bounds = container.bounds_for(quote! {&'a mut #some_ty});
        additional_impls.extend(quote! {
            impl<'a, #some_ty> #name<&'a mut #some_ty> #where_clause #ref_mut_bounds {
                #[doc = #copy_doc]
                pub fn copied(&self) -> #name<#some_ty> where #some_ty: Copy {
                    match self {
                        #some(x) => #some(**x),
                        _ => #none,
                    }
                }

                #[doc = #clone_doc]
                pub fn cloned(&self) -> #name<#some_ty> where #some_ty: Clone {
                    match self {
                        #some(x) => #some((*x).clone()),
                        _ => #none,
                    }
                }
            }
        });
    }

    // transpose on #name<Result<#some_ty, E>>
    if is_generic {
        let doc = format!(
            "Transposes a `{name}<Result<{ty}, E>>` into a `Result<{name}<{ty}>, E>`. Equivalent to `Option::transpose`.",
            name = name, ty = some_ty_name,
        );
        let where_clause = container.where_clause_for(quote! {#some_ty});
        let result_bounds = container.bounds_for(quote! {Result<#some_ty, E>});
        additional_impls.extend(quote! {
            #[automatically_derived]
            impl<#some_ty, E> #name<Result<#some_ty, E>> #where_clause #result_bounds {
                #[doc = #doc]
                pub fn transpose(self) -> Result<#name<#some_ty>, E> {
                    match self {
                        #some(Ok(x)) => Ok(#some(x)),
                        #some(Err(e)) => Err(e),
                        _ => Ok(#none),
                    }
                }
            }
        });
    }

    // flatten on #name<#name<#some_ty>>
    if is_generic {
        let doc = format!(
            "Flattens a double `{name}<{name}<{ty}>>` into a single `{name}<{ty}>`. Equivalent to `Option::flatten`.",
            name = name, ty = some_ty_name,
        );
        let self_where = container.where_clause_for(quote! {#name<#some_ty>});
        additional_impls.extend(quote! {
            #[automatically_derived]
            #imp #name<#name<#some_ty>> #self_where {
                #[doc = #doc]
                pub fn flatten(self) -> #name<#some_ty> {
                    match self {
                        #some(x) => x,
                        _ => #none,
                    }
                }
            }
        });
    }

    // flatten on #name<Option<#some_ty>>
    if is_generic {
        let doc = format!(
            "Flattens a `{name}<Option<{ty}>>` into a single `{name}<{ty}>`. Similar to `Option::flatten`.",
            name = name, ty = some_ty_name,
        );
        let opt_where = container.where_clause_for(quote! {#opt<#some_ty>});
        additional_impls.extend(quote! {
            #[automatically_derived]
            #imp #name<#opt<#some_ty>> #opt_where {
                #[doc = #doc]
                pub fn flatten(self) -> #name<#some_ty> {
                    match self {
                        #some(x) => x.into(),
                        _ => #none,
                    }
                }
            }
        });
    }
}
