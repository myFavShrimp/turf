use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("toml error")]
    TomlError(#[from] toml::de::Error),
    #[error("io error")]
    IoError(#[from] std::io::Error),
}

#[derive(Deserialize)]
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
    fn from_cargo_manifest_metadata() -> Result<(), Error> {
        let manifest_path = format!(
            "{}/Cargo.toml",
            std::env::var("CARGO_MANIFEST_DIR").unwrap()
        );
        let manifest: toml::value::Value = toml::de::from_str(&read_to_string(manifest_path)?)?;
        let metadata_table = manifest.as_table().unwrap();

        let package_metadata = metadata_table
            .get("package")
            .unwrap()
            .as_table()
            .unwrap()
            .get("metadata")
            .unwrap()
            .as_table()
            .unwrap();
        let turf_metadata = package_metadata.get("turf").unwrap().as_table().unwrap();

        dbg!(turf_metadata);

        Ok(())
        // Ok(toml::de::from_str(&read_to_string(path)?)?)
    }
}

#[derive(Deserialize)]
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
    Settings::from_cargo_manifest_metadata();
    assert!(false);
}
