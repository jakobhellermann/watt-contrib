static WASM: &[u8] = include_bytes!("scroll_derive.wasm");
static MACRO: watt::WasmMacro = watt::WasmMacro::new(WASM);
#[proc_macro_derive(Pread)]
pub fn derive_pread(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(derive_pread), input)
}
#[proc_macro_derive(Pwrite)]
pub fn derive_pwrite(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(derive_pwrite), input)
}
#[proc_macro_derive(SizeWith)]
pub fn derive_sizewith(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(derive_sizewith), input)
}
#[proc_macro_derive(IOread)]
pub fn derive_ioread(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(derive_ioread), input)
}
#[proc_macro_derive(IOwrite)]
pub fn derive_iowrite(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(derive_iowrite), input)
}
