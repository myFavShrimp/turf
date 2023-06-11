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
    pub package: Option<PackageWithMetadata>,
}

#[derive(Deserialize, Debug)]
pub struct PackageWithMetadata {
    pub metadata: Option<MetadataWithTurfSettings>,
}

#[derive(Deserialize, Debug)]
pub struct MetadataWithTurfSettings {
    pub turf: Option<crate::settings::Settings>,
    #[serde(rename = "turf-dev")]
    pub turf_dev: Option<crate::settings::Settings>,
}
