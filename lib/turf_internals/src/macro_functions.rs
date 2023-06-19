use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

fn style_sheet_with_compile_options<P>(
    path: P,
    settings: crate::Settings,
) -> Result<(String, HashMap<String, String>), crate::Error>
where
    P: AsRef<Path> + std::fmt::Debug,
{
    let css = grass::from_path(&path, &settings.clone().into()).map_err(|e| {
        let mut manifest_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        manifest_path.push(path);

        crate::Error::from((e, manifest_path))
    })?;
    crate::transformer::transform_stylesheet(&css, settings)
}

pub fn style_sheet_with_default_compile_options<P>(
    path: P,
) -> Result<(String, HashMap<String, String>), crate::Error>
where
    P: AsRef<Path> + std::fmt::Debug,
{
    style_sheet_with_compile_options(path, crate::Settings::default())
}

pub fn style_sheet<P>(path: P) -> Result<(String, HashMap<String, String>), crate::Error>
where
    P: AsRef<Path> + std::fmt::Debug,
{
    let settings = crate::settings::Settings::get()?;
    style_sheet_with_compile_options(path, settings)
}
