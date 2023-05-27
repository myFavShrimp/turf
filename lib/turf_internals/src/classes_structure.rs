use convert_case::{Case, Casing};
use std::collections::HashMap;

pub fn create_classes_structure(classes: HashMap<String, String>) -> proc_macro2::TokenStream {
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
