use lightningcss::{
    selector::{Component, Selector},
    stylesheet::{ParserOptions, PrinterOptions, StyleSheet},
    visit_types,
    visitor::{Visit, VisitTypes, Visitor},
};
use std::convert::Infallible;

struct MyVisitor;
impl<'i> Visitor<'i> for MyVisitor {
    type Error = Infallible;

    const TYPES: VisitTypes = visit_types!(SELECTORS);

    fn visit_selector(&mut self, selectors: &mut Selector<'i>) -> Result<(), Self::Error> {
        dbg!(&selectors);

        for selector in selectors.iter_mut_raw_match_order() {
            match selector {
                Component::Class(c) => {
                    dbg!(&c);
                    *c = format!("{}", c).into();
                }
                _ => {}
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
        .visit(&mut MyVisitor)
        .expect("css visitor never fails");
    Ok(stylesheet.to_css(PrinterOptions::default()).unwrap().code)
}

#[cfg(test)]
mod tests {
    use super::transform_stylesheet;

    #[test]
    fn visitor_testing() {
        let style = r#"
            .test {
                color: red;
            }
        "#;

        let x = transform_stylesheet(style, crate::Settings::default()).unwrap();

        assert_eq!(x, ".test {\n  color: red;\n}\n");
    }
}
