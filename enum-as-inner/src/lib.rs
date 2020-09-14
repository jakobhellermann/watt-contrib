static WASM: &[u8] = include_bytes!("enum-as-inner.wasm");
static MACRO: watt::WasmMacro = watt::WasmMacro::new(WASM);
#[proc_macro_derive(EnumAsInner)]
pub fn enum_as_inner(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(enum_as_inner), input)
}
