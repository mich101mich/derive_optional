use super::*;

pub(crate) fn add_external(container: &DataContainer, additional_impls: &mut TokenStream) {
    let DataContainer {
        ref full_name,
        ref some_ident,
        ref none_ident,
        ref none_pattern,
        ref some_ty,
        ref imp,
        ref wheres,
        ref where_clause,
        ..
    } = *container;

    let some_x = container.some(quote! {x});
    let some_y = container.some(quote! {y});

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
        let value = container.some(quote! {x});
        additional_impls.extend(quote! {
            #imp ::std::convert::From<::std::option::Option<#some_ty>> for #full_name #wheres {
                fn from(src: ::std::option::Option<#some_ty>) -> Self {
                    match src {
                        ::std::option::Option::Some(x) => #value,
                        _ => #none_pattern,
                    }
                }
            }
        });
    }

    // Option: From<Self>
    {
        let pattern = container.some(quote! {x});
        additional_impls.extend(quote! {
            #imp ::std::convert::From<#full_name> for ::std::option::Option<#some_ty> #wheres {
                fn from(src: #full_name) -> Self {
                    match src {
                        #pattern => ::std::option::Option::Some(x),
                        _ => ::std::option::Option::None,
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
                    ::std::option::Option::from(self).into_iter()
                }
            }
        });
    }

    // Self: PartialEq<Self>
    {
        let doc = format!(
            "This method tests for `self` and `other` values to be equal, and is used by `==`. Equivalent to `Option::eq`.",
        );
        additional_impls.extend(quote! {
            #imp ::std::cmp::PartialEq<#full_name> for #full_name
            #where_clause
                #some_ty: ::std::cmp::PartialEq,
            {
                #[doc = #doc]
                fn eq(&self, other: &#full_name) -> bool {
                    match (self, other) {
                        (#some_x, #some_y) => ::std::cmp::PartialEq::eq(x, y),
                        (#none_pattern, #none_pattern) => true,
                        _ => false,
                    }
                }
            }
        });
    }

    // Self: std::ops::Try
    // unstable
}
