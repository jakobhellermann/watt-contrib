static MACRO: watt::WasmMacro = watt::WasmMacro::new(WASM);
static WASM: &[u8] = include_bytes!("async-attributes.wasm");
#[doc = " Enables an async main function."]
#[doc = ""]
#[doc = " # Examples"]
#[doc = ""]
#[doc = " ```ignore"]
#[doc = " #[async_std::main]"]
#[doc = " async fn main() -> std::io::Result<()> {"]
#[doc = "     Ok(())"]
#[doc = " }"]
#[doc = " ```"]
#[cfg(not(test))]
#[proc_macro_attribute]
pub fn main(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(main), args, input)
}
#[doc = " Enables an async test function."]
#[doc = ""]
#[doc = " # Examples"]
#[doc = ""]
#[doc = " ```ignore"]
#[doc = " #[async_std::test]"]
#[doc = " async fn my_test() -> std::io::Result<()> {"]
#[doc = "     assert_eq!(2 * 2, 4);"]
#[doc = "     Ok(())"]
#[doc = " }"]
#[doc = " ```"]
#[proc_macro_attribute]
pub fn test(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(test), args, input)
}
#[doc = " Enables an async benchmark function."]
#[doc = ""]
#[doc = " # Examples"]
#[doc = ""]
#[doc = " ```ignore"]
#[doc = " #![feature(test)]"]
#[doc = " extern crate test;"]
#[doc = ""]
#[doc = " #[async_std::bench]"]
#[doc = " async fn bench_1(b: &mut test::Bencher) {"]
#[doc = "     b.iter(|| {"]
#[doc = "         println!(\"hello world\");"]
#[doc = "     })"]
#[doc = " }"]
#[doc = " ```"]
#[proc_macro_attribute]
pub fn bench(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(bench), args, input)
}
