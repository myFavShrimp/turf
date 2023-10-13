use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use crate::settings::FileOutput;

#[cfg(not(feature = "once_cell"))]
static DIRS_RESET: std::sync::OnceLock<()> = std::sync::OnceLock::new();
#[cfg(feature = "once_cell")]
static DIRS_RESET: once_cell::sync::OnceCell<()> = once_cell::sync::OnceCell::new();

pub fn perform_css_file_output(
    output_paths: FileOutput,
    style: &str,
    current_scss_path: &PathBuf,
) -> Result<(), crate::Error> {
    if DIRS_RESET.get().is_none() {
        if let Some(path) = &output_paths.global_css_file_path {
            if let Err(error) = std::fs::remove_file(path) {
                match error.kind() {
                    std::io::ErrorKind::NotFound => {}
                    _ => Err(error)?,
                }
            };

            create_dir_all(&path.parent().expect("global css file path has parent dir"))?;
        }
        if let Some(path) = &output_paths.separate_css_files_path {
            if let Err(error) = std::fs::remove_dir_all(path) {
                match error.kind() {
                    std::io::ErrorKind::NotFound => {}
                    _ => Err(error)?,
                }
            };

            create_dir_all(&path)?;
        }

        DIRS_RESET
            .set(())
            .expect("internal turf state has already been set, but should be empty");
    }

    if let Some(output_path) = output_paths.separate_css_files_path {
        let mut output_path = output_path.clone();
        output_path.push(
            current_scss_path
                .file_name()
                .expect("current scss file exists"),
        );
        output_path.set_extension("css");

        let mut output_file = File::create(&output_path).unwrap();

        output_file
            .write_all(style.as_bytes())
            .map_err(|error| crate::Error::GlobalCssFileWriteError(output_path, error))?;
    }

    if let Some(output_path) = output_paths.global_css_file_path {
        let mut global_css_file = File::options()
            .create(true)
            .append(true)
            .open(&output_path)?;

        global_css_file
            .write_all(style.as_bytes())
            .map_err(|error| crate::Error::GlobalCssFileWriteError(output_path, error))?;
    }

    Ok(())
}
