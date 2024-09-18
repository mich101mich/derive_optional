use super::*;

pub(crate) fn add_external(container: &DataContainer, additional_impls: &mut TokenStream) {
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

    // Self: From<#some_ty>
    {
        let doc = format!("Moves the value into a `{}`.", some_ident,);
        additional_impls.extend(quote! {
            #imp ::std::convert::From<#some_ty> for #full_name #wheres {
                #[doc = #doc]
                fn from(x: #some_ty) -> Self {
                    #some_x
                }
            }
        });
    }

    // Self: From<Option>
    {
        additional_impls.extend(quote! {
            #imp ::std::convert::From<#opt<#some_ty>> for #full_name #wheres {
                fn from(src: #opt<#some_ty>) -> Self {
                    match src {
                        #opt::Some(x) => #some_x,
                        _ => #none_pattern,
                    }
                }
            }
        });
    }

    // Option: From<Self>
    {
        additional_impls.extend(quote! {
            #imp ::std::convert::From<#full_name> for #opt<#some_ty> #wheres {
                fn from(src: #full_name) -> Self {
                    match src {
                        #some_x => #opt::Some(x),
                        _ => #opt::None,
                    }
                }
            }
        });
    }

    // Self: Default
    {
        let doc = format!(
            "Returns a `{}` value. Equivalent to `Option::default`.",
            none_ident,
        );
        additional_impls.extend(quote! {
            #imp ::std::default::Default for #full_name #wheres {
                #[doc = #doc]
                fn default() -> Self {
                    #none_pattern
                }
            }
        });
    }

    // Self: IntoIterator
    {
        let doc = format!(
            "Returns an iterator over the possibly contained value. Equivalent to `Option::into_iter`.",
        );
        additional_impls.extend(quote! {
            #imp ::std::iter::IntoIterator for #full_name #wheres {
                type Item = #some_ty;
                type IntoIter = ::std::option::IntoIter<#some_ty>;

                #[doc = #doc]
                fn into_iter(self) -> Self::IntoIter {
                    #opt::from(self).into_iter()
                }
            }
        });
    }

    // Self: std::ops::Try
    // unstable
}
