static WASM: &[u8] = include_bytes!("actix_derive.wasm");
static MACRO: watt::WasmMacro = watt::WasmMacro::new(WASM);
#[proc_macro_derive(Message, attributes(rtype))]
pub fn message_derive_rtype(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(message_derive_rtype), input)
}
#[proc_macro_derive(MessageResponse)]
pub fn message_response_derive_rtype(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(message_response_derive_rtype), input)
}
