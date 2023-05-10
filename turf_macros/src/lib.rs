use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn style_sheet(input: TokenStream) -> TokenStream {
    let input = input.to_string();

    let css = grass::from_path(&input, &grass::Options::default()).unwrap_or_else(|e| panic!("{}", e));

    let style = stylist::Style::new(css).unwrap();
    let class_name = style.get_class_name();
    let style_sheet = style.get_style_str();

    let output = quote! {
        static CLASS_NAME: &'static str = #class_name;
        static STYLE_SHEET: &'static str = #style_sheet;
    };
    output.into()
}
