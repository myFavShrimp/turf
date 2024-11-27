use base64::prelude::*;

use crate::StyleSheetKind;

#[derive(thiserror::Error, Debug)]
#[error("Failed to hash style sheet")]
pub enum HashingError {
    FileRead(#[from] std::io::Error),
}

pub fn hash_style_sheet(style_sheet: &StyleSheetKind) -> Result<String, HashingError> {
    let hash = match style_sheet {
        StyleSheetKind::File(ref path) => blake3::hash(&std::fs::read(path).unwrap()).to_hex(),
        StyleSheetKind::Inline(ref style_sheet) => {
            blake3::hash(&std::fs::read(style_sheet).unwrap()).to_hex()
        }
    };

    Ok(BASE64_URL_SAFE_NO_PAD.encode(hash.to_string()))
}
