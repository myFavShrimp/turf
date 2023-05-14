#[test]
fn load_paths_from_cargo_manifest() {
    turf::style_sheet!("use_default_settings/src/mystyle.scss");
    assert!(STYLE_SHEET.contains("{\n    color: #333;\n}\n"));
}
