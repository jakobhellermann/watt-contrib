static WASM: &[u8] = include_bytes!("thiserror-impl.wasm");
static MACRO: watt::WasmMacro = watt::WasmMacro::new(WASM);
#[proc_macro_derive(Error, attributes(backtrace, error, from, source))]
pub fn derive_error(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(derive_error), input)
}
