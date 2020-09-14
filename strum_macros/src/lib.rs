static WASM: &[u8] = include_bytes!("strum_macros.wasm");
static MACRO: watt::WasmMacro = watt::WasmMacro::new(WASM);
#[cfg_attr(
    not(feature = "verbose-enumstring-name"),
    proc_macro_derive(EnumString, attributes(strum))
)]
#[cfg_attr(
    feature = "verbose-enumstring-name",
    proc_macro_derive(StrumEnumString, attributes(strum))
)]
pub fn from_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(from_string), input)
}
#[cfg_attr(
    not(feature = "verbose-asrefstr-name"),
    proc_macro_derive(AsRefStr, attributes(strum))
)]
#[cfg_attr(
    feature = "verbose-asrefstr-name",
    proc_macro_derive(StrumAsRefStr, attributes(strum))
)]
pub fn as_ref_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(as_ref_str), input)
}
#[cfg_attr(
    not(feature = "verbose-variant-names"),
    proc_macro_derive(EnumVariantNames, attributes(strum))
)]
#[cfg_attr(
    feature = "verbose-variant-names",
    proc_macro_derive(StrumEnumVariantNames, attributes(strum))
)]
pub fn variant_names(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(variant_names), input)
}
#[cfg_attr(
    feature = "verbose-asstaticstr-name",
    proc_macro_derive(StrumAsStaticStr, attributes(strum))
)]
#[cfg_attr(
    not(feature = "verbose-asstaticstr-name"),
    proc_macro_derive(AsStaticStr, attributes(strum))
)]
pub fn as_static_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(as_static_str), input)
}
#[cfg_attr(
    feature = "verbose-intostaticstr-name",
    proc_macro_derive(StrumIntoStaticStr, attributes(strum))
)]
#[cfg_attr(
    not(feature = "verbose-intostaticstr-name"),
    proc_macro_derive(IntoStaticStr, attributes(strum))
)]
pub fn into_static_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(into_static_str), input)
}
#[cfg_attr(
    feature = "verbose-tostring-name",
    proc_macro_derive(StrumToString, attributes(strum))
)]
#[cfg_attr(
    not(feature = "verbose-tostring-name"),
    proc_macro_derive(ToString, attributes(strum))
)]
pub fn to_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(to_string), input)
}
#[cfg_attr(
    feature = "verbose-display-name",
    proc_macro_derive(StrumDisplay, attributes(strum))
)]
#[cfg_attr(
    not(feature = "verbose-display-name"),
    proc_macro_derive(Display, attributes(strum))
)]
pub fn display(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(display), input)
}
#[cfg_attr(
    feature = "verbose-enumiter-name",
    proc_macro_derive(StrumEnumIter, attributes(strum))
)]
#[cfg_attr(
    not(feature = "verbose-enumiter-name"),
    proc_macro_derive(EnumIter, attributes(strum))
)]
pub fn enum_iter(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(enum_iter), input)
}
#[cfg_attr(
    feature = "verbose-enummessage-name",
    proc_macro_derive(StrumEnumMessage, attributes(strum))
)]
#[cfg_attr(
    not(feature = "verbose-enummessage-name"),
    proc_macro_derive(EnumMessage, attributes(strum))
)]
pub fn enum_messages(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(enum_messages), input)
}
#[cfg_attr(
    feature = "verbose-enumproperty-name",
    proc_macro_derive(StrumEnumProperty, attributes(strum))
)]
#[cfg_attr(
    not(feature = "verbose-enumproperty-name"),
    proc_macro_derive(EnumProperty, attributes(strum))
)]
pub fn enum_properties(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(enum_properties), input)
}
#[cfg_attr(
    feature = "verbose-enumdiscriminants-name",
    proc_macro_derive(StrumEnumDiscriminants, attributes(strum, strum_discriminants))
)]
#[cfg_attr(
    not(feature = "verbose-enumdiscriminants-name"),
    proc_macro_derive(EnumDiscriminants, attributes(strum, strum_discriminants))
)]
pub fn enum_discriminants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(enum_discriminants), input)
}
#[cfg_attr(
    feature = "verbose-enumcount-name",
    proc_macro_derive(StrumEnumCount, attributes(strum))
)]
#[cfg_attr(
    not(feature = "verbose-enumcount-name"),
    proc_macro_derive(EnumCount, attributes(strum))
)]
pub fn enum_count(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    MACRO.proc_macro_derive(stringify!(enum_count), input)
}
