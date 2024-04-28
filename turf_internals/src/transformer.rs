use lightningcss::{
    selector::{Component, Selector},
    stylesheet::{ParserOptions, StyleSheet},
    visit_types,
    visitor::{Visit, VisitTypes, Visitor},
};
use regex::RegexSet;
use std::{collections::HashMap, convert::Infallible};

const CHARSET: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_-";

pub struct TransformationVisitor {
    pub(crate) classes: HashMap<String, String>,
    pub(crate) random_number_generator: oorandom::Rand32,
    pub(crate) class_name_template: String,
    pub(crate) class_name_exclude_patterns: RegexSet,
    pub(crate) debug: bool,
}

impl TransformationVisitor {
    fn randomized_class_id(&mut self, length: u32) -> String {
        // Creates a random id as part of a class template. The id consists of `length` characters.
        // With the exception of the first character, each character can be an alphanumeric, `_` or `-`.
        // The first character can only be a letter or `_` to stay compliant with the CSS spec.
        assert!(length <= 6, "Can be no longer than 6 characters, was {}", length); // Limited by rand_u32, must be rand_u64 for larger values
        let mut encoded_chars = String::new();
        if length <= 0 { return encoded_chars; }
        // Only allow a letter or `_` for the first character, doesn't allow a number or `-`
        let mut char_index = self.random_number_generator.rand_range(10..63) as usize;
        encoded_chars.push(CHARSET[char_index] as char);
        let mut random_bits = self.random_number_generator.rand_u32();
        for _ in 0..(length - 1) {
            char_index = (random_bits & 0x3F) as usize; // Only use the last 6 bits (0-64)
            encoded_chars.push(CHARSET[char_index] as char);
            random_bits >>= 6; // Shift to the next 6 bits
        }
        encoded_chars
    }

    fn randomized_class_name(&mut self, class_name: String) -> String {
        match self.classes.get(&class_name) {
            Some(random_class_name) => random_class_name.clone(),
            None => {
                let id: String = self.randomized_class_id(6);
                apply_template(&class_name, &self.class_name_template, &id)
            },
        }
    }
}

impl<'i> Visitor<'i> for TransformationVisitor {
    type Error = Infallible;

    fn visit_types(&self) -> VisitTypes {
        visit_types!(SELECTORS)
    }

    fn visit_selector(&mut self, selectors: &mut Selector<'i>) -> Result<(), Self::Error> {
        for selector in selectors.iter_mut_raw_match_order() {
            match selector {
                Component::Class(c) => {
                    let original_class_name = c.to_string();

                    if self.class_name_exclude_patterns.is_empty()
                        || !self
                            .class_name_exclude_patterns
                            .is_match(&original_class_name)
                    {
                        let new_class_name = self
                            .randomized_class_name(original_class_name.clone())
                            .to_string();
                        self.classes
                            .insert(original_class_name.clone(), new_class_name.clone());

                        if self.debug {
                            crate::compile_message(&format!(
                                "class name mapping - {:?} = {:?}",
                                &original_class_name, &new_class_name
                            ));
                        }

                        *c = new_class_name.into();
                    } else {
                        self.classes
                            .insert(original_class_name.clone(), original_class_name.clone());

                        if self.debug {
                            crate::compile_message(&format!(
                                "class name excluded - {:?}",
                                &original_class_name
                            ));
                        }
                    }
                }
                Component::Slotted(s) => {
                    s.visit(self)?
                },
                Component::Host(s) => {
                    if let Some(selector) = s {
                        selector.visit(self)?
                    }
                },
                Component::Negation(s) | Component::Where(s) | Component::Is(s) | 
                        Component::Any(_, s) | Component::Has(s) => {
                    s.iter_mut().try_for_each(|selector| selector.visit(self))?
                },
                _ => ()
            }
        }

        Ok(())
    }
}

fn apply_template(original_class_name: &str, class_name_template: &str, id: &str) -> String {
    class_name_template
        .replace("<original_name>", original_class_name)
        .replace("<id>", id)
}

#[derive(Debug, thiserror::Error)]
pub enum TransformationError {
    #[error("error transforming css - {0}")]
    Lightningcss(String),
    #[error("Initialization of css tranformer failed")]
    Initialization(#[from] crate::settings::TransformationVisitorInitializationError),
}

pub fn transform_stylesheet(
    css: &str,
    settings: crate::Settings,
) -> Result<(String, HashMap<String, String>), TransformationError> {
    let mut stylesheet = StyleSheet::parse(css, ParserOptions::default())
        .map_err(|e| e.to_string())
        .map_err(TransformationError::Lightningcss)?;

    let mut visitor = TransformationVisitor::try_from(&settings)?;

    stylesheet
        .visit(&mut visitor)
        .expect("css visitor never fails");

    let css_result = stylesheet
        .to_css(settings.into())
        .map_err(|e| e.to_string())
        .map_err(TransformationError::Lightningcss)?;

    Ok((css_result.code, visitor.classes))
}

#[cfg(test)]
mod tests {
    use crate::settings::ClassNameGeneration;

    use super::transform_stylesheet;

    #[test]
    fn basic_visitor() {
        let style = r#"
            .test {
                color: red;
            }
        "#;
        let transformation_result =
            transform_stylesheet(style, crate::Settings::default()).unwrap();

        assert!(transformation_result.0.starts_with(".class-"));
        assert!(transformation_result.0.ends_with("{color:red}"));
        assert!(transformation_result.0.starts_with(&format!(
            ".{}",
            transformation_result.1.get("test").unwrap()
        )));
    }

    #[test]
    fn inner_selector_visitor() {
        let style = r#"
            .test:not(.withoutme) {
                color: red;
            }
        "#;
        let transformation_result =
            transform_stylesheet(style, crate::Settings::default()).unwrap();

        assert!(transformation_result.0.starts_with(".class-"));
        assert!(transformation_result.0.ends_with("{color:red}"));
        assert!(transformation_result.0.starts_with(&format!(
            ".{}:not(.class-",
            transformation_result.1.get("test").unwrap()
        )));
        assert!(transformation_result.0.starts_with(&format!(
            ".{}:not(.{})",
            transformation_result.1.get("test").unwrap(),
            transformation_result.1.get("withoutme").unwrap()
        )));
    }

    #[test]
    fn custom_template() {
        let style = r#"
            .test {
                color: red;
            }
        "#;
        let class_name_generation = ClassNameGeneration {
            template: String::from("fancy_style-<original_name>-<id>"),
            ..Default::default()
        };
        let settings = crate::Settings {
            class_names: class_name_generation,
            ..Default::default()
        };
        let transformation_result = transform_stylesheet(style, settings).unwrap();

        assert!(transformation_result.0.starts_with(".fancy_style-test-"));
        assert!(transformation_result.0.ends_with("{color:red}"));
        assert!(transformation_result.0.starts_with(&format!(
            ".{}",
            transformation_result.1.get("test").unwrap()
        )));
    }

    #[test]
    fn custom_template_without_id() {
        let style = r#"
            .test {
                color: red;
            }
        "#;
        let class_name_generation = ClassNameGeneration {
            template: String::from("fancy_style-<original_name>"),
            ..Default::default()
        };
        let settings = crate::Settings {
            class_names: class_name_generation,
            ..Default::default()
        };
        let transformation_result = transform_stylesheet(style, settings).unwrap();

        assert_eq!(transformation_result.0, ".fancy_style-test{color:red}");
        assert!(transformation_result.0.starts_with(&format!(
            ".{}",
            transformation_result.1.get("test").unwrap()
        )));
    }
}
