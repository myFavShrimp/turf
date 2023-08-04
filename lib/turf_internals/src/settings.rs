use std::path::PathBuf;

use serde::Deserialize;

use crate::path::canonicalize;

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

    pub fn canonicalized_load_paths(&self) -> Result<Vec<PathBuf>, crate::PathResolutionError> {
        dbg!(self
            .load_paths
            .clone()
            .unwrap_or(Default::default())
            .into_iter()
            .map(canonicalize)
            .collect())
    }
}

impl<'a> TryFrom<Settings> for grass::Options<'a> {
    type Error = crate::PathResolutionError;

    fn try_from(val: Settings) -> Result<Self, crate::PathResolutionError> {
        Ok(grass::Options::default()
            .style(grass::OutputStyle::Expanded)
            .load_paths(&val.canonicalized_load_paths()?))
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
            random_number_generator: oorandom::Rand32::new(random_seed()?),
            class_name_template: value
                .class_name_template
                .clone()
                .unwrap_or(String::from("class-<id>")),
        })
    }
}

fn random_seed() -> Result<u64, getrandom::Error> {
    let mut buf = [0u8; 8];
    getrandom::getrandom(&mut buf)?;
    Ok(u64::from_ne_bytes(buf))
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

#[cfg(not(feature = "once_cell"))]
static TURF_SETTINGS: std::sync::OnceLock<Settings> = std::sync::OnceLock::new();
#[cfg(not(feature = "once_cell"))]
static TURF_DEV_SETTINGS: std::sync::OnceLock<Settings> = std::sync::OnceLock::new();

#[cfg(feature = "once_cell")]
static TURF_SETTINGS: once_cell::sync::OnceCell<Settings> = once_cell::sync::OnceCell::new();
#[cfg(feature = "once_cell")]
static TURF_DEV_SETTINGS: once_cell::sync::OnceCell<Settings> = once_cell::sync::OnceCell::new();

impl Settings {
    pub fn get() -> Result<Self, crate::Error> {
        let dev_settings = Self::dev_profile_settings()?;
        let prod_settings = Self::prod_profile_settings()?;

        Ok(Self::choose_settings(
            dev_settings,
            prod_settings,
            cfg!(debug_assertions),
        ))
    }

    fn choose_settings(
        dev: Option<Settings>,
        prod: Option<Settings>,
        is_debug_build: bool,
    ) -> Self {
        if let (Some(cfg), true) = (dev.or(prod.clone()), is_debug_build) {
            cfg
        } else if let (Some(cfg), false) = (prod, is_debug_build) {
            cfg
        } else {
            Settings::default()
        }
    }

    fn dev_profile_settings() -> Result<Option<Self>, crate::Error> {
        if let Some(turf_dev_settings) = TURF_DEV_SETTINGS.get() {
            return Ok(Some(turf_dev_settings.clone()));
        }

        let dev_settings_maybe = crate::manifest::cargo_manifest()?
            .package
            .and_then(|package| package.metadata)
            .and_then(|metadata| metadata.turf_dev);

        if let Some(turf_dev_settings) = dev_settings_maybe.clone() {
            TURF_DEV_SETTINGS
                .set(turf_dev_settings)
                .expect("internal turf-dev settings have already been set, but should be empty");
        }

        Ok(dev_settings_maybe)
    }

    fn prod_profile_settings() -> Result<Option<Self>, crate::Error> {
        if let Some(turf_prod_settings) = TURF_SETTINGS.get() {
            return Ok(Some(turf_prod_settings.clone()));
        }

        let prod_settings_maybe = crate::manifest::cargo_manifest()?
            .package
            .and_then(|package| package.metadata)
            .and_then(|metadata| metadata.turf);

        if let Some(turf_prod_settings) = prod_settings_maybe.clone() {
            TURF_SETTINGS
                .set(turf_prod_settings)
                .expect("internal turf settings have already been set, but should be empty");
        }

        Ok(prod_settings_maybe)
    }
}

#[cfg(test)]
mod debug_tests {
    use super::Settings;

    #[test]
    fn use_dev_settings_for_debug_build() {
        let mut dev_settings = Settings::default();
        dev_settings.class_name_template = Some(String::from("abc"));

        let mut prod_settings = Settings::default();
        prod_settings.class_name_template = Some(String::from("def"));

        let selected_settings =
            Settings::choose_settings(Some(dev_settings.clone()), Some(prod_settings), true);

        assert_eq!(
            selected_settings.class_name_template,
            dev_settings.class_name_template
        );
    }

    #[test]
    fn use_prod_settings_for_debug_build_when_no_dev_settings_where_given() {
        let mut prod_settings = Settings::default();
        prod_settings.class_name_template = Some(String::from("def"));

        let selected_settings = Settings::choose_settings(None, Some(prod_settings.clone()), true);

        assert_eq!(
            selected_settings.class_name_template,
            prod_settings.class_name_template
        );
    }

    #[test]
    fn use_prod_settings_for_release_build() {
        let mut dev_settings = Settings::default();
        dev_settings.class_name_template = Some(String::from("abc"));

        let mut prod_settings = Settings::default();
        prod_settings.class_name_template = Some(String::from("def"));

        let selected_settings =
            Settings::choose_settings(Some(dev_settings), Some(prod_settings.clone()), false);

        assert_eq!(
            selected_settings.class_name_template,
            prod_settings.class_name_template
        );
    }

    #[test]
    fn do_not_use_dev_settings_for_release_build() {
        let mut dev_settings = Settings::default();
        dev_settings.class_name_template = Some(String::from("abc"));

        let selected_settings = Settings::choose_settings(Some(dev_settings.clone()), None, false);

        assert_ne!(
            selected_settings.class_name_template,
            dev_settings.class_name_template
        );
    }
}
