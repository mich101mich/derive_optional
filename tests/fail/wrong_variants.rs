////////// too few variants //////////
#[derive(derive_optional::Optional)]
enum NoVariants {}
//~  ^^^^^^^^^^ Optional needs exactly two variants

#[derive(derive_optional::Optional)]
enum OneVariant {
//~  ^^^^^^^^^^ Optional needs exactly two variants
    Variant,
}

#[derive(derive_optional::Optional)]
enum OneVariantWithData<T> {
//~  ^^^^^^^^^^^^^^^^^^ Optional needs exactly two variants
    Variant(T),
}

////////// too many variants //////////
#[derive(derive_optional::Optional)]
enum TooManyVariants {
    Variant1,
    Variant2,
    Variant3(usize),
    Variant4 { field: usize },
    Variant5,
}

//~~~~~~~~~~~~~~~~~~~~ errors ~~~~~~~~~~~~~~~~~~~~//

// error: Optional only works when there are exactly 2 enum variants
//  --> wrong_variants.rs:5:5
//   |
// 5 |     Variant3(usize),
//   |     ^^^^^^^^^^^^^^^

// error: Optional only works when there are exactly 2 enum variants
//  --> wrong_variants.rs:6:5
//   |
// 6 |     Variant4 { field: usize },
//   |     ^^^^^^^^^^^^^^^^^^^^^^^^^

// error: Optional only works when there are exactly 2 enum variants
//  --> wrong_variants.rs:7:5
//   |
// 7 |     Variant5,
//   |     ^^^^^^^^

////////// only unit variants //////////
#[derive(derive_optional::Optional)]
enum OnlyUnitVariants {
    Variant1,
    Variant2,
}

//~~~~~~~~~~~~~~~~~~~~ errors ~~~~~~~~~~~~~~~~~~~~//

// error: Optional requires exactly one of its variants (the 'some' variant) to contain data
//  --> wrong_variants.rs:3:5
//   |
// 3 |     Variant1,
//   |     ^^^^^^^^

// error: Optional requires exactly one of its variants (the 'some' variant) to contain data
//  --> wrong_variants.rs:4:5
//   |
// 4 |     Variant2,
//   |     ^^^^^^^^

////////// both variants contain data //////////
#[derive(derive_optional::Optional)]
enum OnlyDataVariants<T> {
    Variant1(T),
    Variant2(T),
}

//~~~~~~~~~~~~~~~~~~~~ errors ~~~~~~~~~~~~~~~~~~~~//

// error: Optional requires exactly one of its variants (the 'none' variant) to have no fields
//  --> wrong_variants.rs:3:5
//   |
// 3 |     Variant1(T),
//   |     ^^^^^^^^^^^

// error: Optional requires exactly one of its variants (the 'none' variant) to have no fields
//  --> wrong_variants.rs:4:5
//   |
// 4 |     Variant2(T),
//   |     ^^^^^^^^^^^

////////// named fields on both variants //////////
#[derive(derive_optional::Optional)]
enum OnlyNamedVariants {
    Variant1 { field: usize },
    Variant2 { field: usize },
}

//~~~~~~~~~~~~~~~~~~~~ errors ~~~~~~~~~~~~~~~~~~~~//

// error: Optional needs one unit variant (the 'none' variant) and one variant with data (the 'some' variant)
//  --> wrong_variants.rs:3:5
//   |
// 3 |     Variant1 { field: usize },
//   |     ^^^^^^^^^^^^^^^^^^^^^^^^^

// error: Optional needs one unit variant (the 'none' variant) and one variant with data (the 'some' variant)
//  --> wrong_variants.rs:4:5
//   |
// 4 |     Variant2 { field: usize },
//   |     ^^^^^^^^^^^^^^^^^^^^^^^^^

////////// named fields on some variants //////////
#[derive(derive_optional::Optional)]
enum NamedDataVariant<T> {
    Variant1 { field: T },
    //~      ^^^^^^^^^^^^ Optional currently does not support named fields in the 'some' variant, only tuple variants.
    //~                   Hint: Change this variant to for example `Variant1(T)`
    Variant2,
}
#[derive(derive_optional::Optional)]
enum NamedDataVariant2<T> {
    Variant1,
    Variant2 { field: T },
    //~      ^^^^^^^^^^^^ Optional currently does not support named fields in the 'some' variant, only tuple variants.
    //~                   Hint: Change this variant to for example `Variant2(T)`
}

////////// empty named fields //////////
#[derive(derive_optional::Optional)]
enum NamedDataVariantNoFields {
    Variant1 {},
    //~      ^^ Optional currently does not support named fields in the 'some' variant, only tuple variants.
    //~         Hint: Change this variant to for example `Variant1(...)`
    Variant2,
}
#[derive(derive_optional::Optional)]
enum NamedDataVariantNoFields2 {
    Variant1,
    Variant2 {},
    //~      ^^ Optional currently does not support named fields in the 'some' variant, only tuple variants.
    //~         Hint: Change this variant to for example `Variant2(...)`
}

////////// multiple named fields //////////
#[derive(derive_optional::Optional)]
enum NamedDataVariantMultiFields<T> {
    Variant1 { a: T, b: usize, c: T },
    //~      ^^^^^^^^^^^^^^^^^^^^^^^^ Optional currently does not support named fields in the 'some' variant, only tuple variants.
    //~                               Hint: Change this variant to for example `Variant1(NamedDataVariantMultiFieldsVariant1)`
    //~                                     with `struct NamedDataVariantMultiFieldsVariant1 { a: T, b: usize, c: T }`
    Variant2,
}
#[derive(derive_optional::Optional)]
enum NamedDataVariantMultiFields2<T> {
    Variant1,
    Variant2 { a: T, b: usize, c: T },
    //~      ^^^^^^^^^^^^^^^^^^^^^^^^ Optional currently does not support named fields in the 'some' variant, only tuple variants.
    //~                               Hint: Change this variant to for example `Variant2(NamedDataVariantMultiFields2Variant2)`
    //~                                     with `struct NamedDataVariantMultiFields2Variant2 { a: T, b: usize, c: T }`
}

fn main() {}
