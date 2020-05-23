use proc_macro_hack::proc_macro_hack;
static MACRO: watt::WasmMacro = watt::WasmMacro::new(WASM);
static WASM: &[u8] = include_bytes!("futures-macro.wasm");
#[doc = " The `join!` macro."]
#[proc_macro_hack]
pub fn join_internal(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro(stringify!(join_internal), input)
}
#[doc = " The `try_join!` macro."]
#[proc_macro_hack]
pub fn try_join_internal(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro(stringify!(try_join_internal), input)
}
#[doc = " The `select!` macro."]
#[proc_macro_hack]
pub fn select_internal(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro(stringify!(select_internal), input)
}
#[doc = " The `select_biased!` macro."]
#[proc_macro_hack]
pub fn select_biased_internal(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro(stringify!(select_biased_internal), input)
}
