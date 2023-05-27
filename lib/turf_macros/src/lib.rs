//! You're probably looking for `turf` instead.

use convert_case::{Case, Casing};
use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn style_sheet(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let sanitized_input = input.trim_matches('"');

    let (style_sheet, class_names) =
        match turf_internals::macro_functions::style_sheet(sanitized_input)
            .map_err(to_compile_error)
        {
            Ok(values) => values,
            Err(e) => return e,
        };

    let mut out = quote! {
        static STYLE_SHEET: &'static str = #style_sheet;
    };
    out.extend(create_classes_structure(class_names));

    out.into()
}

fn to_compile_error(e: turf_internals::Error) -> TokenStream {
    let message = e.to_string();
    quote! {
        compile_error!(#message);
    }
    .into()
}

fn create_classes_structure(classes: HashMap<String, String>) -> proc_macro2::TokenStream {
    let original_class_names: Vec<proc_macro2::Ident> = classes
        .keys()
        .map(|class| class.to_case(Case::UpperSnake))
        .map(|class| quote::format_ident!("{}", class.as_str().to_uppercase()))
        .collect();

    let randomized_class_names: Vec<&String> = classes.values().collect();

    quote::quote! {
        struct ClassName;
        impl ClassName {
            #(pub const #original_class_names: &'static str = #randomized_class_names;)*
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::create_classes_structure;

    #[test]
    fn test() {
        let mut class_names = HashMap::new();
        class_names.insert(String::from("test-class"), String::from("abc-123"));

        let y = create_classes_structure(class_names);

        assert_eq!(
            y.to_string(),
            quote::quote! {
                struct ClassName;
                impl ClassName {
                    pub const TEST_CLASS: &'static str = "abc-123";
                }
            }
            .to_string()
        )
    }
}
