static MACRO: watt::WasmMacro = watt::WasmMacro::new(WASM);
static WASM: &[u8] = include_bytes!("tokio-macros.wasm");
#[doc = " Marks async function to be executed by selected runtime."]
#[doc = ""]
#[doc = " ## Options:"]
#[doc = ""]
#[doc = ""]
#[doc = " - `core_threads=n` - Sets core threads to `n` (requires `rt-threaded` feature)."]
#[doc = " - `max_threads=n` - Sets max threads to `n` (requires `rt-core` or `rt-threaded` feature)."]
#[doc = ""]
#[doc = " ## Function arguments:"]
#[doc = ""]
#[doc = " Arguments are allowed for any functions aside from `main` which is special"]
#[doc = ""]
#[doc = " ## Usage"]
#[doc = ""]
#[doc = " ### Using default"]
#[doc = ""]
#[doc = " ```rust"]
#[doc = " #[tokio::main]"]
#[doc = " async fn main() {"]
#[doc = "     println!(\"Hello world\");"]
#[doc = " }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " ### Set number of core threads"]
#[doc = ""]
#[doc = " ```rust"]
#[doc = " #[tokio::main(core_threads = 1)]"]
#[doc = " async fn main() {"]
#[doc = "     println!(\"Hello world\");"]
#[doc = " }"]
#[doc = " ```"]
#[proc_macro_attribute]
#[cfg(not(test))]
pub fn main_threaded(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(main_threaded), args, input)
}
#[doc = " Marks async function to be executed by selected runtime."]
#[doc = ""]
#[doc = " ## Options:"]
#[doc = ""]
#[doc = " - `basic_scheduler` - All tasks are executed on the current thread."]
#[doc = " - `threaded_scheduler` - Uses the multi-threaded scheduler. Used by default (requires `rt-threaded` feature)."]
#[doc = ""]
#[doc = " ## Function arguments:"]
#[doc = ""]
#[doc = " Arguments are allowed for any functions aside from `main` which is special"]
#[doc = ""]
#[doc = " ## Usage"]
#[doc = ""]
#[doc = " ### Using default"]
#[doc = ""]
#[doc = " ```rust"]
#[doc = " #[tokio::main]"]
#[doc = " async fn main() {"]
#[doc = "     println!(\"Hello world\");"]
#[doc = " }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " ### Select runtime"]
#[doc = ""]
#[doc = " ```rust"]
#[doc = " #[tokio::main(basic_scheduler)]"]
#[doc = " async fn main() {"]
#[doc = "     println!(\"Hello world\");"]
#[doc = " }"]
#[doc = " ```"]
#[proc_macro_attribute]
#[cfg(not(test))]
pub fn main(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(main), args, input)
}
#[doc = " Marks async function to be executed by selected runtime."]
#[doc = ""]
#[doc = " ## Options:"]
#[doc = ""]
#[doc = " - `max_threads=n` - Sets max threads to `n`."]
#[doc = ""]
#[doc = " ## Function arguments:"]
#[doc = ""]
#[doc = " Arguments are allowed for any functions aside from `main` which is special"]
#[doc = ""]
#[doc = " ## Usage"]
#[doc = ""]
#[doc = " ### Using default"]
#[doc = ""]
#[doc = " ```rust"]
#[doc = " #[tokio::main]"]
#[doc = " async fn main() {"]
#[doc = "     println!(\"Hello world\");"]
#[doc = " }"]
#[doc = " ```"]
#[proc_macro_attribute]
#[cfg(not(test))]
pub fn main_basic(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(main_basic), args, input)
}
#[doc = " Marks async function to be executed by runtime, suitable to test environment"]
#[doc = ""]
#[doc = " ## Options:"]
#[doc = ""]
#[doc = " - `core_threads=n` - Sets core threads to `n` (requires `rt-threaded` feature)."]
#[doc = " - `max_threads=n` - Sets max threads to `n` (requires `rt-core` or `rt-threaded` feature)."]
#[doc = ""]
#[doc = " ## Usage"]
#[doc = ""]
#[doc = " ### Select runtime"]
#[doc = ""]
#[doc = " ```no_run"]
#[doc = " #[tokio::test(core_threads = 1)]"]
#[doc = " async fn my_test() {"]
#[doc = "     assert!(true);"]
#[doc = " }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " ### Using default"]
#[doc = ""]
#[doc = " ```no_run"]
#[doc = " #[tokio::test]"]
#[doc = " async fn my_test() {"]
#[doc = "     assert!(true);"]
#[doc = " }"]
#[doc = " ```"]
#[proc_macro_attribute]
pub fn test_threaded(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(test_threaded), args, input)
}
#[doc = " Marks async function to be executed by runtime, suitable to test environment"]
#[doc = ""]
#[doc = " ## Options:"]
#[doc = ""]
#[doc = " - `basic_scheduler` - All tasks are executed on the current thread. Used by default."]
#[doc = " - `threaded_scheduler` - Use multi-threaded scheduler (requires `rt-threaded` feature)."]
#[doc = ""]
#[doc = " ## Usage"]
#[doc = ""]
#[doc = " ### Select runtime"]
#[doc = ""]
#[doc = " ```no_run"]
#[doc = " #[tokio::test(threaded_scheduler)]"]
#[doc = " async fn my_test() {"]
#[doc = "     assert!(true);"]
#[doc = " }"]
#[doc = " ```"]
#[doc = ""]
#[doc = " ### Using default"]
#[doc = ""]
#[doc = " ```no_run"]
#[doc = " #[tokio::test]"]
#[doc = " async fn my_test() {"]
#[doc = "     assert!(true);"]
#[doc = " }"]
#[doc = " ```"]
#[proc_macro_attribute]
pub fn test(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(test), args, input)
}
#[doc = " Marks async function to be executed by runtime, suitable to test environment"]
#[doc = ""]
#[doc = " ## Options:"]
#[doc = ""]
#[doc = " - `max_threads=n` - Sets max threads to `n`."]
#[doc = ""]
#[doc = " ## Usage"]
#[doc = ""]
#[doc = " ```no_run"]
#[doc = " #[tokio::test]"]
#[doc = " async fn my_test() {"]
#[doc = "     assert!(true);"]
#[doc = " }"]
#[doc = " ```"]
#[proc_macro_attribute]
pub fn test_basic(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(test_basic), args, input)
}
#[doc = " Implementation detail of the `select!` macro. This macro is **not** intended"]
#[doc = " to be used as part of the public API and is permitted to change."]
#[proc_macro]
#[doc(hidden)]
pub fn select_priv_declare_output_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro(stringify!(select_priv_declare_output_enum), input)
}
