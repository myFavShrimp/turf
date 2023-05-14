use std::path::Path;

fn style_sheet_with_compile_options<'a, P>(
    path: P,
    options: &grass::Options,
) -> Result<(String, String), crate::Error>
where
    P: AsRef<Path> + std::fmt::Debug,
{
    let css = grass::from_path(path, options)?;

    let style = stylist::Style::new(css)?;
    let class_name = style.get_class_name();
    let style_sheet = style.get_style_str();

    Ok((class_name.into(), style_sheet.into()))
}

pub fn style_sheet_with_default_compile_options<'a, P>(
    path: P,
) -> Result<(String, String), crate::Error>
where
    P: AsRef<Path> + std::fmt::Debug,
{
    style_sheet_with_compile_options(path, &grass::Options::default())
}

pub fn style_sheet<'a, P>(path: P) -> Result<(String, String), crate::Error>
where
    P: AsRef<Path> + std::fmt::Debug,
{
    let options = crate::settings::Settings::from_cargo_manifest_metadata_or_default()?;
    style_sheet_with_compile_options(path, &options.into())
}
