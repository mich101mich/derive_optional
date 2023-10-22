use super::*;

pub(crate) fn add_section(container: &DataContainer, impl_block: &mut TokenStream) {
    let DataContainer {
        ref name,
        ref some_ident,
        ref none_ident,
        ref some_ty,
        ref func,
        ..
    } = *container;

    let some_x = container.some(quote! {x});

    /////////////////////////////////////////////////////////////////////////
    // Getting to contained values
    /////////////////////////////////////////////////////////////////////////

    // expect
    {
        let doc = format!(
            "Returns the contained `{}` value, consuming `self`. Equivalent to `Option::expect`.

# Panics

Panics if the value is a `{}` with a custom panic message provided by `msg`.",
            name, none_ident,
        );
        // can't be c_func right now because of `panic`'s formatting. std uses nightly-only functions from feature(core_panic)
        impl_block.extend(quote! {
            #[doc = #doc]
            #func expect(self, msg: &str) -> #some_ty {
                match self {
                    #some_x => x,
                    _ => panic!("{}", msg),
                }
            }
        });
    }

    // unwrap
    {
        let msg = format!("called `{}::unwrap()` on a `{}` value", name, none_ident);
        let doc = format!(
            "Returns the contained `{}` value, consuming `self`. Equivalent to `Option::unwrap`.

# Panics

Panics if the value is a `{}`.",
            name, none_ident,
        );
        // can't be c_func right now because of `panic`'s formatting. std uses nightly-only functions from feature(core_panic)
        impl_block.extend(quote! {
            #[doc = #doc]
            #func unwrap(self) -> #some_ty {
                match self {
                    #some_x => x,
                    _ => panic!("{}", #msg),
                }
            }
        });
    }

    // unwrap_or
    {
        let doc = format!(
            "Returns the contained `{}` value or a provided default. Equivalent to `Option::unwrap_or`.",
            name,
        );
        // can't be c_func right now because of destructors (https://github.com/rust-lang/rust/issues/67792)
        impl_block.extend(quote! {
            #[doc = #doc]
            #func unwrap_or(self, default: #some_ty) -> #some_ty {
                match self {
                    #some_x => x,
                    _ => default,
                }
            }
        });
    }

    // unwrap_or_else
    {
        let doc = format!(
            "Returns the contained `{}` value or computes it from a closure. Equivalent to `Option::unwrap_or_else`.",
            name,
        );
        // can't be c_func right now because of destructors (https://github.com/rust-lang/rust/issues/67792)
        impl_block.extend(quote! {
            #[doc = #doc]
            #func unwrap_or_else<F>(self, f: F) -> #some_ty
            where
                F: FnOnce() -> #some_ty,
            {
                match self {
                    #some_x => x,
                    _ => f(),
                }
            }
        });
    }

    // unwrap_or_default
    {
        let doc = format!(
            "Returns the contained `{}` value or its default. Equivalent to `Option::unwrap_or_default`.",
            name,
        );
        // can't be c_func right now because of destructors (https://github.com/rust-lang/rust/issues/67792)
        impl_block.extend(quote! {
            #[doc = #doc]
            #func unwrap_or_default(self) -> #some_ty
            where
                #some_ty: ::std::default::Default,
            {
                match self {
                    #some_x => x,
                    _ => ::std::default::Default::default(),
                }
            }
        });
    }

    // unwrap_unchecked
    {
        let doc = format!(
            "Returns the contained `{}` value without checking, consuming `self`. Equivalent to `Option::unwrap_unchecked`.
            
# Safety

The caller must guarantee that the value is a `{}`. Otherwise, undefined behavior occurs.",
            name, some_ident,
        );
        // can't be c_func right now because of destructors (https://github.com/rust-lang/rust/issues/67792)
        impl_block.extend(quote! {
            #[doc = #doc]
            pub unsafe fn unwrap_unchecked(self) -> #some_ty {
                match self {
                    #some_x => x,
                    // SAFETY: the safety contract must be upheld by the caller.
                    _ => unsafe { ::std::hint::unreachable_unchecked() },
                }
            }
        });
    }
}
