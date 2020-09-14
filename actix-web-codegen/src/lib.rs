static WASM: &[u8] = include_bytes!("actix-web-codegen.wasm");
static MACRO: watt::WasmMacro = watt::WasmMacro::new(WASM);
#[doc = " Creates route handler with `GET` method guard."]
#[doc = ""]
#[doc = " Syntax: `#[get(\"path\"[, attributes])]`"]
#[doc = ""]
#[doc = " ## Attributes:"]
#[doc = ""]
#[doc = " - `\"path\"` - Raw literal string with path for which to register handler. Mandatory."]
#[doc = " - `guard=\"function_name\"` - Registers function as guard using `actix_web::guard::fn_guard`"]
#[doc = " - `wrap=\"Middleware\"` - Registers a resource middleware."]
#[proc_macro_attribute]
pub fn get(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(get), args, input)
}
#[doc = " Creates route handler with `POST` method guard."]
#[doc = ""]
#[doc = " Syntax: `#[post(\"path\"[, attributes])]`"]
#[doc = ""]
#[doc = " Attributes are the same as in [get](attr.get.html)"]
#[proc_macro_attribute]
pub fn post(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(post), args, input)
}
#[doc = " Creates route handler with `PUT` method guard."]
#[doc = ""]
#[doc = " Syntax: `#[put(\"path\"[, attributes])]`"]
#[doc = ""]
#[doc = " Attributes are the same as in [get](attr.get.html)"]
#[proc_macro_attribute]
pub fn put(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(put), args, input)
}
#[doc = " Creates route handler with `DELETE` method guard."]
#[doc = ""]
#[doc = " Syntax: `#[delete(\"path\"[, attributes])]`"]
#[doc = ""]
#[doc = " Attributes are the same as in [get](attr.get.html)"]
#[proc_macro_attribute]
pub fn delete(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(delete), args, input)
}
#[doc = " Creates route handler with `HEAD` method guard."]
#[doc = ""]
#[doc = " Syntax: `#[head(\"path\"[, attributes])]`"]
#[doc = ""]
#[doc = " Attributes are the same as in [head](attr.head.html)"]
#[proc_macro_attribute]
pub fn head(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(head), args, input)
}
#[doc = " Creates route handler with `CONNECT` method guard."]
#[doc = ""]
#[doc = " Syntax: `#[connect(\"path\"[, attributes])]`"]
#[doc = ""]
#[doc = " Attributes are the same as in [connect](attr.connect.html)"]
#[proc_macro_attribute]
pub fn connect(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(connect), args, input)
}
#[doc = " Creates route handler with `OPTIONS` method guard."]
#[doc = ""]
#[doc = " Syntax: `#[options(\"path\"[, attributes])]`"]
#[doc = ""]
#[doc = " Attributes are the same as in [options](attr.options.html)"]
#[proc_macro_attribute]
pub fn options(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(options), args, input)
}
#[doc = " Creates route handler with `TRACE` method guard."]
#[doc = ""]
#[doc = " Syntax: `#[trace(\"path\"[, attributes])]`"]
#[doc = ""]
#[doc = " Attributes are the same as in [trace](attr.trace.html)"]
#[proc_macro_attribute]
pub fn trace(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(trace), args, input)
}
#[doc = " Creates route handler with `PATCH` method guard."]
#[doc = ""]
#[doc = " Syntax: `#[patch(\"path\"[, attributes])]`"]
#[doc = ""]
#[doc = " Attributes are the same as in [patch](attr.patch.html)"]
#[proc_macro_attribute]
pub fn patch(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(patch), args, input)
}
#[doc = " Marks async main function as the actix system entry-point."]
#[doc = ""]
#[doc = " ## Usage"]
#[doc = ""]
#[doc = " ```rust"]
#[doc = " #[actix_web::main]"]
#[doc = " async fn main() {"]
#[doc = "     async { println!(\"Hello world\"); }.await"]
#[doc = " }"]
#[doc = " ```"]
#[proc_macro_attribute]
#[cfg(not(test))]
pub fn main(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    MACRO.proc_macro_attribute(stringify!(main), args, input)
}
