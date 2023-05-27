use lightningcss::{
    selector::{Component, Selector},
    stylesheet::{ParserOptions, StyleSheet},
    visit_types,
    visitor::{Visit, VisitTypes, Visitor},
};
use std::{collections::HashMap, convert::Infallible};

pub struct TransformationVisitor {
    pub(crate) classes: HashMap<String, String>,
    pub(crate) random_number_generator: oorandom::Rand32,
    pub(crate) class_name_template: String,
}

impl TransformationVisitor {
    fn randomized_class_name(&mut self, class_name: String) -> String {
        match self.classes.get(&class_name) {
            Some(random_class_name) => random_class_name.clone(),
            None => {
                let new_class_name = apply_template(
                    &class_name,
                    &self.class_name_template,
                    &self.random_number_generator.rand_u32().to_string(),
                );
                self.classes.insert(class_name, new_class_name.clone());

                new_class_name
            }
        }
    }
}

impl<'i> Visitor<'i> for TransformationVisitor {
    type Error = Infallible;

    const TYPES: VisitTypes = visit_types!(SELECTORS);

    fn visit_selector(&mut self, selectors: &mut Selector<'i>) -> Result<(), Self::Error> {
        for selector in selectors.iter_mut_raw_match_order() {
            if let Component::Class(c) = selector {
                *c = format!("{}", self.randomized_class_name(c.to_string())).into();
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

#[derive(Debug)]
pub struct LightningcssError(String);

impl From<String> for LightningcssError {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for LightningcssError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for LightningcssError {}

pub fn transform_stylesheet(
    css: &str,
    settings: crate::Settings,
) -> Result<(String, HashMap<String, String>), crate::Error> {
    let mut stylesheet = StyleSheet::parse(css, ParserOptions::default())
        .map_err(|e| e.to_string())
        .map_err(LightningcssError::from)?;

    let mut visitor = TransformationVisitor::try_from(&settings)?;

    stylesheet
        .visit(&mut visitor)
        .expect("css visitor never fails");

    let css_result = stylesheet
        .to_css(settings.into())
        .map_err(|e| e.to_string())
        .map_err(LightningcssError::from)?;

    Ok((css_result.code, visitor.classes))
}

#[cfg(test)]
mod tests {
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
    fn custom_template() {
        let style = r#"
            .test {
                color: red;
            }
        "#;
        let settings = crate::Settings {
            class_name_template: Some(String::from("fancy_style-<original_name>-<id>")),
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
        let settings = crate::Settings {
            class_name_template: Some(String::from("fancy_style-<original_name>")),
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
