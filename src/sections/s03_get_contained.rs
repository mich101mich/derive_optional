use super::*;

pub(crate) fn add_section(container: &DataContainer, impl_block: &mut TokenStream) {
    #[allow(unused_variables)]
    #[rustfmt::skip]
    let DataContainer {
        ref name, ref full_name, ref full_name_string, ref some, ref none, ref some_name, ref none_name,
        ref some_name_snake, ref none_name_snake, ref some_ty, ref some_ty_name, is_generic, ref bounds, ref imp,
        ref func, ref c_func, ref opt
    } = *container;

    /////////////////////////////////////////////////////////////////////////
    // Getting to contained values
    /////////////////////////////////////////////////////////////////////////

    // expect
    {
        let doc = format!(
            "Returns the contained `{some}` value, consuming `self`. Equivalent to `Option::expect`.

# Panics

Panics if the value is a `{none}` with a custom panic message provided by `msg`.",
            some = some_name, none = none_name,
        );
        // can't be c_func right now because of `panic`'s formatting. std uses nightly-only functions from feature(core_panic)
        impl_block.extend(quote! {
            #[doc = #doc]
            #func expect(self, msg: &str) -> #some_ty {
                match self {
                    #some(x) => x,
                    _ => panic!("{}", msg),
                }
            }
        });
    }

    // unwrap
    {
        let msg = format!(
            "called `{name}::unwrap()` on a `{none}` value",
            name = name,
            none = none_name
        );
        let doc = format!(
            "Returns the contained `{some}` value, consuming `self`. Equivalent to `Option::unwrap`.

# Panics

Panics if the value is a `{none}`.",
            some = some_name, none = none_name,
        );
        // can't be c_func right now because of `panic`'s formatting. std uses nightly-only functions from feature(core_panic)
        impl_block.extend(quote! {
            #[doc = #doc]
            #func unwrap(self) -> #some_ty {
                match self {
                    #some(x) => x,
                    _ => panic!("{}", #msg),
                }
            }
        });
    }

    // unwrap_or
    {
        let doc = format!(
            "Returns the contained `{some}` value or a provided default. Equivalent to `Option::unwrap_or`.",
            some = some_name,
        );
        // can't be c_func right now because of destructors (https://github.com/rust-lang/rust/issues/67792)
        impl_block.extend(quote! {
            #[doc = #doc]
            #func unwrap_or(self, default: #some_ty) -> #some_ty {
                match self {
                    #some(x) => x,
                    _ => default,
                }
            }
        });
    }

    // unwrap_or_else
    {
        let doc = format!(
            "Returns the contained `{some}` value or computes it from a closure. Equivalent to `Option::unwrap_or_else`.",
            some = some_name,
        );
        // can't be c_func right now because of destructors (https://github.com/rust-lang/rust/issues/67792)
        impl_block.extend(quote! {
            #[doc = #doc]
            #func unwrap_or_else<F>(self, f: F) -> #some_ty
            where
                F: FnOnce() -> #some_ty,
            {
                match self {
                    #some(x) => x,
                    _ => f(),
                }
            }
        });
    }

    // unwrap_or_default
    {
        let doc = format!(
            "Returns the contained `{some}` value or its default. Equivalent to `Option::unwrap_or_default`.",
            some = some_name,
        );
        // can't be c_func right now because of destructors (https://github.com/rust-lang/rust/issues/67792)
        impl_block.extend(quote! {
            #[doc = #doc]
            #func unwrap_or_default(self) -> #some_ty
            where
                #some_ty: ::std::default::Default,
            {
                match self {
                    #some(x) => x,
                    _ => ::std::default::Default::default(),
                }
            }
        });
    }

    // unwrap_unchecked
    {
        let doc = format!(
            "Returns the contained `{some}` value without checking, consuming `self`. Equivalent to `Option::unwrap_unchecked`.
            
# Safety

The caller must guarantee that the value is a `{some}`. Otherwise, undefined behavior occurs.",
            some = some_name,
        );
        // can't be c_func right now because of destructors (https://github.com/rust-lang/rust/issues/67792)
        impl_block.extend(quote! {
            #[doc = #doc]
            pub unsafe fn unwrap_unchecked(self) -> #some_ty {
                match self {
                    #some(x) => x,
                    // SAFETY: the safety contract must be upheld by the caller.
                    _ => unsafe { ::std::hint::unreachable_unchecked() },
                }
            }
        });
    }
}
