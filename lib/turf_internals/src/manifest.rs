use std::fs::read_to_string;

use serde::Deserialize;

pub fn cargo_manifest() -> Result<ManifestWithPackage, crate::Error> {
    let manifest_path = format!(
        "{}/Cargo.toml",
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    );
    Ok(toml::de::from_str(&read_to_string(manifest_path)?)?)
}

#[derive(Deserialize, Debug)]
pub struct ManifestWithPackage {
    pub package: PackageWithMetadata,
}

#[derive(Deserialize, Debug)]
pub struct PackageWithMetadata {
    pub metadata: MetadataWithTurfSettings,
}

#[derive(Deserialize, Debug)]
pub struct MetadataWithTurfSettings {
    pub turf: crate::settings::Settings,
}
