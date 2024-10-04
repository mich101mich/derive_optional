#[derive(derive_optional::Optional)]
enum NoVariants {}

#[derive(derive_optional::Optional)]
enum OneVariant {
    Variant,
}

#[derive(derive_optional::Optional)]
enum OneVariantWithData<T> {
    Variant(T),
}

#[derive(derive_optional::Optional)]
enum TooManyVariants {
    Variant1,
    Variant2,
    Variant3(usize),
    Variant4 { field: usize },
    Variant5,
}

#[derive(derive_optional::Optional)]
enum OnlyUnitVariants {
    Variant1,
    Variant2,
}

#[derive(derive_optional::Optional)]
enum OnlyDataVariants<T> {
    Variant1(T),
    Variant2(T),
}

#[derive(derive_optional::Optional)]
enum OnlyNamedVariants {
    Variant1 { field: usize },
    Variant2 { field: usize },
}

#[derive(derive_optional::Optional)]
enum NamedDataVariant<T> {
    Variant1 { field: T },
    Variant2,
}
#[derive(derive_optional::Optional)]
enum NamedDataVariant2<T> {
    Variant1,
    Variant2 { field: T },
}

#[derive(derive_optional::Optional)]
enum NamedDataVariantNoFields {
    Variant1 {},
    Variant2,
}
#[derive(derive_optional::Optional)]
enum NamedDataVariantNoFields2 {
    Variant1,
    Variant2 {},
}

#[derive(derive_optional::Optional)]
enum NamedDataVariantMultiFields<T> {
    Variant1 { a: T, b: usize, c: T },
    Variant2,
}
#[derive(derive_optional::Optional)]
enum NamedDataVariantMultiFields2<T> {
    Variant1,
    Variant2 { a: T, b: usize, c: T },
}

fn main() {}
