use std::path::{Path, PathBuf};

pub fn canonicalize<P>(path: P) -> PathBuf
where
    P: AsRef<Path>,
{
    let mut manifest_path = PathBuf::from(std::env::current_dir().unwrap());
    manifest_path.push(path);

    manifest_path
}
