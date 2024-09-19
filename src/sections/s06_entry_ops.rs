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
    // Entry-like operations to insert a value and return a reference
    /////////////////////////////////////////////////////////////////////////

    // insert
    {
        let doc = format!(
            "Inserts a value into the `{name}`, then returns a mutable reference to it. Equivalent to `Option::insert`.",
            name = name
        );
        impl_block.extend(quote! {
            #[doc = #doc]
            #func insert(&mut self, x: #some_ty) -> &mut #some_ty {
                *self = #some(x);
                match self {
                    #some(ref mut x) => x,

                    // SAFETY: a value was just inserted.
                    _ => unsafe { ::std::hint::unreachable_unchecked() },
                }
            }
        });
    }

    // get_or_insert
    {
        let doc = format!(
            "Returns a mutable reference to the contained value, inserting the provided value if empty. Equivalent to `Option::get_or_insert`.",
        );
        impl_block.extend(quote! {
            #[doc = #doc]
            #func get_or_insert(&mut self, value: #some_ty) -> &mut #some_ty {
                match self {
                    #some(ref mut x) => x,
                    _ => self.insert(value),
                }
            }
        });
    }

    // get_or_insert_default
    {
        let doc = format!(
            "Returns a mutable reference to the contained value, inserting the default value if empty. Equivalent to `Option::get_or_insert_default`.",
        );
        impl_block.extend(quote! {
            #[doc = #doc]
            #func get_or_insert_default(&mut self) -> &mut #some_ty
            where
                #some_ty: ::std::default::Default,
            {
                self.get_or_insert_with(::std::default::Default::default)
            }
        });
    }

    // get_or_insert_with
    {
        let doc = format!(
            "Returns a mutable reference to the contained value, inserting the result of the provided function if empty. Equivalent to `Option::get_or_insert_with`.",
        );
        impl_block.extend(quote! {
            #[doc = #doc]
            #func get_or_insert_with<F>(&mut self, f: F) -> &mut #some_ty
            where
                F: FnOnce() -> #some_ty,
            {
                match self {
                    #some(ref mut x) => x,
                    _ => self.insert(f()),
                }
            }
        });
    }
}
