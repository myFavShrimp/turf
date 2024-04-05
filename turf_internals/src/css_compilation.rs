use std::path::{Path, PathBuf};

use crate::{path_utils, Settings, StyleSheetKind};

#[derive(thiserror::Error, Debug)]
pub enum CssCompilationError {
    #[error("error compiling scss file '{1}' - {0}")]
    File(Box<grass::Error>, PathBuf),
    #[error("error compiling inline scss '{0}'")]
    Inline(#[from] Box<grass::Error>),
    #[error(transparent)]
    PathResolutionError(#[from] path_utils::PathResolutionError),
}

impl<P> From<(Box<grass::Error>, P)> for CssCompilationError
where
    P: AsRef<Path> + std::fmt::Debug,
{
    fn from(value: (Box<grass::Error>, P)) -> Self {
        let canonicalized_path = value.1.as_ref().canonicalize();

        match canonicalized_path {
            Ok(path) => CssCompilationError::File(value.0, path),
            Err(e) => path_utils::PathResolutionError {
                path: value.1.as_ref().to_path_buf(),
                source: e,
            }
            .into(),
        }
    }
}

pub fn compile_style_sheet(
    style_sheet: &StyleSheetKind,
    settings: &Settings,
) -> Result<String, CssCompilationError> {
    Ok(match style_sheet {
        StyleSheetKind::File(ref path) => grass::from_path(path, &settings.clone().try_into()?)
            .map_err(|e| CssCompilationError::from((e, path.clone())))?,
        StyleSheetKind::Inline(ref style_sheet) => {
            grass::from_string(style_sheet, &settings.clone().try_into()?)?
        }
    })
}
