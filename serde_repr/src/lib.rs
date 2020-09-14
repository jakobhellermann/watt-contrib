static WASM: &[u8] = include_bytes!("serde_repr.wasm");
static MACRO: watt::WasmMacro = watt::WasmMacro::new(WASM);
#[proc_macro_derive(Serialize_repr)]
pub fn derive_serialize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(derive_serialize), input)
}
#[proc_macro_derive(Deserialize_repr, attributes(serde))]
pub fn derive_deserialize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(derive_deserialize), input)
}
