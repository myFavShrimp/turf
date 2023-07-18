#[test]
fn use_default_settings() {
    turf::style_sheet!("use_default_settings/src/mystyle.scss");
    assert!(STYLE_SHEET.ends_with("{color:#333}"));
    assert!(STYLE_SHEET.starts_with(".class-"));
    assert!(STYLE_SHEET.starts_with(&format!(".{}", ClassName::TEST)));
}
