use std::path::PathBuf;

use serde::Deserialize;

use crate::manifest::{MetadataWithTurfSettings, PackageWithMetadata};

#[derive(Deserialize, Debug, Default)]
pub struct Settings {
    minify: bool,
    load_paths: Vec<PathBuf>,
}

impl<'a> From<Settings> for grass::Options<'a> {
    fn from(val: Settings) -> Self {
        grass::Options::default()
            .style(grass::OutputStyle::Expanded)
            .load_paths(&val.load_paths)
    }
}

impl<'a> From<Settings> for lightningcss::printer::PrinterOptions<'a> {
    fn from(val: Settings) -> Self {
        lightningcss::printer::PrinterOptions {
            minify: val.minify,
            project_root: None,
            targets: None,
            analyze_dependencies: None,
            pseudo_classes: None,
        }
    }
}

impl Settings {
    pub fn from_cargo_manifest_metadata_or_default() -> Result<Self, crate::Error> {
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
