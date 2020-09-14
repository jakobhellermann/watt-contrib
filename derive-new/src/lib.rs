static WASM: &[u8] = include_bytes!("derive-new.wasm");
static MACRO: watt::WasmMacro = watt::WasmMacro::new(WASM);
#[proc_macro_derive(new, attributes(new))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(derive), input)
}
