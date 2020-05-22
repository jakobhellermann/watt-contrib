static MACRO: watt::WasmMacro = watt::WasmMacro::new(WASM);
static WASM: &[u8] = include_bytes!("async-trait.wasm");
#[proc_macro_attribute]
pub fn async_trait(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(async_trait), args, input)
}
