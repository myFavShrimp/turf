mod manifest;
mod settings;

pub use settings::{Settings, OutputStyle};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("could not parse cargo manifest")]
    ParseError(#[from] toml::de::Error),
    #[error("error reading cargo manifest")]
    IoError(#[from] std::io::Error),
}

#[test]
fn dbg_dev_helper() {
    dbg!(settings::Settings::from_cargo_manifest_metadata());
    assert!(false);
}
