#[test]
fn load_paths_from_cargo_manifest() {
    turf::style_sheet!("ignore_settings_from_cargo_manifest/src/mystyle.scss");
    assert!(STYLE_SHEET.contains("{\n    color: #333;\n}\n"));
}
