//! You're probably looking for `turf` instead.

pub mod macro_functions;
mod manifest;
mod settings;

pub use settings::{OutputStyle, Settings};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error parsing cargo manifest - {0}")]
    ParseError(#[from] toml::de::Error),
    #[error("error reading cargo manifest - {0}")]
    IoError(#[from] std::io::Error),
    #[error("error compiling scss - {0}")]
    GrassError(#[from] Box<grass::Error>),
    #[error("error reading compiled scss - {0}")]
}
