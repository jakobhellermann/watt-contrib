static MACRO: watt::WasmMacro = watt::WasmMacro::new(WASM);
static WASM: &[u8] = include_bytes!("tracing-attributes.wasm");
#[doc = " Instruments a function to create and enter a `tracing` [span] every time"]
#[doc = " the function is called."]
#[doc = ""]
#[doc = " The generated span's name will be the name of the function. Any arguments"]
#[doc = " to that function will be recorded as fields using [`fmt::Debug`]. To skip"]
#[doc = " recording a function's or method's argument, pass the argument's name"]
#[doc = " to the `skip` argument on the `#[instrument]` macro. For example,"]
#[doc = " `skip` can be used when an argument to an instrumented function does"]
#[doc = " not implement [`fmt::Debug`], or to exclude an argument with a verbose"]
#[doc = " or costly Debug implementation. Note that:"]
#[doc = " - multiple argument names can be passed to `skip`."]
#[doc = " - arguments passed to `skip` do _not_ need to implement `fmt::Debug`."]
#[doc = ""]
#[doc = " You can also pass additional fields (key-value pairs with arbitrary data)"]
#[doc = " to the generated span. This is achieved using the `fields` argument on the"]
#[doc = " `#[instrument]` macro. You can use a string, integer or boolean literal as"]
#[doc = " a value for each field. The name of the field must be a single valid Rust"]
#[doc = " identifier, nested (dotted) field names are not supported."]
#[doc = ""]
#[doc = " Note that overlap between the names of fields and (non-skipped) arguments"]
#[doc = " will result in a compile error."]
#[doc = ""]
#[doc = " # Examples"]
#[doc = " Instrumenting a function:"]
#[doc = " ```"]
#[doc = " # use tracing_attributes::instrument;"]
#[doc = " #[instrument]"]
#[doc = " pub fn my_function(my_arg: usize) {"]
#[doc = "     // This event will be recorded inside a span named `my_function` with the"]
#[doc = "     // field `my_arg`."]
#[doc = "     tracing::info!(\"inside my_function!\");"]
#[doc = "     // ..."]
#[doc = " }"]
#[doc = " ```"]
#[doc = " Setting the level for the generated span:"]
#[doc = " ```"]
#[doc = " # use tracing_attributes::instrument;"]
#[doc = " #[instrument(level = \"debug\")]"]
#[doc = " pub fn my_function() {"]
#[doc = "     // ..."]
#[doc = " }"]
#[doc = " ```"]
#[doc = " Overriding the generated span's target:"]
#[doc = " ```"]
#[doc = " # use tracing_attributes::instrument;"]
#[doc = " #[instrument(target = \"my_target\")]"]
#[doc = " pub fn my_function() {"]
#[doc = "     // ..."]
#[doc = " }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " To skip recording an argument, pass the argument's name to the `skip`:"]
#[doc = ""]
#[doc = " ```"]
#[doc = " # use tracing_attributes::instrument;"]
#[doc = " struct NonDebug;"]
#[doc = ""]
#[doc = " #[instrument(skip(non_debug))]"]
#[doc = " fn my_function(arg: usize, non_debug: NonDebug) {"]
#[doc = "     // ..."]
#[doc = " }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " To add an additional context to the span, you can pass key-value pairs to `fields`:"]
#[doc = ""]
#[doc = " ```"]
#[doc = " # use tracing_attributes::instrument;"]
#[doc = " #[instrument(fields(foo=\"bar\", id=1, show=true))]"]
#[doc = " fn my_function(arg: usize) {"]
#[doc = "     // ..."]
#[doc = " }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " If the function returns a `Result<T, E>` and `E` implements `std::fmt::Display`, you can add"]
#[doc = " `err` to emit error events when the function returns `Err`:"]
#[doc = ""]
#[doc = " ```"]
#[doc = " # use tracing_attributes::instrument;"]
#[doc = " #[instrument(err)]"]
#[doc = " fn my_function(arg: usize) -> Result<(), std::io::Error> {"]
#[doc = "     Ok(())"]
#[doc = " }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " If `tracing_futures` is specified as a dependency in `Cargo.toml`,"]
#[doc = " `async fn`s may also be instrumented:"]
#[doc = ""]
#[doc = " ```"]
#[doc = " # use tracing_attributes::instrument;"]
#[doc = " #[instrument]"]
#[doc = " pub async fn my_function() -> Result<(), ()> {"]
#[doc = "     // ..."]
#[doc = "     # Ok(())"]
#[doc = " }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " It also works with [async-trait](https://crates.io/crates/async-trait)"]
#[doc = " (a crate that allows async functions on traits,"]
#[doc = " something not currently possible with rustc alone),"]
#[doc = " and hopefully most libraries that exhibit similar behaviors:"]
#[doc = ""]
#[doc = " ```"]
#[doc = " # use tracing::instrument;"]
#[doc = " use async_trait::async_trait;"]
#[doc = ""]
#[doc = " #[async_trait]"]
#[doc = " pub trait Foo {"]
#[doc = "     async fn foo(&self, v: usize) -> ();"]
#[doc = " }"]
#[doc = ""]
#[doc = " #[derive(Debug)]"]
#[doc = " struct FooImpl;"]
#[doc = ""]
#[doc = " #[async_trait]"]
#[doc = " impl Foo for FooImpl {"]
#[doc = "     #[instrument(skip(self))]"]
#[doc = "     async fn foo(&self, v: usize) {}"]
#[doc = " }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " [span]: https://docs.rs/tracing/latest/tracing/span/index.html"]
#[doc = " [`tracing`]: https://github.com/tokio-rs/tracing"]
#[doc = " [`fmt::Debug`]: https://doc.rust-lang.org/std/fmt/trait.Debug.html"]
#[proc_macro_attribute]
pub fn instrument(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(instrument), args, input)
}