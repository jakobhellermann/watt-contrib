static MACRO: watt::WasmMacro = watt::WasmMacro::new(WASM);
static WASM: &[u8] = include_bytes!("typed-builder.wasm");
#[doc = " `TypedBuilder` is not a real type - deriving it will generate a `::builder()` method on your"]
#[doc = " struct that will return a compile-time checked builder. Set the fields using setters with the"]
#[doc = " same name as the struct's fields that accept `Into` types for the type of the field, and call"]
#[doc = " `.build()` when you are done to create your object."]
#[doc = ""]
#[doc = " Trying to set the same fields twice will generate a compile-time error. Trying to build without"]
#[doc = " setting one of the fields will also generate a compile-time error - unless that field is marked"]
#[doc = " as `#[builder(default)]`, in which case the `::default()` value of it's type will be picked. If"]
#[doc = " you want to set a different default, use `#[builder(default=...)]`."]
#[doc = ""]
#[doc = " # Examples"]
#[doc = ""]
#[doc = " ```"]
#[doc = " #[macro_use]"]
#[doc = " extern crate typed_builder;"]
#[doc = ""]
#[doc = " #[derive(PartialEq, TypedBuilder)]"]
#[doc = " struct Foo {"]
#[doc = "     // Mandatory Field:"]
#[doc = "     x: i32,"]
#[doc = ""]
#[doc = "     // #[default] without parameter - use the type's default"]
#[doc = "     // #[builder(setter(strip_option))] - wrap the setter argument with `Some(...)`"]
#[doc = "     #[builder(default, setter(strip_option))]"]
#[doc = "     y: Option<i32>,"]
#[doc = ""]
#[doc = "     // Or you can set the default"]
#[doc = "     #[builder(default=20)]"]
#[doc = "     z: i32,"]
#[doc = " }"]
#[doc = ""]
#[doc = " fn main() {"]
#[doc = "     assert!("]
#[doc = "         Foo::builder().x(1).y(2).z(3).build()"]
#[doc = "         == Foo { x: 1, y: Some(2), z: 3, });"]
#[doc = ""]
#[doc = "     // Change the order of construction:"]
#[doc = "     assert!("]
#[doc = "         Foo::builder().z(1).x(2).y(3).build()"]
#[doc = "         == Foo { x: 2, y: Some(3), z: 1, });"]
#[doc = ""]
#[doc = "     // Optional fields are optional:"]
#[doc = "     assert!("]
#[doc = "         Foo::builder().x(1).build()"]
#[doc = "         == Foo { x: 1, y: None, z: 20, });"]
#[doc = ""]
#[doc = "     // This will not compile - because we did not set x:"]
#[doc = "     // Foo::builder().build();"]
#[doc = ""]
#[doc = "     // This will not compile - because we set y twice:"]
#[doc = "     // Foo::builder().x(1).y(2).y(3);"]
#[doc = " }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " # Customisation with attributes"]
#[doc = ""]
#[doc = " In addition to putting `#[derive(TypedBuilder)]` on a type, you can specify a `#[builder(…)]`"]
#[doc = " attribute on the type, and on any fields in it."]
#[doc = ""]
#[doc = " On the **type**, the following values are permitted:"]
#[doc = ""]
#[doc = " - `doc`: enable documentation of the builder type. By default, the builder type is given"]
#[doc = "   `#[doc(hidden)]`, so that the `builder()` method will show `FooBuilder` as its return type,"]
#[doc = "   but it won’t be a link. If you turn this on, the builder type and its `build` method will get"]
#[doc = "   sane defaults. The field methods on the builder will be undocumented by default."]
#[doc = ""]
#[doc = " - `builder_method_doc = \"…\"` replaces the default documentation that will be generated for the"]
#[doc = "   `builder()` method of the type for which the builder is being generated."]
#[doc = ""]
#[doc = " - `builder_type_doc = \"…\"` replaces the default documentation that will be generated for the"]
#[doc = "   builder type. Setting this implies `doc`."]
#[doc = ""]
#[doc = " - `build_method_doc = \"…\"` replaces the default documentation that will be generated for the"]
#[doc = "   `build()` method of the builder type. Setting this implies `doc`."]
#[doc = ""]
#[doc = " On each **field**, the following values are permitted:"]
#[doc = ""]
#[doc = " - `default`: make the field optional, defaulting to `Default::default()`. This requires that"]
#[doc = "   the field type implement `Default`. Mutually exclusive with any other form of default."]
#[doc = ""]
#[doc = " - `default = …`: make the field optional, defaulting to the expression `…`."]
#[doc = ""]
#[doc = " - `setter(...)`: settings for the field setters. The following values are permitted inside:"]
#[doc = ""]
#[doc = "   - `doc = \"…\"`: sets the documentation for the field’s setter on the builder type. This will be"]
#[doc = "     of no value unless you enable docs for the builder type with `#[builder(doc)]` or similar on"]
#[doc = "     the type."]
#[doc = ""]
#[doc = "   - `skip`: do not define a method on the builder for this field. This requires that a default"]
#[doc = "     be set."]
#[doc = ""]
#[doc = "   - `into`: automatically convert the argument of the setter method to the type of the field."]
#[doc = "     Note that this conversion interferes with Rust's type inference and integer literal"]
#[doc = "     detection, so this may reduce ergonomics if the field type is generic or an unsigned integer."]
#[doc = ""]
#[doc = "   - `strip_option`: for `Option<...>` fields only, this makes the setter wrap its argument with"]
#[doc = "     `Some(...)`, relieving the caller from having to do this. Note that with this setting on"]
#[doc = "     one cannot set the field to `None` with the setter - so the only way to get it to be `None`"]
#[doc = "     is by using `#[builder(default)]` and not calling the field's setter."]
#[proc_macro_derive(TypedBuilder, attributes(builder))]
pub fn derive_typed_builder(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(derive_typed_builder), input)
}
