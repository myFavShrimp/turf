use std::{path::{PathBuf, Path}, fs::read_to_string};

use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("toml error")]
    TomlError(#[from] toml::de::Error),
    #[error("io error")]
    IoError(#[from] std::io::Error),
}

#[derive(Deserialize)]
pub struct Settings {
    output_style: OutputStyle,
    load_paths: Vec<PathBuf>,
}

impl<'a> Into<grass::Options<'a>> for Settings {
    fn into(self) -> grass::Options<'a> {
        grass::Options::default()
            .style(self.output_style.into())
            .load_paths(&self.load_paths)
    }
}

impl Settings {
    pub fn from_file<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        Ok(toml::de::from_str(&read_to_string(path)?)?)
    }
}

#[derive(Deserialize)]
pub enum OutputStyle {
    Expanded,
    Compressed,
}

impl Into<grass::OutputStyle> for OutputStyle {
    fn into(self) -> grass::OutputStyle {
        match self {
            OutputStyle::Expanded => grass::OutputStyle::Expanded,
            OutputStyle::Compressed => grass::OutputStyle::Compressed,
        }
    }
}
