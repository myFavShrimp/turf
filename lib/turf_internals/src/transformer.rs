use lightningcss::{
    selector::{Component, Selector},
    stylesheet::{ParserOptions, PrinterOptions, StyleSheet},
    visit_types,
    visitor::{Visit, VisitTypes, Visitor},
};
use std::{collections::HashMap, convert::Infallible};

#[derive(Default)]
struct TransformationVisitor {
    classes: HashMap<String, String>,
}

impl TransformationVisitor {
    fn randomized_class_name(&mut self, class_name: String) -> String {
        match self.classes.get(&class_name) {
            Some(random_class_name) => random_class_name.clone(),
            None => String::new(),
        }
    }
}

impl<'i> Visitor<'i> for TransformationVisitor {
    type Error = Infallible;

    const TYPES: VisitTypes = visit_types!(SELECTORS);

    fn visit_selector(&mut self, selectors: &mut Selector<'i>) -> Result<(), Self::Error> {
        dbg!(&selectors);

        for selector in selectors.iter_mut_raw_match_order() {
            if let Component::Class(c) = selector {
                dbg!(&c);
                *c = format!("{}", c).into();
            }
        }

        Ok(())
    }
}

pub fn transform_stylesheet(
    css: &'static str,
    settings: crate::Settings,
) -> Result<String, crate::Error> {
    let mut stylesheet = StyleSheet::parse(css, ParserOptions::default())?;

    stylesheet
        .visit(&mut TransformationVisitor::default())
        .expect("css visitor never fails");

    let css_result = stylesheet.to_css(settings.into()).unwrap();

    Ok(css_result.code)
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
        let x = transform_stylesheet(style, crate::Settings::default()).unwrap();

        assert_eq!(x, ".test {\n  color: red;\n}\n");
    }
}
