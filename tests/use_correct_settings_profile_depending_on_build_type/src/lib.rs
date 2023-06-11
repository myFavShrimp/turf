#[test]
fn use_correct_settings() {
    turf::style_sheet!("use_default_settings/src/mystyle.scss");
    if cfg!(debug_assertions) {
        assert_eq!(STYLE_SHEET, ".dev-test {\n  color: #333;\n}\n");
    }
    if cfg!(not(debug_assertions)) {
        assert_eq!(STYLE_SHEET, ".prod-test{color:#333}");
    }
}
