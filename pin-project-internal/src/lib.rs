static WASM: &[u8] = include_bytes!("pin-project-internal.wasm");
static MACRO: watt::WasmMacro = watt::WasmMacro::new(WASM);
#[doc = " An attribute that creates projection types covering all the fields of"]
#[doc = " struct or enum."]
#[doc = ""]
#[doc = " This attribute creates projection types according to the following rules:"]
#[doc = ""]
#[doc = " * For the fields that use `#[pin]` attribute, create the pinned reference to"]
#[doc = "   the field."]
#[doc = " * For the other fields, create a normal reference to the field."]
#[doc = ""]
#[doc = " And the following methods are implemented on the original type:"]
#[doc = ""]
#[doc = " ```rust"]
#[doc = " # use std::pin::Pin;"]
#[doc = " # type Projection<'a> = &'a ();"]
#[doc = " # type ProjectionRef<'a> = &'a ();"]
#[doc = " # trait Dox {"]
#[doc = " fn project(self: Pin<&mut Self>) -> Projection<'_>;"]
#[doc = " fn project_ref(self: Pin<&Self>) -> ProjectionRef<'_>;"]
#[doc = " # }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " By passing an argument with the same name as the method to the attribute,"]
#[doc = " you can name the projection type returned from the method:"]
#[doc = ""]
#[doc = " ```rust"]
#[doc = " use pin_project::pin_project;"]
#[doc = " use std::pin::Pin;"]
#[doc = ""]
#[doc = " #[pin_project(project = StructProj)]"]
#[doc = " struct Struct<T> {"]
#[doc = "     #[pin]"]
#[doc = "     field: T,"]
#[doc = " }"]
#[doc = ""]
#[doc = " impl<T> Struct<T> {"]
#[doc = "     fn method(self: Pin<&mut Self>) {"]
#[doc = "         let this: StructProj<'_, T> = self.project();"]
#[doc = "         let StructProj { field } = this;"]
#[doc = "         let _: Pin<&mut T> = field;"]
#[doc = "     }"]
#[doc = " }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " Note that the projection types returned by `project` and `project_ref` have"]
#[doc = " an additional lifetime at the beginning of generics."]
#[doc = ""]
#[doc = " The visibility of the projected type and projection method is based on the"]
#[doc = " original type. However, if the visibility of the original type is `pub`, the"]
#[doc = " visibility of the projected type and the projection method is downgraded to"]
#[doc = " `pub(crate)`."]
#[doc = ""]
#[doc = " # Safety"]
#[doc = ""]
#[doc = " This attribute is completely safe. In the absence of other `unsafe` code"]
#[doc = " *that you write*, it is impossible to cause [undefined"]
#[doc = " behavior][undefined-behavior] with this attribute."]
#[doc = ""]
#[doc = " This is accomplished by enforcing the four requirements for pin projection"]
#[doc = " stated in [the Rust documentation][pin-projection]:"]
#[doc = ""]
#[doc = " 1. The struct must only be [`Unpin`] if all the structural fields are"]
#[doc = "    [`Unpin`]."]
#[doc = ""]
#[doc = "    To enforce this, this attribute will automatically generate an [`Unpin`]"]
#[doc = "    implementation for you, which will require that all structurally pinned"]
#[doc = "    fields be [`Unpin`]."]
#[doc = ""]
#[doc = "    If you attempt to provide an [`Unpin`] impl, the blanket impl will then"]
#[doc = "    apply to your type, causing a compile-time error due to the conflict with"]
#[doc = "    the second impl."]
#[doc = ""]
#[doc = "    If you wish to provide a manual [`Unpin`] impl, you can do so via the"]
#[doc = "    [`UnsafeUnpin`][unsafe-unpin] argument."]
#[doc = ""]
#[doc = " 2. The destructor of the struct must not move structural fields out of its"]
#[doc = "    argument."]
#[doc = ""]
#[doc = "    To enforce this, this attribute will generate code like this:"]
#[doc = ""]
#[doc = "    ```rust"]
#[doc = "    struct MyStruct {}"]
#[doc = "    trait MyStructMustNotImplDrop {}"]
#[doc = "    impl<T: Drop> MyStructMustNotImplDrop for T {}"]
#[doc = "    impl MyStructMustNotImplDrop for MyStruct {}"]
#[doc = "    ```"]
#[doc = ""]
#[doc = "    If you attempt to provide an [`Drop`] impl, the blanket impl will then"]
#[doc = "    apply to your type, causing a compile-time error due to the conflict with"]
#[doc = "    the second impl."]
#[doc = ""]
#[doc = "    If you wish to provide a custom [`Drop`] impl, you can annotate an impl"]
#[doc = "    with [`#[pinned_drop]`][pinned-drop]. This impl takes a pinned version of"]
#[doc = "    your struct - that is, [`Pin`]`<&mut MyStruct>` where `MyStruct` is the"]
#[doc = "    type of your struct."]
#[doc = ""]
#[doc = "    You can call `project()` on this type as usual, along with any other"]
#[doc = "    methods you have defined. Because your code is never provided with"]
#[doc = "    a `&mut MyStruct`, it is impossible to move out of pin-projectable"]
#[doc = "    fields in safe code in your destructor."]
#[doc = ""]
#[doc = " 3. You must make sure that you uphold the [`Drop`"]
#[doc = "    guarantee][drop-guarantee]: once your struct is pinned, the memory that"]
#[doc = "    contains the content is not overwritten or deallocated without calling"]
#[doc = "    the content's destructors."]
#[doc = ""]
#[doc = "    Safe code doesn't need to worry about this - the only way to violate"]
#[doc = "    this requirement is to manually deallocate memory (which is `unsafe`),"]
#[doc = "    or to overwrite a field with something else."]
#[doc = "    Because your custom destructor takes [`Pin`]`<&mut MyStruct>`, it's"]
#[doc = "    impossible to obtain a mutable reference to a pin-projected field in safe"]
#[doc = "    code."]
#[doc = ""]
#[doc = " 4. You must not offer any other operations that could lead to data being"]
#[doc = "    moved out of the structural fields when your type is pinned."]
#[doc = ""]
#[doc = "    As with requirement 3, it is impossible for safe code to violate this."]
#[doc = "    This crate ensures that safe code can never obtain a mutable reference to"]
#[doc = "    `#[pin]` fields, which prevents you from ever moving out of them in safe"]
#[doc = "    code."]
#[doc = ""]
#[doc = " Pin projections are also incompatible with [`#[repr(packed)]`][repr-packed]"]
#[doc = " structs. Attempting to use this attribute on a"]
#[doc = " [`#[repr(packed)]`][repr-packed] struct results in a compile-time error."]
#[doc = ""]
#[doc = " # Examples"]
#[doc = ""]
#[doc = " `#[pin_project]` can be used on structs and enums."]
#[doc = ""]
#[doc = " ```rust"]
#[doc = " use pin_project::pin_project;"]
#[doc = " use std::pin::Pin;"]
#[doc = ""]
#[doc = " #[pin_project]"]
#[doc = " struct Struct<T, U> {"]
#[doc = "     #[pin]"]
#[doc = "     pinned: T,"]
#[doc = "     unpinned: U,"]
#[doc = " }"]
#[doc = ""]
#[doc = " impl<T, U> Struct<T, U> {"]
#[doc = "     fn method(self: Pin<&mut Self>) {"]
#[doc = "         let this = self.project();"]
#[doc = "         let _: Pin<&mut T> = this.pinned;"]
#[doc = "         let _: &mut U = this.unpinned;"]
#[doc = "     }"]
#[doc = " }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " ```rust"]
#[doc = " use pin_project::pin_project;"]
#[doc = " use std::pin::Pin;"]
#[doc = ""]
#[doc = " #[pin_project]"]
#[doc = " struct TupleStruct<T, U>(#[pin] T, U);"]
#[doc = ""]
#[doc = " impl<T, U> TupleStruct<T, U> {"]
#[doc = "     fn method(self: Pin<&mut Self>) {"]
#[doc = "         let this = self.project();"]
#[doc = "         let _: Pin<&mut T> = this.0;"]
#[doc = "         let _: &mut U = this.1;"]
#[doc = "     }"]
#[doc = " }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " To use `#[pin_project]` on enums, you need to name the projection type"]
#[doc = " returned from the method."]
#[doc = ""]
#[doc = " ```rust"]
#[doc = " use pin_project::pin_project;"]
#[doc = " use std::pin::Pin;"]
#[doc = ""]
#[doc = " #[pin_project(project = EnumProj)]"]
#[doc = " enum Enum<T, U> {"]
#[doc = "     Tuple(#[pin] T),"]
#[doc = "     Struct { field: U },"]
#[doc = "     Unit,"]
#[doc = " }"]
#[doc = ""]
#[doc = " impl<T, U> Enum<T, U> {"]
#[doc = "     fn method(self: Pin<&mut Self>) {"]
#[doc = "         match self.project() {"]
#[doc = "             EnumProj::Tuple(x) => {"]
#[doc = "                 let _: Pin<&mut T> = x;"]
#[doc = "             }"]
#[doc = "             EnumProj::Struct { field } => {"]
#[doc = "                 let _: &mut U = field;"]
#[doc = "             }"]
#[doc = "             EnumProj::Unit => {}"]
#[doc = "         }"]
#[doc = "     }"]
#[doc = " }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " If you want to call the `project()` method multiple times or later use the"]
#[doc = " original [`Pin`] type, it needs to use [`.as_mut()`][`Pin::as_mut`] to avoid"]
#[doc = " consuming the [`Pin`]."]
#[doc = ""]
#[doc = " ```rust"]
#[doc = " use pin_project::pin_project;"]
#[doc = " use std::pin::Pin;"]
#[doc = ""]
#[doc = " #[pin_project]"]
#[doc = " struct Struct<T> {"]
#[doc = "     #[pin]"]
#[doc = "     field: T,"]
#[doc = " }"]
#[doc = ""]
#[doc = " impl<T> Struct<T> {"]
#[doc = "     fn call_project_twice(mut self: Pin<&mut Self>) {"]
#[doc = "         // `project` consumes `self`, so reborrow the `Pin<&mut Self>` via `as_mut`."]
#[doc = "         self.as_mut().project();"]
#[doc = "         self.as_mut().project();"]
#[doc = "     }"]
#[doc = " }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " # `!Unpin`"]
#[doc = ""]
#[doc = " If you want to ensure that [`Unpin`] is not implemented, use the `!Unpin`"]
#[doc = " argument to `#[pin_project]`."]
#[doc = ""]
#[doc = " ```rust"]
#[doc = " use pin_project::pin_project;"]
#[doc = ""]
#[doc = " #[pin_project(!Unpin)]"]
#[doc = " struct Struct<T> {"]
#[doc = "     field: T,"]
#[doc = " }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " This is equivalent to using `#[pin]` attribute for the [`PhantomPinned`]"]
#[doc = " field."]
#[doc = ""]
#[doc = " ```rust"]
#[doc = " use pin_project::pin_project;"]
#[doc = " use std::marker::PhantomPinned;"]
#[doc = ""]
#[doc = " #[pin_project]"]
#[doc = " struct Struct<T> {"]
#[doc = "     field: T,"]
#[doc = "     #[pin]"]
#[doc = "     _pin: PhantomPinned,"]
#[doc = " }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " Note that using [`PhantomPinned`] without `#[pin]` attribute has no effect."]
#[doc = ""]
#[doc = " # `UnsafeUnpin`"]
#[doc = ""]
#[doc = " If you want to implement [`Unpin`] manually, you must use the `UnsafeUnpin`"]
#[doc = " argument to `#[pin_project]`."]
#[doc = ""]
#[doc = " ```rust"]
#[doc = " use pin_project::{pin_project, UnsafeUnpin};"]
#[doc = ""]
#[doc = " #[pin_project(UnsafeUnpin)]"]
#[doc = " struct Struct<T, U> {"]
#[doc = "     #[pin]"]
#[doc = "     pinned: T,"]
#[doc = "     unpinned: U,"]
#[doc = " }"]
#[doc = ""]
#[doc = " unsafe impl<T: Unpin, U> UnsafeUnpin for Struct<T, U> {}"]
#[doc = " ```"]
#[doc = ""]
#[doc = " Note the usage of the unsafe [`UnsafeUnpin`] trait, instead of the usual"]
#[doc = " [`Unpin`] trait. [`UnsafeUnpin`] behaves exactly like [`Unpin`], except that"]
#[doc = " is unsafe to implement. This unsafety comes from the fact that pin"]
#[doc = " projections are being used. If you implement [`UnsafeUnpin`], you must"]
#[doc = " ensure that it is only implemented when all pin-projected fields implement"]
#[doc = " [`Unpin`]."]
#[doc = ""]
#[doc = " See [`UnsafeUnpin`] trait for more details."]
#[doc = ""]
#[doc = " # `#[pinned_drop]`"]
#[doc = ""]
#[doc = " In order to correctly implement pin projections, a type's [`Drop`] impl must"]
#[doc = " not move out of any structurally pinned fields. Unfortunately,"]
#[doc = " [`Drop::drop`] takes `&mut Self`, not [`Pin`]`<&mut Self>`."]
#[doc = ""]
#[doc = " To ensure that this requirement is upheld, the `#[pin_project]` attribute"]
#[doc = " will provide a [`Drop`] impl for you. This [`Drop`] impl will delegate to"]
#[doc = " an impl block annotated with `#[pinned_drop]` if you use the `PinnedDrop`"]
#[doc = " argument to `#[pin_project]`."]
#[doc = ""]
#[doc = " This impl block acts just like a normal [`Drop`] impl,"]
#[doc = " except for the following two:"]
#[doc = ""]
#[doc = " * `drop` method takes [`Pin`]`<&mut Self>`"]
#[doc = " * Name of the trait is `PinnedDrop`."]
#[doc = ""]
#[doc = " ```rust"]
#[doc = " # use std::pin::Pin;"]
#[doc = " pub trait PinnedDrop {"]
#[doc = "     fn drop(self: Pin<&mut Self>);"]
#[doc = " }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " `#[pin_project]` implements the actual [`Drop`] trait via `PinnedDrop` you"]
#[doc = " implemented. To drop a type that implements `PinnedDrop`, use the [`drop`]"]
#[doc = " function just like dropping a type that directly implements [`Drop`]."]
#[doc = ""]
#[doc = " In particular, it will never be called more than once, just like"]
#[doc = " [`Drop::drop`]."]
#[doc = ""]
#[doc = " For example:"]
#[doc = ""]
#[doc = " ```rust"]
#[doc = " use pin_project::{pin_project, pinned_drop};"]
#[doc = " use std::{fmt::Debug, pin::Pin};"]
#[doc = ""]
#[doc = " #[pin_project(PinnedDrop)]"]
#[doc = " struct Struct<T: Debug, U: Debug> {"]
#[doc = "     #[pin]"]
#[doc = "     pinned_field: T,"]
#[doc = "     unpin_field: U,"]
#[doc = " }"]
#[doc = ""]
#[doc = " #[pinned_drop]"]
#[doc = " impl<T: Debug, U: Debug> PinnedDrop for Struct<T, U> {"]
#[doc = "     fn drop(self: Pin<&mut Self>) {"]
#[doc = "         println!(\"Dropping pinned field: {:?}\", self.pinned_field);"]
#[doc = "         println!(\"Dropping unpin field: {:?}\", self.unpin_field);"]
#[doc = "     }"]
#[doc = " }"]
#[doc = ""]
#[doc = " fn main() {"]
#[doc = "     let _x = Struct { pinned_field: true, unpin_field: 40 };"]
#[doc = " }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " See also [`#[pinned_drop]`][`pinned_drop`] attribute."]
#[doc = ""]
#[doc = " # `project_replace()`"]
#[doc = ""]
#[doc = " In addition to the `project()` and `project_ref()` methods which are always"]
#[doc = " provided when you use the `#[pin_project]` attribute, there is a third"]
#[doc = " method, `project_replace()` which can be useful in some situations. It is"]
#[doc = " equivalent to [`Pin::set`], except that the unpinned fields are moved and"]
#[doc = " returned, instead of being dropped in-place."]
#[doc = ""]
#[doc = " ```rust"]
#[doc = " # use std::pin::Pin;"]
#[doc = " # type ProjectionOwned = ();"]
#[doc = " # trait Dox {"]
#[doc = " fn project_replace(self: Pin<&mut Self>, other: Self) -> ProjectionOwned;"]
#[doc = " # }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " The `ProjectionOwned` type is identical to the `Self` type, except that"]
#[doc = " all pinned fields have been replaced by equivalent [`PhantomData`] types."]
#[doc = ""]
#[doc = " This method is opt-in, because it is only supported for [`Sized`] types, and"]
#[doc = " because it is incompatible with the [`#[pinned_drop]`][pinned-drop]"]
#[doc = " attribute described above. It can be enabled by using"]
#[doc = " `#[pin_project(project_replace)]`."]
#[doc = ""]
#[doc = " For example:"]
#[doc = ""]
#[doc = " ```rust"]
#[doc = " use pin_project::pin_project;"]
#[doc = " use std::{marker::PhantomData, pin::Pin};"]
#[doc = ""]
#[doc = " #[pin_project(project_replace)]"]
#[doc = " struct Struct<T, U> {"]
#[doc = "     #[pin]"]
#[doc = "     pinned_field: T,"]
#[doc = "     unpinned_field: U,"]
#[doc = " }"]
#[doc = ""]
#[doc = " impl<T, U> Struct<T, U> {"]
#[doc = "     fn method(self: Pin<&mut Self>, other: Self) {"]
#[doc = "         let this = self.project_replace(other);"]
#[doc = "         let _: U = this.unpinned_field;"]
#[doc = "         let _: PhantomData<T> = this.pinned_field;"]
#[doc = "     }"]
#[doc = " }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " By passing the value to the `project_replace` argument, you can name the"]
#[doc = " returned type of `project_replace()`. This is necessary whenever"]
#[doc = " destructuring the return type of `project_replace()`, and work in exactly"]
#[doc = " the same way as the `project` and `project_ref` arguments."]
#[doc = ""]
#[doc = " ```rust"]
#[doc = " use pin_project::pin_project;"]
#[doc = ""]
#[doc = " #[pin_project(project_replace = EnumProjOwn)]"]
#[doc = " enum Enum<T, U> {"]
#[doc = "     A {"]
#[doc = "         #[pin]"]
#[doc = "         pinned_field: T,"]
#[doc = "         unpinned_field: U,"]
#[doc = "     },"]
#[doc = "     B,"]
#[doc = " }"]
#[doc = ""]
#[doc = " let mut x = Box::pin(Enum::A { pinned_field: 42, unpinned_field: \"hello\" });"]
#[doc = ""]
#[doc = " match x.as_mut().project_replace(Enum::B) {"]
#[doc = "     EnumProjOwn::A { unpinned_field, .. } => assert_eq!(unpinned_field, \"hello\"),"]
#[doc = "     EnumProjOwn::B => unreachable!(),"]
#[doc = " }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " [`PhantomData`]: core::marker::PhantomData"]
#[doc = " [`PhantomPinned`]: core::marker::PhantomPinned"]
#[doc = " [`Pin::as_mut`]: core::pin::Pin::as_mut"]
#[doc = " [`Pin::set`]: core::pin::Pin::set"]
#[doc = " [`Pin`]: core::pin::Pin"]
#[doc = " [`UnsafeUnpin`]: https://docs.rs/pin-project/0.4/pin_project/trait.UnsafeUnpin.html"]
#[doc = " [`pinned_drop`]: ./attr.pinned_drop.html"]
#[doc = " [drop-guarantee]: https://doc.rust-lang.org/nightly/std/pin/index.html#drop-guarantee"]
#[doc = " [pin-projection]: https://doc.rust-lang.org/nightly/std/pin/index.html#projections-and-structural-pinning"]
#[doc = " [pinned-drop]: ./attr.pin_project.html#pinned_drop"]
#[doc = " [repr-packed]: https://doc.rust-lang.org/nomicon/other-reprs.html#reprpacked"]
#[doc = " [undefined-behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html"]
#[doc = " [unsafe-unpin]: ./attr.pin_project.html#unsafeunpin"]
#[proc_macro_attribute]
pub fn pin_project(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(pin_project), args, input)
}
#[doc = " An attribute for annotating an impl block that implements [`Drop`]."]
#[doc = ""]
#[doc = " This attribute is only needed when you wish to provide a [`Drop`]"]
#[doc = " impl for your type."]
#[doc = ""]
#[doc = " This impl block acts just like a normal [`Drop`] impl,"]
#[doc = " except for the following two:"]
#[doc = ""]
#[doc = " * `drop` method takes [`Pin`]`<&mut Self>`"]
#[doc = " * Name of the trait is `PinnedDrop`."]
#[doc = ""]
#[doc = " ```rust"]
#[doc = " # use std::pin::Pin;"]
#[doc = " pub trait PinnedDrop {"]
#[doc = "     fn drop(self: Pin<&mut Self>);"]
#[doc = " }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " `#[pin_project]` implements the actual [`Drop`] trait via `PinnedDrop` you"]
#[doc = " implemented. To drop a type that implements `PinnedDrop`, use the [`drop`]"]
#[doc = " function just like dropping a type that directly implements [`Drop`]."]
#[doc = ""]
#[doc = " In particular, it will never be called more than once, just like"]
#[doc = " [`Drop::drop`]."]
#[doc = ""]
#[doc = " # Example"]
#[doc = ""]
#[doc = " ```rust"]
#[doc = " use pin_project::{pin_project, pinned_drop};"]
#[doc = " use std::pin::Pin;"]
#[doc = ""]
#[doc = " #[pin_project(PinnedDrop)]"]
#[doc = " struct Foo {"]
#[doc = "     #[pin]"]
#[doc = "     field: u8,"]
#[doc = " }"]
#[doc = ""]
#[doc = " #[pinned_drop]"]
#[doc = " impl PinnedDrop for Foo {"]
#[doc = "     fn drop(self: Pin<&mut Self>) {"]
#[doc = "         println!(\"Dropping: {}\", self.field);"]
#[doc = "     }"]
#[doc = " }"]
#[doc = ""]
#[doc = " fn main() {"]
#[doc = "     let _x = Foo { field: 50 };"]
#[doc = " }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " See also [\"pinned-drop\" section of `#[pin_project]` attribute][pinned-drop]."]
#[doc = ""]
#[doc = " # Why `#[pinned_drop]` attribute is needed?"]
#[doc = ""]
#[doc = " Implementing `PinnedDrop::drop` is safe, but calling it is not safe."]
#[doc = " [double dropping is unsound](https://github.com/rust-lang/rust/pull/62360)."]
#[doc = ""]
#[doc = " Ideally, it would be desirable to be able to forbid manual calls in"]
#[doc = " the same way as [`Drop::drop`], but the library cannot do it. So, by using"]
#[doc = " macros and replacing them with private traits like the following, we prevent"]
#[doc = " users from calling `PinnedDrop::drop` in safe code."]
#[doc = ""]
#[doc = " ```rust"]
#[doc = " # use std::pin::Pin;"]
#[doc = " pub trait PinnedDrop {"]
#[doc = "     unsafe fn drop(self: Pin<&mut Self>);"]
#[doc = " }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " This allows implementing [`Drop`] safely using `#[pinned_drop]`."]
#[doc = " Also by using the [`drop`] function just like dropping a type that directly"]
#[doc = " implements [`Drop`], can drop safely a type that implements `PinnedDrop`."]
#[doc = ""]
#[doc = " [`Pin`]: core::pin::Pin"]
#[doc = " [pinned-drop]: ./attr.pin_project.html#pinned_drop"]
#[proc_macro_attribute]
pub fn pinned_drop(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(pinned_drop), args, input)
}
#[doc = " (deprecated) An attribute to provide way to refer to the projected type returned by"]
#[doc = " `project` method."]
#[doc = ""]
#[doc = " **This attribute is deprecated. Consider naming projected type by passing"]
#[doc = " `project` argument to `#[pin_project]` attribute instead, see [release note]"]
#[doc = " for details**"]
#[doc = ""]
#[doc = " The following syntaxes are supported."]
#[doc = ""]
#[doc = " # `let` bindings"]
#[doc = ""]
#[doc = " *The attribute at the expression position is not stable, so you need to use"]
#[doc = " a dummy `#[project]` attribute for the function.*"]
#[doc = ""]
#[doc = " ## Examples"]
#[doc = ""]
#[doc = " ```rust"]
#[doc = " # #![allow(deprecated)]"]
#[doc = " use pin_project::{pin_project, project};"]
#[doc = " use std::pin::Pin;"]
#[doc = ""]
#[doc = " #[pin_project]"]
#[doc = " struct Foo<T, U> {"]
#[doc = "     #[pin]"]
#[doc = "     future: T,"]
#[doc = "     field: U,"]
#[doc = " }"]
#[doc = ""]
#[doc = " impl<T, U> Foo<T, U> {"]
#[doc = "     #[project] // Nightly does not need a dummy attribute to the function."]
#[doc = "     fn baz(self: Pin<&mut Self>) {"]
#[doc = "         #[project]"]
#[doc = "         let Foo { future, field } = self.project();"]
#[doc = ""]
#[doc = "         let _: Pin<&mut T> = future;"]
#[doc = "         let _: &mut U = field;"]
#[doc = "     }"]
#[doc = " }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " # `match` expressions"]
#[doc = ""]
#[doc = " *The attribute at the expression position is not stable, so you need to use"]
#[doc = " a dummy `#[project]` attribute for the function.*"]
#[doc = ""]
#[doc = " ## Examples"]
#[doc = ""]
#[doc = " ```rust"]
#[doc = " # #![allow(deprecated)]"]
#[doc = " use pin_project::{pin_project, project};"]
#[doc = " use std::pin::Pin;"]
#[doc = ""]
#[doc = " #[pin_project]"]
#[doc = " enum Enum<A, B, C> {"]
#[doc = "     Tuple(#[pin] A, B),"]
#[doc = "     Struct { field: C },"]
#[doc = "     Unit,"]
#[doc = " }"]
#[doc = ""]
#[doc = " impl<A, B, C> Enum<A, B, C> {"]
#[doc = "     #[project] // Nightly does not need a dummy attribute to the function."]
#[doc = "     fn baz(self: Pin<&mut Self>) {"]
#[doc = "         #[project]"]
#[doc = "         match self.project() {"]
#[doc = "             Enum::Tuple(x, y) => {"]
#[doc = "                 let _: Pin<&mut A> = x;"]
#[doc = "                 let _: &mut B = y;"]
#[doc = "             }"]
#[doc = "             Enum::Struct { field } => {"]
#[doc = "                 let _: &mut C = field;"]
#[doc = "             }"]
#[doc = "             Enum::Unit => {}"]
#[doc = "         }"]
#[doc = "     }"]
#[doc = " }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " # `impl` blocks"]
#[doc = ""]
#[doc = " All methods and associated functions in `#[project] impl` block become"]
#[doc = " methods of the projected type. If you want to implement methods on the"]
#[doc = " original type, you need to create another (non-`#[project]`) `impl` block."]
#[doc = ""]
#[doc = " To call a method implemented in `#[project] impl` block, you need to first"]
#[doc = " get the projected-type with `let this = self.project();`."]
#[doc = ""]
#[doc = " ## Examples"]
#[doc = ""]
#[doc = " ```rust"]
#[doc = " # #![allow(deprecated)]"]
#[doc = " use pin_project::{pin_project, project};"]
#[doc = " use std::pin::Pin;"]
#[doc = ""]
#[doc = " #[pin_project]"]
#[doc = " struct Foo<T, U> {"]
#[doc = "     #[pin]"]
#[doc = "     future: T,"]
#[doc = "     field: U,"]
#[doc = " }"]
#[doc = ""]
#[doc = " // impl for the original type"]
#[doc = " impl<T, U> Foo<T, U> {"]
#[doc = "     fn bar(self: Pin<&mut Self>) {"]
#[doc = "         self.project().baz()"]
#[doc = "     }"]
#[doc = " }"]
#[doc = ""]
#[doc = " // impl for the projected type"]
#[doc = " #[project]"]
#[doc = " impl<T, U> Foo<T, U> {"]
#[doc = "     fn baz(self) {"]
#[doc = "         let Self { future, field } = self;"]
#[doc = ""]
#[doc = "         let _: Pin<&mut T> = future;"]
#[doc = "         let _: &mut U = field;"]
#[doc = "     }"]
#[doc = " }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " # `use` statements"]
#[doc = ""]
#[doc = " ## Examples"]
#[doc = ""]
#[doc = " ```rust"]
#[doc = " # #![allow(deprecated)]"]
#[doc = " # mod dox {"]
#[doc = " use pin_project::pin_project;"]
#[doc = ""]
#[doc = " #[pin_project]"]
#[doc = " struct Foo<A> {"]
#[doc = "     #[pin]"]
#[doc = "     field: A,"]
#[doc = " }"]
#[doc = ""]
#[doc = " mod bar {"]
#[doc = "     use super::Foo;"]
#[doc = "     use pin_project::project;"]
#[doc = "     use std::pin::Pin;"]
#[doc = ""]
#[doc = "     #[project]"]
#[doc = "     use super::Foo;"]
#[doc = ""]
#[doc = "     #[project]"]
#[doc = "     fn baz<A>(foo: Pin<&mut Foo<A>>) {"]
#[doc = "         #[project]"]
#[doc = "         let Foo { field } = foo.project();"]
#[doc = "         let _: Pin<&mut A> = field;"]
#[doc = "     }"]
#[doc = " }"]
#[doc = " # }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " [release note]: https://github.com/taiki-e/pin-project/releases/tag/v0.4.21"]
#[cfg_attr(
    deprecated_proc_macro,
    deprecated(
        since = "0.4.21",
        note = "consider naming projected type by passing `project` \
                argument to #[pin_project] attribute instead, see release note \
                <https://github.com/taiki-e/pin-project/releases/tag/v0.4.21> \
                for details"
    )
)]
#[proc_macro_attribute]
pub fn project(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(project), args, input)
}
#[doc = " (deprecated) An attribute to provide way to refer to the projected type returned by"]
#[doc = " `project_ref` method."]
#[doc = ""]
#[doc = " **This attribute is deprecated. Consider naming projected type by passing"]
#[doc = " `project_ref` argument to `#[pin_project]` attribute instead, see [release note]"]
#[doc = " for details**"]
#[doc = ""]
#[doc = " This is the same as [`#[project]`][`project`] attribute except it refers to"]
#[doc = " the projected type returned by the `project_ref` method."]
#[doc = ""]
#[doc = " See [`#[project]`][`project`] attribute for more details."]
#[doc = ""]
#[doc = " [release note]: https://github.com/taiki-e/pin-project/releases/tag/v0.4.21"]
#[doc = " [`project`]: ./attr.project.html"]
#[cfg_attr(
    deprecated_proc_macro,
    deprecated(
        since = "0.4.21",
        note = "consider naming projected type by passing `project_ref` \
                argument to #[pin_project] attribute instead, see release note \
                <https://github.com/taiki-e/pin-project/releases/tag/v0.4.21> \
                for details"
    )
)]
#[proc_macro_attribute]
pub fn project_ref(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(project_ref), args, input)
}
#[doc = " (deprecated) An attribute to provide way to refer to the projected type returned by"]
#[doc = " `project_replace` method."]
#[doc = ""]
#[doc = " **This attribute is deprecated. Consider naming projected type by passing"]
#[doc = " `project_replace` argument to `#[pin_project]` attribute instead, see [release note]"]
#[doc = " for details**"]
#[doc = ""]
#[doc = " This is the same as [`#[project]`][`project`] attribute except it refers to"]
#[doc = " the projected type returned by the `project_replace` method."]
#[doc = ""]
#[doc = " See [`#[project]`][`project`] attribute for more details."]
#[doc = ""]
#[doc = " [release note]: https://github.com/taiki-e/pin-project/releases/tag/v0.4.21"]
#[doc = " [`project`]: ./attr.project.html"]
#[cfg_attr(
    deprecated_proc_macro,
    deprecated(
        since = "0.4.21",
        note = "consider naming projected type by passing `project_replace` \
                argument to #[pin_project] attribute instead, see release note \
                <https://github.com/taiki-e/pin-project/releases/tag/v0.4.21> \
                for details"
    )
)]
#[proc_macro_attribute]
pub fn project_replace(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(project_replace), args, input)
}
#[doc(hidden)]
#[proc_macro_derive(__PinProjectInternalDerive, attributes(pin))]
pub fn __pin_project_internal_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(__pin_project_internal_derive), input)
}
