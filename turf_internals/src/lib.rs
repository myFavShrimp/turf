//! You're probably looking for `turf` instead.

mod file_output;
mod manifest;
mod path_utils;
mod settings;
mod transformer;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Mutex,
};

pub use settings::Settings;
use settings::SettingsError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error compiling scss file '{1}' - {0}")]
    GrassError(Box<grass::Error>, PathBuf),
    #[error("error transforming css - {0}")]
    CssError(#[from] transformer::TransformationError),
    #[error("no input file was specified")]
    NoInputFileError,
    #[error(transparent)]
    PathResolutionError(#[from] path_utils::PathResolutionError),

    #[error(transparent)]
    CssFileWriteError(#[from] file_output::CssFileWriteError),
    #[error(transparent)]
    Settings(#[from] SettingsError),
}

fn compile_message(message: &str) {
    println!("🌱 turf [INFO]: {message}");
}

pub enum StyleSheetKind {
    File(PathBuf),
    Inline(String),
}

pub struct CompiledStyleSheet {
    pub css: String,
    pub class_names: HashMap<String, String>,
    pub original_style_sheet: StyleSheetKind,
}

fn style_sheet_with_compile_options(
    style_sheet_input: StyleSheetKind,
    settings: Settings,
) -> Result<CompiledStyleSheet, crate::Error> {
    let css = match style_sheet_input {
        StyleSheetKind::File(ref path) => grass::from_path(&path, &settings.clone().try_into()?)
            .map_err(|e| crate::Error::from((e, path.clone())))?,
        StyleSheetKind::Inline(_) => todo!(),
    };

    let (style_sheet_css, class_names) = transformer::transform_stylesheet(&css, settings.clone())?;

    match style_sheet_input {
        StyleSheetKind::File(ref path) => {
            if let Some(file_output) = settings.file_output {
                file_output::perform_css_file_output(file_output, &style_sheet_css, &path)?;
            }
        }
        StyleSheetKind::Inline(_) => todo!(),
    };

    Ok(CompiledStyleSheet {
        css: style_sheet_css,
        class_names,
        original_style_sheet: style_sheet_input,
    })
}

pub fn style_sheet<P>(path: P) -> Result<CompiledStyleSheet, crate::Error>
where
    P: AsRef<Path> + std::fmt::Debug,
{
    let settings = Settings::get()?;
    if path.as_ref() == Path::new("") {
        return Err(crate::Error::NoInputFileError);
    };

    let canonicalized_path = path_utils::canonicalize(path)?;
    style_sheet_with_compile_options(StyleSheetKind::File(canonicalized_path), settings)
}

static LOAD_PATHS_TRACKED: std::sync::OnceLock<Mutex<bool>> = std::sync::OnceLock::new();

#[derive(Debug, thiserror::Error)]
pub enum LoadPathTrackingError {
    #[error("Could not read internal state")]
    Mutex,
    #[error(transparent)]
    Settings(#[from] SettingsError),
    #[error(transparent)]
    PathResolution(#[from] path_utils::PathResolutionError),
}

pub fn get_untracked_load_paths() -> Result<Vec<PathBuf>, LoadPathTrackingError> {
    let load_paths_tracked_mutex = LOAD_PATHS_TRACKED.get_or_init(|| Mutex::new(false));
    let mut load_paths_tracked = match load_paths_tracked_mutex.lock() {
        Err(_) => return Err(LoadPathTrackingError::Mutex),
        Ok(val) => val,
    };

    if *load_paths_tracked {
        Ok(Vec::new())
    } else {
        let settings = Settings::get()?;
        *load_paths_tracked = true;

        let mut result = Vec::new();

        for path in settings.load_paths {
            result.extend(path_utils::get_file_paths_recusively(path)?);
        }

        Ok(result)
    }
}
