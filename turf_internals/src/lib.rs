use std::{fs::read_to_string, path::PathBuf};

use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("toml error")]
    TomlError(#[from] toml::de::Error),
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("error with cargo manifest '{0}'")]
    ManifestError(#[from] ManifestError),
}

#[derive(thiserror::Error, Debug)]
pub enum ManifestError {
    #[error("error reading cargo manifest")]
    ReadError,
    #[error("key not found in metadata: {0}")]
    KeyError(&'static str),
}

#[derive(Deserialize, Debug)]
pub struct Settings {
    output_style: OutputStyle,
    load_paths: Vec<PathBuf>,
}

impl<'a> Into<grass::Options<'a>> for Settings {
    fn into(self) -> grass::Options<'a> {
        grass::Options::default()
            .style(self.output_style.into())
            .load_paths(&self.load_paths)
    }
}

impl Settings {
    fn from_cargo_manifest_metadata() -> Result<Self, Error> {
        let manifest_path = format!(
            "{}/Cargo.toml",
            std::env::var("CARGO_MANIFEST_DIR").unwrap()
        );
        let manifest: toml::value::Value = toml::de::from_str(&read_to_string(manifest_path)?)?;
        let metadata_table = manifest.as_table().ok_or(ManifestError::ReadError)?;

        let package_metadata = metadata_table
            .get("package")
            .ok_or(ManifestError::KeyError("package"))?
            .as_table()
            .ok_or(ManifestError::ReadError)?
            .get("metadata")
            .ok_or(ManifestError::KeyError("metadata"))?
            .as_table()
            .ok_or(ManifestError::ReadError)?;
        let turf_metadata = package_metadata
            .get("turf")
            .ok_or(ManifestError::KeyError("turf"))?
            .as_table()
            .ok_or(ManifestError::ReadError)?;

        Ok(toml::from_str(&turf_metadata.to_string())?)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum OutputStyle {
    Expanded,
    Compressed,
}

impl Into<grass::OutputStyle> for OutputStyle {
    fn into(self) -> grass::OutputStyle {
        match self {
            OutputStyle::Expanded => grass::OutputStyle::Expanded,
            OutputStyle::Compressed => grass::OutputStyle::Compressed,
        }
    }
}

#[test]
fn dbg_dev_helper() {
    dbg!(Settings::from_cargo_manifest_metadata());
    assert!(false);
}
