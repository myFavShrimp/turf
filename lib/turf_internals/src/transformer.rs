use lightningcss::{
    selector::{Component, Selector},
    stylesheet::{ParserOptions, StyleSheet},
    visit_types,
    visitor::{Visit, VisitTypes, Visitor},
};
use std::{collections::HashMap, convert::Infallible};

fn random_seed() -> Result<u64, getrandom::Error> {
    let mut buf = [0u8; 8];
    getrandom::getrandom(&mut buf)?;
    Ok(u64::from_ne_bytes(buf))
}

struct TransformationVisitor {
    pub classes: HashMap<String, String>,
    random_number_generator: oorandom::Rand32,
}

impl Default for TransformationVisitor {
    fn default() -> Self {
        Self {
            classes: Default::default(),
            random_number_generator: oorandom::Rand32::new(random_seed().unwrap()),
        }
    }
}

impl TransformationVisitor {
    fn randomized_class_name(&mut self, class_name: String) -> String {
        match self.classes.get(&class_name.clone()) {
            Some(random_class_name) => random_class_name.clone(),
            None => {
                let mut new_class_name = class_name.clone();
                new_class_name.push_str(&self.random_number_generator.rand_u32().to_string());
                new_class_name
            }
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
                *c = format!("{}", dbg!(self.randomized_class_name(c.to_string()))).into();
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct LightningcssError(String);

impl From<String> for LightningcssError {
    fn from(value: String) -> Self {
        Self(value.to_string())
    }
}

impl std::fmt::Display for LightningcssError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for LightningcssError {}

pub fn transform_stylesheet(css: &str, settings: crate::Settings) -> Result<String, crate::Error> {
    let mut stylesheet = StyleSheet::parse(css, ParserOptions::default())
        .map_err(|e| e.to_string())
        .map_err(LightningcssError::from)?;

    stylesheet
        .visit(&mut TransformationVisitor::default())
        .expect("css visitor never fails");

    let css_result = stylesheet
        .to_css(settings.into())
        .map_err(|e| e.to_string())
        .map_err(LightningcssError::from)?;

    Ok(css_result.code.clone())
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

        assert!(x.starts_with("."));
        assert!(x.ends_with(" {\n  color: red;\n}\n"));
    }
}
