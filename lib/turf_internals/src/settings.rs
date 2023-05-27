use std::path::PathBuf;

use serde::Deserialize;

use crate::manifest::{MetadataWithTurfSettings, PackageWithMetadata};

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Settings {
    pub(crate) minify: Option<bool>,
    pub(crate) load_paths: Option<Vec<PathBuf>>,
    pub(crate) browser_targets: Option<BrowserVersions>,
}

impl<'a> From<Settings> for grass::Options<'a> {
    fn from(val: Settings) -> Self {
        grass::Options::default()
            .style(grass::OutputStyle::Expanded)
            .load_paths(&val.load_paths.unwrap_or(Default::default()))
    }
}

impl<'a> From<Settings> for lightningcss::printer::PrinterOptions<'a> {
    fn from(val: Settings) -> Self {
        lightningcss::printer::PrinterOptions {
            minify: val.minify.unwrap_or(true),
            project_root: None,
            targets: val.browser_targets.map(From::<BrowserVersions>::from),
            analyze_dependencies: None,
            pseudo_classes: None,
        }
    }
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct BrowserVersions {
    pub android: Option<BrowserVersion>,
    pub chrome: Option<BrowserVersion>,
    pub edge: Option<BrowserVersion>,
    pub firefox: Option<BrowserVersion>,
    pub ie: Option<BrowserVersion>,
    pub ios_saf: Option<BrowserVersion>,
    pub opera: Option<BrowserVersion>,
    pub safari: Option<BrowserVersion>,
    pub samsung: Option<BrowserVersion>,
}

impl From<BrowserVersions> for lightningcss::targets::Browsers {
    fn from(value: BrowserVersions) -> Self {
        Self {
            android: value.android.map(From::<BrowserVersion>::from),
            chrome: value.chrome.map(From::<BrowserVersion>::from),
            edge: value.edge.map(From::<BrowserVersion>::from),
            firefox: value.firefox.map(From::<BrowserVersion>::from),
            ie: value.ie.map(From::<BrowserVersion>::from),
            ios_saf: value.ios_saf.map(From::<BrowserVersion>::from),
            opera: value.opera.map(From::<BrowserVersion>::from),
            safari: value.safari.map(From::<BrowserVersion>::from),
            samsung: value.samsung.map(From::<BrowserVersion>::from),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum BrowserVersion {
    WithMajor(u8),
    WithMinor(u8, u8),
    WithPatch(u8, u8, u8),
}

impl From<BrowserVersion> for u32 {
    fn from(value: BrowserVersion) -> Self {
        u32::from_ne_bytes(match value {
            BrowserVersion::WithMajor(major) => [major, 0, 0, 0],
            BrowserVersion::WithMinor(major, minor) => [major, minor, 0, 0],
            BrowserVersion::WithPatch(major, minor, path) => [major, minor, path, 0],
        })
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
