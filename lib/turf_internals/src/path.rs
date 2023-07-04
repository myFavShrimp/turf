use std::path::{Path, PathBuf};

pub fn canonicalize<P>(path: P) -> PathBuf
where
    P: AsRef<Path>,
{
    let mut manifest_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    manifest_path.push(path);

    manifest_path
}
