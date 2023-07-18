#[test]
fn load_paths_from_cargo_manifest() {
    turf::style_sheet!("load_settings_from_cargo_manifest/src/mystyle.scss");
    assert!(STYLE_SHEET.starts_with(".class-"));
    assert!(STYLE_SHEET.ends_with(" {\n  color: #69e69d;\n}\n"));
    assert!(STYLE_SHEET.starts_with(&format!(".{}", ClassName::TEST)));
}
