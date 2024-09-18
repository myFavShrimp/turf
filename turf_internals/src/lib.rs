//! You're probably looking for `turf` instead.

mod css_compilation;
mod file_output;
mod hashing;
mod manifest;
mod path_utils;
mod settings;
mod transformer;

use std::{collections::HashMap, path::PathBuf, sync::Mutex};

pub use settings::Settings;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    CssCompilation(#[from] css_compilation::CssCompilationError),
    #[error(transparent)]
    Hashing(#[from] hashing::HashingError),
    #[error("error transforming css - {0}")]
    CssTransformation(#[from] transformer::TransformationError),
    #[error("no input file was specified")]
    NoInputFile,
    #[error(transparent)]
    PathResolution(#[from] path_utils::PathResolutionError),

    #[error(transparent)]
    CssFileWrite(#[from] file_output::CssFileWriteError),
    #[error(transparent)]
    Settings(#[from] settings::SettingsError),
}

fn compile_message(message: &str) {
    println!("🌱 turf [INFO]: {message}");
}

#[derive(Debug)]
pub enum StyleSheetKind {
    File(PathBuf),
    Inline(String),
}

#[derive(Debug)]
pub struct CompiledStyleSheet {
    pub css: String,
    pub class_names: HashMap<String, String>,
    pub original_style_sheet: StyleSheetKind,
}

fn style_sheet_with_compile_options(
    style_sheet_input: StyleSheetKind,
    settings: Settings,
) -> Result<CompiledStyleSheet, crate::Error> {
    let hash = hashing::hash_style_sheet(&style_sheet_input, &settings)?;
    let css = css_compilation::compile_style_sheet(&style_sheet_input, &settings)?;

    let (style_sheet_css, class_names) =
        transformer::transform_stylesheet(&css, &hash, settings.clone())?;

    if let Some(file_output) = settings.file_output {
        file_output::perform_css_file_output(file_output, &style_sheet_css, &style_sheet_input)?;
    }

    Ok(CompiledStyleSheet {
        css: style_sheet_css,
        class_names,
        original_style_sheet: style_sheet_input,
    })
}

pub fn style_sheet(style_sheet: StyleSheetKind) -> Result<CompiledStyleSheet, crate::Error> {
    let settings = Settings::get()?;

    let style_sheet = match style_sheet {
        StyleSheetKind::File(path) => {
            if path == PathBuf::from("") {
                return Err(crate::Error::NoInputFile);
            };
            let canonicalized_path = path_utils::canonicalize(path)?;
            StyleSheetKind::File(canonicalized_path)
        }
        StyleSheetKind::Inline(inline_style_sheet) => StyleSheetKind::Inline(inline_style_sheet),
    };

    style_sheet_with_compile_options(style_sheet, settings)
}

static LOAD_PATHS_TRACKED: Mutex<bool> = Mutex::new(false);

#[derive(Debug, thiserror::Error)]
pub enum LoadPathTrackingError {
    #[error("Could not read internal state")]
    Mutex,
    #[error(transparent)]
    Settings(#[from] settings::SettingsError),
    #[error(transparent)]
    PathResolution(#[from] path_utils::PathResolutionError),
}

pub fn get_untracked_load_paths() -> Result<Vec<PathBuf>, LoadPathTrackingError> {
    let mut load_paths_tracked = match LOAD_PATHS_TRACKED.lock() {
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
