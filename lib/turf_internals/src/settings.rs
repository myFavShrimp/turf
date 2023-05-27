use std::path::PathBuf;

use serde::Deserialize;

use crate::manifest::{MetadataWithTurfSettings, PackageWithMetadata};

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Settings {
    pub(crate) minify: Option<bool>,
    pub(crate) load_paths: Option<Vec<PathBuf>>,
    pub(crate) browser_targets: Option<BrowserVersions>,
    pub(crate) class_name_template: Option<String>,
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

impl TryFrom<&crate::Settings> for crate::transformer::TransformationVisitor {
    type Error = crate::Error;

    fn try_from(value: &crate::Settings) -> Result<Self, Self::Error> {
        Ok(Self {
            classes: Default::default(),
            random_number_generator: oorandom::Rand32::new(crate::random_seed()?),
            class_name_template: value
                .class_name_template
                .clone()
                .unwrap_or(String::from("class-<id>")),
        })
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
    Major(u8),
    MajorMinor(u8, u8),
    MajorMinorPatch(u8, u8, u8),
}

impl From<BrowserVersion> for u32 {
    fn from(value: BrowserVersion) -> Self {
        let version = match value {
            BrowserVersion::Major(major) => (major, 0, 0),
            BrowserVersion::MajorMinor(major, minor) => (major, minor, 0),
            BrowserVersion::MajorMinorPatch(major, minor, path) => (major, minor, path),
        };
        (version.0 as u32 & 0xff) << 16 | (version.1 as u32 & 0xff) << 8 | (version.2 as u32 & 0xff)
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
