use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Mutex,
};

use crate::{path::canonicalize, Settings};

fn style_sheet_with_compile_options<P>(
    path: P,
    settings: Settings,
) -> Result<(String, HashMap<String, String>), crate::Error>
where
    P: AsRef<Path> + std::fmt::Debug,
{
    if path.as_ref() == Path::new("") {
        return Err(crate::Error::NoInputFileError);
    };

    let path = canonicalize(path);
    let css = grass::from_path(&path, &settings.clone().into())
        .map_err(|e| crate::Error::from((e, path)))?;
    crate::transformer::transform_stylesheet(&css, settings)
}

pub fn style_sheet<P>(path: P) -> Result<(String, HashMap<String, String>), crate::Error>
where
    P: AsRef<Path> + std::fmt::Debug,
{
    let settings = Settings::get()?;
    let (style_sheet, class_names) = style_sheet_with_compile_options(path, settings)?;

    Ok((style_sheet, class_names))
}

static LOAD_PATHS_TRACKED: std::sync::OnceLock<Mutex<bool>> = std::sync::OnceLock::new();

pub fn get_untracked_load_paths() -> Result<Vec<PathBuf>, crate::Error> {
    let load_paths_tracked_mutex = LOAD_PATHS_TRACKED.get_or_init(|| Mutex::new(false)).clone();
    let mut load_paths_tracked = match load_paths_tracked_mutex.lock() {
        Err(_) => return Err(crate::Error::MutexError),
        Ok(val) => val,
    };

    if *load_paths_tracked {
        println!(" ==== TRACKED ====");
        Ok(Vec::new())
    } else {
        println!(" ==== UNTRACKED ====");
        let settings = Settings::get()?;
        *load_paths_tracked = true;

        let result = settings.load_paths.unwrap_or(Vec::new()).into_iter().fold(
            Vec::new(),
            |mut acc, path| {
                acc.extend(get_file_paths_recusively(path).unwrap());
                acc
            },
        );
        // get_file_paths_recusively()

        Ok(result)
    }
}

fn get_file_paths_recusively(path: PathBuf) -> std::io::Result<Vec<PathBuf>> {
    use std::fs::read_dir;

    let mut result = Vec::new();

    for item in read_dir(path)? {
        let item_path = item?.path();

        if item_path.is_file() {
            let mut parent = PathBuf::from("..");
            parent.push(item_path);
            result.push(dbg!(parent));
        } else if item_path.is_dir() {
            result.extend(get_file_paths_recusively(item_path)?);
        }
    }

    Ok(result)
}
