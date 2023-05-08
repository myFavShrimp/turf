use std::path::PathBuf;

pub struct Settings {
    output_style: OutputStyle,
    load_paths: Vec<PathBuf>,
}

pub enum OutputStyle {
    Expanded,
    Compressed,
}
