//! You're probably looking for `turf` instead.

mod file_output;
pub mod macro_functions;
mod manifest;
mod path;
mod settings;
mod transformer;

use std::path::{Path, PathBuf};

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
    PathResolutionError(#[from] PathResolutionError),

    #[error(transparent)]
    CssFileWriteError(#[from] file_output::CssFileWriteError),
    #[error(transparent)]
    Settings(#[from] SettingsError),
}

#[derive(thiserror::Error, Debug)]
#[error("error resolving path '{path}' - {source}")]
pub struct PathResolutionError {
    path: PathBuf,
    source: std::io::Error,
}

impl From<(PathBuf, std::io::Error)> for PathResolutionError {
    fn from(value: (PathBuf, std::io::Error)) -> Self {
        Self {
            path: value.0,
            source: value.1,
        }
    }
}

impl<P> From<(Box<grass::Error>, P)> for Error
where
    P: AsRef<Path> + std::fmt::Debug,
{
    fn from(value: (Box<grass::Error>, P)) -> Self {
        let canonicalized_path = value.1.as_ref().canonicalize();

        match canonicalized_path {
            Ok(path) => Error::GrassError(value.0, path),
            Err(e) => PathResolutionError {
                path: value.1.as_ref().to_path_buf(),
                source: e,
            }
            .into(),
        }
    }
}

fn compile_message(message: &str) {
    println!("🌱 turf [INFO]: {message}");
}
