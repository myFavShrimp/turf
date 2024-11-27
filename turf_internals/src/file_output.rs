use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use crate::{settings::FileOutput, StyleSheetKind};

static DIRS_RESET: std::sync::OnceLock<()> = std::sync::OnceLock::new();

#[derive(Debug, thiserror::Error)]
#[error("error writing css file '{0}' - {1}")]
pub struct CssFileWriteError(PathBuf, std::io::Error);

fn reset_file_output(output_paths: &FileOutput) -> Result<(), CssFileWriteError> {
    if let Some(path) = &output_paths.global_css_file_path {
        if let Err(error) = std::fs::remove_file(path) {
            match error.kind() {
                std::io::ErrorKind::NotFound => {}
                _ => Err(CssFileWriteError(path.clone(), error))?,
            }
        };

        create_dir_all(path.parent().expect("global css file path has parent dir"))
            .map_err(|error| CssFileWriteError(path.clone(), error))?;
    }
    if let Some(path) = &output_paths.separate_css_files_path {
        if let Err(error) = std::fs::remove_dir_all(path) {
            match error.kind() {
                std::io::ErrorKind::NotFound => {}
                _ => Err(CssFileWriteError(path.clone(), error))?,
            }
        };

        create_dir_all(path).map_err(|error| CssFileWriteError(path.clone(), error))?;
    }

    Ok(())
}

fn append_to_separate_file(
    style: &str,
    mut separate_files_dir: PathBuf,
    style_sheet: &StyleSheetKind,
) -> Result<(), CssFileWriteError> {
    match style_sheet {
        StyleSheetKind::File(path) => {
            separate_files_dir.push(path.file_name().expect("current css file exists"));
            separate_files_dir.set_extension("css");
        }
        StyleSheetKind::Inline(style_sheet) => {
            let hash = xxhash_rust::xxh3::xxh3_64(style_sheet.as_bytes());
            separate_files_dir.push(&format!("{hash:x?}.css"));
        }
    };

    let mut output_file = File::options()
        .create(true)
        .append(true)
        .open(&separate_files_dir)
        .map_err(|error| CssFileWriteError(separate_files_dir.clone(), error))?;

    output_file
        .write_all(style.as_bytes())
        .map_err(|error| CssFileWriteError(separate_files_dir, error))?;

    Ok(())
}

fn append_to_global_file(style: &str, global_file_path: &PathBuf) -> Result<(), CssFileWriteError> {
    let mut global_css_file = File::options()
        .create(true)
        .append(true)
        .open(global_file_path)
        .map_err(|error| CssFileWriteError(global_file_path.clone(), error))?;

    global_css_file
        .write_all(style.as_bytes())
        .map_err(|error| CssFileWriteError(global_file_path.clone(), error))?;

    Ok(())
}

pub fn perform_css_file_output(
    output_paths: FileOutput,
    style: &str,
    style_sheet_kind: &StyleSheetKind,
) -> Result<(), CssFileWriteError> {
    if DIRS_RESET.get().is_none() {
        reset_file_output(&output_paths)?;

        DIRS_RESET
            .set(())
            .expect("internal turf state has already been set, but should be empty");
    }

    if let Some(output_path) = output_paths.separate_css_files_path {
        append_to_separate_file(style, output_path, style_sheet_kind)?;
    }

    if let Some(output_path) = output_paths.global_css_file_path {
        append_to_global_file(style, &output_path)?;
    }

    Ok(())
}
