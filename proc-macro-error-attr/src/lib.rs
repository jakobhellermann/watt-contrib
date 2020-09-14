static WASM: &[u8] = include_bytes!("proc-macro-error-attr.wasm");
static MACRO: watt::WasmMacro = watt::WasmMacro::new(WASM);
#[proc_macro_attribute]
pub fn proc_macro_error(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(proc_macro_error), args, input)
}
