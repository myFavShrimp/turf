//! You're probably looking for `turf` instead.

pub mod macro_functions;
mod manifest;
mod path;
mod settings;
mod transformer;

use std::path::{Path, PathBuf};

pub use settings::Settings;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error parsing cargo manifest - {0}")]
    TomlParseError(#[from] toml::de::Error),
    #[error("error reading cargo manifest - {0}")]
    ManifestError(#[from] std::io::Error),
    #[error("error reading path '{1}' - {0}")]
    PathError(std::io::Error, PathBuf),
    #[error("error compiling scss file '{1}' - {0}")]
    GrassError(Box<grass::Error>, PathBuf),
    #[error("error transforming css - {0}")]
    CssError(#[from] transformer::LightningcssError),
    #[error("error obtaining random id - {0}")]
    RandError(#[from] getrandom::Error),
}

impl<P> From<(Box<grass::Error>, P)> for Error
where
    P: AsRef<Path> + std::fmt::Debug,
{
    fn from(value: (Box<grass::Error>, P)) -> Self {
        let canonicalized_path = value.1.as_ref().canonicalize();

        match canonicalized_path {
            Ok(path) => Error::GrassError(value.0, path),
            Err(e) => Error::PathError(e, value.1.as_ref().to_path_buf()),
        }
    }
}

fn compile_message(message: &str) {
    println!("🌱 turf [INFO]: {message}");
}
