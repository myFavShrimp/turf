#[test]
fn load_paths_from_cargo_manifest() {
    turf::configured_style_sheet!("load_settings_from_cargo_manifest/src/mystyle.scss");
    assert!(STYLE_SHEET.contains("color: #69e69d;"));
}