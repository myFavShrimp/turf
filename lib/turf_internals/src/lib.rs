//! You're probably looking for `turf` instead.

mod classes_structure;
pub mod macro_functions;
mod manifest;
mod settings;
mod transformer;

pub use settings::Settings;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error parsing cargo manifest - {0}")]
    TomlParseError(#[from] toml::de::Error),
    #[error("error reading cargo manifest - {0}")]
    IoError(#[from] std::io::Error),
    #[error("error compiling scss - {0}")]
    GrassError(#[from] Box<grass::Error>),
    #[error("error reading compiled scss - {0}")]
    CssError(#[from] transformer::LightningcssError),
}
