mod manifest;
mod settings;
pub mod macro_functions;

pub use settings::{Settings, OutputStyle};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error parsing cargo manifest - {0}")]
    ParseError(#[from] toml::de::Error),
    #[error("error reading cargo manifest - {0}")]
    IoError(#[from] std::io::Error),
    #[error("error compiling scss - {0}")]
    GrassError(#[from] Box<grass::Error>),
    #[error("error reading compiled scss - {0}")]
    StylistError(#[from] stylist::Error),
}

#[test]
fn dbg_dev_helper() {
    dbg!(settings::Settings::from_cargo_manifest_metadata());
    assert!(false);
}
