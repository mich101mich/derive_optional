use super::*;

pub(crate) fn add_external(container: &DataContainer, additional_impls: &mut TokenStream) {
    #[allow(unused_variables)]
    #[rustfmt::skip]
    let DataContainer {
        ref name, ref full_name, ref full_name_string, ref some, ref none, ref some_name, ref none_name,
        ref some_name_snake, ref none_name_snake, ref some_ty, ref some_ty_name, is_generic, ref bounds, ref imp,
        ref func, ref c_func, ref opt
    } = *container;

    // Self: From<#some_ty>
    {
        let doc = format!("Moves the value into a `{}`.", some_name);
        additional_impls.extend(quote! {
            #[automatically_derived]
            #imp ::std::convert::From<#some_ty> for #full_name {
                #[doc = #doc]
                fn from(x: #some_ty) -> Self {
                    #some(x)
                }
            }
        });
    }

    // Self: From<Option>
    {
        additional_impls.extend(quote! {
            #[automatically_derived]
            #imp ::std::convert::From<#opt<#some_ty>> for #full_name {
                fn from(src: #opt<#some_ty>) -> Self {
                    match src {
                        #opt::Some(x) => #some(x),
                        _ => #none,
                    }
                }
            }
        });
    }

    // Option: From<Self>
    {
        additional_impls.extend(quote! {
            #[automatically_derived]
            #imp ::std::convert::From<#full_name> for #opt<#some_ty> {
                fn from(src: #full_name) -> Self {
                    match src {
                        #some(x) => #opt::Some(x),
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
            none_name,
        );
        // note that this does not require `Default` for `#some_ty`, since the `none` variant is returned
        additional_impls.extend(quote! {
            #[automatically_derived]
            #imp ::std::default::Default for #full_name {
                #[doc = #doc]
                fn default() -> Self {
                    #none
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
            #[automatically_derived]
            #imp ::std::iter::IntoIterator for #full_name {
                type Item = #some_ty;
                type IntoIter = ::std::option::IntoIter<#some_ty>;

                #[doc = #doc]
                fn into_iter(self) -> Self::IntoIter {
                    #opt::<#some_ty>::from(self).into_iter()
                }
            }
        });
    }

    // Self: std::ops::Try
    // unstable
    #[cfg(feature = "try_op")]
    {
        additional_impls.extend(quote! {
            #[automatically_derived]
            #imp ::std::ops::Try for #full_name {
                type Output = #some_ty;
                type Residual = <#opt<#some_ty> as ::std::ops::Try>::Residual;

                #[inline]
                fn from_output(output: Self::Output) -> Self {
                    #some(output)
                }

                #[inline]
                fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
                    match self {
                        #some(x) => std::ops::ControlFlow::Continue(x),
                        #none => std::ops::ControlFlow::Break(#opt::None),
                    }
                }
            }

            #[automatically_derived]
            #imp ::std::ops::FromResidual for #full_name {
                #[inline]
                fn from_residual(_residual: <Self as ::std::ops::Try>::Residual) -> Self {
                    #none
                }
            }
        });
    }
}
