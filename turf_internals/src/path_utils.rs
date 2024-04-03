use std::path::{Path, PathBuf};

#[derive(thiserror::Error, Debug)]
#[error("error resolving path '{path}' - {source}")]
pub struct PathResolutionError {
    path: PathBuf,
    source: std::io::Error,
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

impl<P> From<(Box<grass::Error>, P)> for crate::Error
where
    P: AsRef<Path> + std::fmt::Debug,
{
    fn from(value: (Box<grass::Error>, P)) -> Self {
        let canonicalized_path = value.1.as_ref().canonicalize();

        match canonicalized_path {
            Ok(path) => crate::Error::GrassError(value.0, path),
            Err(e) => PathResolutionError {
                path: value.1.as_ref().to_path_buf(),
                source: e,
            }
            .into(),
        }
    }
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
