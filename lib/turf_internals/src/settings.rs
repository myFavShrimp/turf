use std::{path::PathBuf, sync::OnceLock};

use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Settings {
    pub(crate) debug: Option<bool>,
    pub(crate) minify: Option<bool>,
    pub(crate) load_paths: Option<Vec<PathBuf>>,
    pub(crate) browser_targets: Option<BrowserVersions>,
    pub(crate) class_name_template: Option<String>,
}

impl Settings {
    pub fn debug_enabled(&self) -> bool {
        self.debug.unwrap_or(false)
    }
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
            targets: val
                .browser_targets
                .map(From::<BrowserVersions>::from)
                .into(),
            analyze_dependencies: None,
            pseudo_classes: None,
        }
    }
}

impl TryFrom<&crate::Settings> for crate::transformer::TransformationVisitor {
    type Error = crate::Error;

    fn try_from(value: &crate::Settings) -> Result<Self, Self::Error> {
        Ok(Self {
            debug: value.debug.unwrap_or(false),
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

static TURF_SETTINGS: OnceLock<Settings> = OnceLock::new();
static TURF_DEV_SETTINGS: OnceLock<Settings> = OnceLock::new();

impl Settings {
    pub fn get() -> Result<Self, crate::Error> {
        if cfg!(debug_assertions) {
            if let Some(turf_dev_settings) = TURF_DEV_SETTINGS.get().or(TURF_SETTINGS.get()) {
                Ok(turf_dev_settings.clone())
            } else {
                let turf_dev_settings = Self::dev_from_cargo_manifest_metadata()?
                    .or(Self::prod_from_cargo_manifest_metadata()?)
                    .unwrap_or(Self::default());

                TURF_DEV_SETTINGS.set(turf_dev_settings.clone()).expect(
                    "internal turf-dev settings have already been set, but should be empty",
                );

                if turf_dev_settings.debug_enabled() {
                    println!("{:#?}", &turf_dev_settings);
                }

                Ok(turf_dev_settings)
            }
        } else if let Some(turf_settings) = TURF_SETTINGS.get() {
            Ok(turf_settings.clone())
        } else {
            let turf_settings =
                Self::prod_from_cargo_manifest_metadata()?.unwrap_or(Self::default());

            TURF_SETTINGS
                .set(turf_settings.clone())
                .expect("internal turf settings have already been set, but should be empty");

            if turf_settings.debug_enabled() {
                println!("{:#?}", &turf_settings);
            }

            Ok(turf_settings)
        }
    }

    fn dev_from_cargo_manifest_metadata() -> Result<Option<Self>, crate::Error> {
        let manifest_data = crate::manifest::cargo_manifest()?;

        Ok(manifest_data
            .package
            .and_then(|package| package.metadata)
            .and_then(|metadata| metadata.turf_dev))
    }

    fn prod_from_cargo_manifest_metadata() -> Result<Option<Self>, crate::Error> {
        let manifest_data = crate::manifest::cargo_manifest()?;

        Ok(manifest_data
            .package
            .and_then(|package| package.metadata)
            .and_then(|metadata| metadata.turf))
    }
}
