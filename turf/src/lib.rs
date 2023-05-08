use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn style_sheet(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let path_str = input.value();
    let css = grass::from_path(path_str, &grass::Options::default()).unwrap();

    let style = stylist::Style::new(css).unwrap();
    let class_name = style.get_class_name();
    let style_sheet = style.get_style_str();

    let output = quote! {
        static CLASS_NAME: &'static str = #class_name;
        static STYLE_SHEET: &'static str = #style_sheet;
    };
    output.into()
}
