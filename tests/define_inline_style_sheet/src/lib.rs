#[test]
fn inline_scss_style() {
    turf::inline_style_sheet! {
        @import "color";

        .test {
            color: $some-color;
        }
    };
    assert!(STYLE_SHEET.starts_with(".class-"));
    assert!(STYLE_SHEET.ends_with(" {\n  color: #69e69d;\n}\n"));
    assert!(STYLE_SHEET.starts_with(&format!(".{}", ClassName::TEST)));
}
