static MACRO: watt::WasmMacro = watt::WasmMacro::new(WASM);
static WASM: &[u8] = include_bytes!("quickcheck_macros.wasm");
#[proc_macro_attribute]
pub fn quickcheck(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(quickcheck), args, input)
}
