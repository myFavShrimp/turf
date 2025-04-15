use std::path::PathBuf;

use serde::Deserialize;

use crate::{
    manifest::ManifestError,
    path_utils::{canonicalize, PathResolutionError},
};

#[derive(Deserialize, Debug, Default, Clone)]
pub struct FileOutput {
    pub(crate) global_css_file_path: Option<PathBuf>,
    pub(crate) separate_css_files_path: Option<PathBuf>,
}

pub(crate) static DEFAULT_CLASS_NAME_TEMPLATE: &str = "class-<id>";

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ClassNameGeneration {
    pub(crate) template: String,
    #[serde(default)]
    pub(crate) excludes: Vec<String>,
}

impl Default for ClassNameGeneration {
    fn default() -> Self {
        Self {
            template: DEFAULT_CLASS_NAME_TEMPLATE.to_owned(),
            excludes: vec![],
        }
    }
}

pub(crate) static DEFAULT_MINIFY: bool = true;

fn default_minify() -> bool {
    DEFAULT_MINIFY
}

#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    #[serde(default)]
    pub(crate) debug: bool,
    #[serde(default = "default_minify")]
    pub(crate) minify: bool,
    #[serde(default)]
    pub(crate) load_paths: Vec<PathBuf>,
    #[serde(default)]
    pub(crate) browser_targets: BrowserTargets,
    #[serde(default)]
    pub(crate) class_names: ClassNameGeneration,
    pub(crate) file_output: Option<FileOutput>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            debug: false,
            minify: DEFAULT_MINIFY,
            load_paths: Vec::new(),
            browser_targets: BrowserTargets(None),
            class_names: ClassNameGeneration::default(),
            file_output: None,
        }
    }
}

impl Settings {
    pub fn canonicalized_load_paths(&self) -> Result<Vec<PathBuf>, PathResolutionError> {
        self.load_paths
            .clone()
            .into_iter()
            .map(canonicalize)
            .collect()
    }
}

impl<'a> TryFrom<Settings> for grass::Options<'a> {
    type Error = PathResolutionError;

    fn try_from(val: Settings) -> Result<Self, PathResolutionError> {
        Ok(grass::Options::default()
            .style(grass::OutputStyle::Expanded)
            .load_paths(&val.canonicalized_load_paths()?))
    }
}

impl<'a> From<Settings> for lightningcss::printer::PrinterOptions<'a> {
    fn from(val: Settings) -> Self {
        lightningcss::printer::PrinterOptions {
            minify: val.minify,
            project_root: None,
            targets: val.browser_targets.0.into(),
            analyze_dependencies: None,
            pseudo_classes: None,
        }
    }
}

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(try_from = "RawBrowserTargets")]
pub struct BrowserTargets(pub Option<lightningcss::targets::Browsers>);

#[derive(Deserialize, Debug, Default, Clone)]
pub struct RawBrowserTargets(Vec<String>);

#[derive(Debug, thiserror::Error)]
#[error("Error reading browser_targets: {0:#?}")]
pub struct FromRawTargetsErrorCollection(Vec<FromRawTargetsError>);

#[derive(thiserror::Error)]
#[error("Failed to read browser target: {target:?} - {error}")]
pub struct FromRawTargetsError {
    target: String,
    error: String,
}

impl std::fmt::Debug for FromRawTargetsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl TryFrom<RawBrowserTargets> for BrowserTargets {
    type Error = FromRawTargetsErrorCollection;

    fn try_from(value: RawBrowserTargets) -> Result<Self, Self::Error> {
        let errors = value
            .0
            .iter()
            .filter_map(|target| {
                match lightningcss::targets::Browsers::from_browserslist([target]) {
                    Ok(_) => None,
                    Err(e) => Some(FromRawTargetsError {
                        error: e.to_string(),
                        target: target.clone(),
                    }),
                }
            })
            .collect::<Vec<_>>();

        if !errors.is_empty() {
            return Err(FromRawTargetsErrorCollection(errors));
        }

        Ok(
            lightningcss::targets::Browsers::from_browserslist(value.0.clone())
                .map(BrowserTargets)
                .unwrap(),
        )
    }
}

static TURF_SETTINGS: std::sync::OnceLock<Settings> = std::sync::OnceLock::new();
static TURF_DEV_SETTINGS: std::sync::OnceLock<Settings> = std::sync::OnceLock::new();

#[derive(Debug, thiserror::Error)]
#[error("Could not obtain turf settings from the Cargo manifest")]
pub struct SettingsError(#[from] ManifestError);

impl Settings {
    pub fn get() -> Result<Self, SettingsError> {
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

    fn dev_profile_settings() -> Result<Option<Self>, SettingsError> {
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

    fn prod_profile_settings() -> Result<Option<Self>, SettingsError> {
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
    use crate::settings::ClassNameGeneration;

    use super::Settings;

    #[test]
    fn use_dev_settings_for_debug_build() {
        let mut dev_settings = Settings::default();
        let class_name_generation = ClassNameGeneration {
            template: String::from("abc"),
            ..Default::default()
        };
        dev_settings.class_names = class_name_generation;

        let mut prod_settings = Settings::default();
        let class_name_generation = ClassNameGeneration {
            template: String::from("def"),
            ..Default::default()
        };
        prod_settings.class_names = class_name_generation;

        let selected_settings =
            Settings::choose_settings(Some(dev_settings.clone()), Some(prod_settings), true);

        assert_eq!(selected_settings.class_names, dev_settings.class_names);
    }

    #[test]
    fn use_prod_settings_for_debug_build_when_no_dev_settings_where_given() {
        let mut prod_settings = Settings::default();
        let class_name_generation = ClassNameGeneration {
            template: String::from("def"),
            ..Default::default()
        };
        prod_settings.class_names = class_name_generation;

        let selected_settings = Settings::choose_settings(None, Some(prod_settings.clone()), true);

        assert_eq!(selected_settings.class_names, prod_settings.class_names);
    }

    #[test]
    fn use_prod_settings_for_release_build() {
        let mut dev_settings = Settings::default();
        let class_name_generation = ClassNameGeneration {
            template: String::from("abc"),
            ..Default::default()
        };
        dev_settings.class_names = class_name_generation;

        let mut prod_settings = Settings::default();
        let class_name_generation = ClassNameGeneration {
            template: String::from("def"),
            ..Default::default()
        };
        prod_settings.class_names = class_name_generation;

        let selected_settings =
            Settings::choose_settings(Some(dev_settings), Some(prod_settings.clone()), false);

        assert_eq!(selected_settings.class_names, prod_settings.class_names);
    }

    #[test]
    fn do_not_use_dev_settings_for_release_build() {
        let mut dev_settings = Settings::default();
        let class_name_generation = ClassNameGeneration {
            template: String::from("abc"),
            ..Default::default()
        };
        dev_settings.class_names = class_name_generation;

        let selected_settings = Settings::choose_settings(Some(dev_settings.clone()), None, false);

        assert_ne!(selected_settings.class_names, dev_settings.class_names);
    }
}
