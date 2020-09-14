static WASM: &[u8] = include_bytes!("proc-macro-hack.wasm");
static MACRO: watt::WasmMacro = watt::WasmMacro::new(WASM);
#[proc_macro_attribute]
pub fn proc_macro_hack(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(proc_macro_hack), args, input)
}
#[doc(hidden)]
#[proc_macro_derive(ProcMacroHack)]
pub fn enum_hack(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(enum_hack), input)
}
#[doc(hidden)]
#[proc_macro_attribute]
pub fn fake_call_site(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(fake_call_site), args, input)
}
