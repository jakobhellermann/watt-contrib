static WASM: &[u8] = include_bytes!("wasm-bindgen-test-macro.wasm");
static MACRO: watt::WasmMacro = watt::WasmMacro::new(WASM);
#[proc_macro_attribute]
pub fn wasm_bindgen_test(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(wasm_bindgen_test), args, input)
}
