//! You're probably looking for `turf` instead.

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
    #[error("error transforming css - {0}")]
    CssError(#[from] transformer::LightningcssError),
    #[error("error obtaining random id - {0}")]
    RandError(#[from] getrandom::Error),
}

fn random_seed() -> Result<u64, getrandom::Error> {
    let mut buf = [0u8; 8];
    getrandom::getrandom(&mut buf)?;
    Ok(u64::from_ne_bytes(buf))
}
