use proc_macro::TokenStream;
use quote::{quote};

#[proc_macro]
pub fn style_sheet(input: TokenStream) -> TokenStream {
    let input = input.to_string();

    let (class_name, style_sheet) = match turf_internals::macro_functions::style_sheet_with_default_compile_options(input).map_err(to_compile_error) {
        Ok(values) => values,
        Err(e) => return e,
    };

    quote! {
        static CLASS_NAME: &'static str = #class_name;
        static STYLE_SHEET: &'static str = #style_sheet;
    }.into()
}

#[proc_macro]
pub fn configured_style_sheet(input: TokenStream) -> TokenStream {
    let input = input.to_string();

    let (class_name, style_sheet) = match turf_internals::macro_functions::style_sheet(input).map_err(to_compile_error) {
        Ok(values) => values,
        Err(e) => return e,
    };

    quote! {
        static CLASS_NAME: &'static str = #class_name;
        static STYLE_SHEET: &'static str = #style_sheet;
    }.into()
}

fn to_compile_error(e: turf_internals::Error) -> TokenStream {
    let message = e.to_string();
    quote! {
        compile_error!(#message);
    }.into()
}
