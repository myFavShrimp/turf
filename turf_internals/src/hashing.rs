use crate::StyleSheetKind;

#[derive(thiserror::Error, Debug)]
#[error("Failed to hash style sheet")]
pub enum StyleSheetHashingError {
    FileRead(#[from] std::io::Error),
}

pub fn hash_style_sheet(style_sheet: &StyleSheetKind) -> Result<String, StyleSheetHashingError> {
    let hash = match style_sheet {
        StyleSheetKind::File(ref path) => xxhash_rust::xxh3::xxh3_128(&std::fs::read(path)?),
        StyleSheetKind::Inline(ref style_sheet) => {
            xxhash_rust::xxh3::xxh3_128(style_sheet.as_bytes())
        }
    };

    Ok(format!("{hash:x}"))
}
