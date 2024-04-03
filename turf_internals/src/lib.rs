//! You're probably looking for `turf` instead.

mod file_output;
pub mod macro_functions;
mod manifest;
mod path;
mod settings;
mod transformer;

use std::path::PathBuf;

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
    PathResolutionError(#[from] path::PathResolutionError),

    #[error(transparent)]
    CssFileWriteError(#[from] file_output::CssFileWriteError),
    #[error(transparent)]
    Settings(#[from] SettingsError),
}

fn compile_message(message: &str) {
    println!("🌱 turf [INFO]: {message}");
}
