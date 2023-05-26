use std::path::PathBuf;

use serde::Deserialize;

use crate::manifest::{MetadataWithTurfSettings, PackageWithMetadata};

#[derive(Deserialize, Debug, Default)]
pub struct Settings {
    minify: bool,
    load_paths: Vec<PathBuf>,
}

impl<'a> Into<grass::Options<'a>> for Settings {
    fn into(self) -> grass::Options<'a> {
        grass::Options::default()
            .style(grass::OutputStyle::Expanded)
            .load_paths(&self.load_paths)
    }
}

impl<'a> Into<lightningcss::printer::PrinterOptions<'a>> for Settings {
    fn into(self) -> lightningcss::printer::PrinterOptions<'a> {
        lightningcss::printer::PrinterOptions {
            minify: self.minify,
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
