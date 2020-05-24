static MACRO: watt::WasmMacro = watt::WasmMacro::new(WASM);
static WASM: &[u8] = include_bytes!("wasm-bindgen-macro.wasm");
#[proc_macro_attribute]
pub fn wasm_bindgen(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(wasm_bindgen), args, input)
}
#[proc_macro_attribute]
pub fn __wasm_bindgen_class_marker(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(__wasm_bindgen_class_marker), args, input)
}
