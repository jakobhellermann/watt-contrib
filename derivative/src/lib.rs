static WASM: &[u8] = include_bytes!("derivative.wasm");
static MACRO: watt::WasmMacro = watt::WasmMacro::new(WASM);
#[cfg_attr(not(test), proc_macro_derive(Derivative, attributes(derivative)))]
pub fn derivative(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(derivative), input)
}
