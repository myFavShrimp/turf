use std::path::{Path, PathBuf};

pub fn canonicalize<P>(path: P) -> Result<PathBuf, crate::PathResolutionError>
where
    P: AsRef<Path>,
{
    let mut manifest_path = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR environment variable"),
    );
    manifest_path.push(path.as_ref().clone());

    std::fs::canonicalize(manifest_path).map_err(|e| (path.as_ref().to_path_buf(), e).into())
}
