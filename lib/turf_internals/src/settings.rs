use std::path::PathBuf;

use serde::Deserialize;

use crate::manifest::{MetadataWithTurfSettings, PackageWithMetadata};

#[derive(Deserialize, Debug, Default)]
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

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub enum OutputStyle {
    #[default]
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

impl Settings {
    pub fn from_cargo_manifest_metadata() -> Result<Self, crate::Error> {
        let manifest_data = crate::manifest::cargo_manifest()?;

        if let Some(PackageWithMetadata {
            metadata:
                Some(MetadataWithTurfSettings {
                    turf: Some(turf_settings),
                }),
        }) = manifest_data.package
        {
            Ok(turf_settings)
        } else {
            Ok(Settings::default())
        }
    }
}
