use std::path::PathBuf;

use base64::prelude::*;

use crate::{Settings, StyleSheetKind};

#[derive(thiserror::Error, Debug)]
pub enum HashingError {
    #[error("error representing file path '{0}' as str")]
    PathStrRepresentation(PathBuf),
    #[error("error hashing file '{1}' - {0}")]
    FileHashing(Box<dyn std::error::Error + 'static + Send + Sync>, PathBuf),
    #[error("hashed metadata of file '{0}' could not be generated")]
    HashMetadataGenerationFailure(PathBuf),
}

pub fn hash_style_sheet(
    style_sheet: &StyleSheetKind,
    settings: &Settings,
) -> Result<String, HashingError> {
    Ok(match style_sheet {
        StyleSheetKind::File(ref path) => {
            let snapshot = filesystem_hashing::create_snapshot(
                path.to_str()
                    .ok_or(HashingError::PathStrRepresentation(path.clone()))?,
                filesystem_hashing::hasher::HashType::BLAKE3,
                vec![],
                settings.debug,
            )
            .map_err(|err| HashingError::FileHashing(err.into(), path.clone()))?;
            let file_hashes = snapshot
                .file_hashes
                .lock()
                .expect("this lock should never be poisoned");
            let file_metadata = file_hashes
                .values()
                .next()
                .ok_or(HashingError::HashMetadataGenerationFailure(path.clone()))?;
            BASE64_URL_SAFE_NO_PAD.encode(&file_metadata.check_sum)
        }
        StyleSheetKind::Inline(ref style_sheet) => {
            BASE64_URL_SAFE_NO_PAD.encode(blake3::hash(style_sheet.as_bytes()).as_bytes())
        }
    })
}
