use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    output_style: OutputStyle,
    load_paths: Vec<PathBuf>,
}

#[derive(Deserialize)]
pub enum OutputStyle {
    Expanded,
    Compressed,
}
