use std::path::{Path, PathBuf};

#[derive(thiserror::Error, Debug)]
#[error("error resolving path '{path}' - {source}")]
pub struct PathResolutionError {
    pub(crate) path: PathBuf,
    pub(crate) source: std::io::Error,
}

impl From<(PathBuf, std::io::Error)> for PathResolutionError {
    fn from(value: (PathBuf, std::io::Error)) -> Self {
        Self {
            path: value.0,
            source: value.1,
        }
    }
}

pub fn canonicalize<P>(path: P) -> Result<PathBuf, PathResolutionError>
where
    P: AsRef<Path>,
{
    let mut canonicalized_path = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR environment variable"),
    );
    canonicalized_path.push(path.as_ref());

    std::fs::canonicalize(canonicalized_path.clone()).map_err(|e| (canonicalized_path, e).into())
}

pub fn get_file_paths_recusively(path: PathBuf) -> Result<Vec<PathBuf>, PathResolutionError> {
    use std::fs::read_dir;

    let path = canonicalize(path)?;
    let mut result = Vec::new();

    for item in read_dir(path.clone()).map_err(|e| (path.clone(), e))? {
        let item_path = item.map_err(|e| (path.clone(), e))?.path();

        if item_path.is_file() {
            result.push(canonicalize(item_path)?);
        } else if item_path.is_dir() {
            result.extend(get_file_paths_recusively(item_path)?);
        }
    }

    Ok(result)
}
