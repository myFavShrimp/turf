# turf 🌱

> **Warning** | The repository reflects the current development state, which may differ from the officially released version. While the repository provides insights into ongoing development and potential upcoming features, it may not necessarily represent the release available through crates.io.

`turf` allows you to build SCSS to CSS during compile time and inject those styles into your binary.

[![Crates.io][crates-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
[![Build Status][actions-badge]][actions-url]
[![MIT licensed][lic-badge]][lic-url]

[crates-badge]: https://img.shields.io/crates/v/turf.svg
[crates-url]: https://crates.io/crates/turf
[docs-badge]: https://img.shields.io/docsrs/turf/latest.svg?logo=docsdotrs&label=docs.rs
[docs-url]: https://docs.rs/turf
[actions-badge]: https://github.com/myFavShrimp/turf/actions/workflows/rust-ci.yml/badge.svg
[actions-url]: https://github.com/myFavShrimp/turf/actions/workflows/rust-ci.yml
[lic-url]: https://github.com/myFavShrimp/turf/blob/master/LICENSE
[lic-badge]: https://img.shields.io/badge/license-MIT-blue.svg

**turf will:**

- 🌿 transform your SCSS files into CSS with [grass](https://github.com/connorskees/grass/), right at compilation time
- 🪴 generate unique and dynamic class names for your CSS during compilation
- 🔬 minify and optimize your CSS using [lightningcss](https://github.com/parcel-bundler/lightningcss), ensuring compatibility with various browser targets
- 🎨 inject the generated CSS into your binary, guaranteeing quick access to your styles whenever you need them

## Usage

For a complete runnable example project, you can check out one of the examples:

| [leptos-example](https://github.com/myFavShrimp/turf/tree/main/examples/leptos-example) | [yew-example](https://github.com/myFavShrimp/turf/tree/main/examples/yew-example) | [dioxus-example](https://github.com/myFavShrimp/turf/tree/main/examples/dioxus-example) | [axum-askama-htmx](https://github.com/myFavShrimp/turf/tree/main/examples/axum-askama-htmx) |
| --------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |

**1. Create SCSS styles for your application**

```scss
// file at scss/file/path.scss

.TopLevelClass {
    color: red;

    .SomeClass {
        color: blue;
    }
}
```

**2. Use the `style_sheet` macro to include the resulting CSS in your code**

```rust,ignore
turf::style_sheet!("scss/file/path.scss");
```

The macro from the above example will expand to the following code:

```rust
static STYLE_SHEET: &'static str = "<style_sheet>";
struct ClassName;
impl ClassName {
    pub const TOP_LEVEL_CLASS: &'static str = "<unique_class_name>";
    pub const SOME_CLASS: &'static str = "<another_unique_class_name>";
}
```

**3. Use the `ClassName` struct and its associated constants to access the generated class names**

```rust,ignore
let top_level_class_name = ClassName::TOP_LEVEL_CLASS;
let some_class_name = ClassName::SOME_CLASS;
```

### Configuration

The configuration for turf can be specified in the Cargo.toml file using the `[package.metadata.turf]` and `[package.metadata.turf-dev]` keys. This allows you to conveniently manage your SCSS compilation settings for both development and production builds within your project's manifest.

Both profiles offer the exact same configuration options. However, if you haven't specified a `[package.metadata.turf-dev]` profile, the `[package.metadata.turf]` settings will also be applied to debug builds. This ensures consistency in the compilation process across different build types unless you explicitly define a separate configuration for the development profile.

Example configuration:

```toml
[package.metadata.turf]
minify = true
load_paths = ["path/to/scss/files", "path/to/other/scss/files"]

[package.metadata.turf.class_names]
template = "custom-<id>-<original_name>"
excludes = ["exclude-this-class-please", "^abc-[123]{4}"]

[package.metadata.turf.browser_targets]
chrome = [80, 1, 2]
firefox = 65
safari = [12, 3]

[package.metadata.turf.file_output]
global_css_file_path = "path/to/global.css"
separate_css_files_path = "dir/for/separate/css/"
```

The following configuration options are available:

- `minify` (default: `true`): Specifies whether the generated CSS should be minified or not. If set to true, the CSS output will be compressed and optimized for reduced file size. If set to false, the CSS output will be formatted with indentation and line breaks for improved readability.

- `load_paths`: Specifies additional paths to search for SCSS files to include during compilation. It accepts a list of string values, where each value represents a directory path to be included. This option allows you to import SCSS files from multiple directories.

- `browser_targets`: Defines the target browser versions for compatibility when generating CSS. It expects a structure that contains specific versions for different browsers. Each browser can have its own version specified.

- `class_names`: Allows configuration of the CSS class name generation. It expects a structure that contains two values for generating CSS class names and excluding class names from the uniquification process.

- `debug` (default: `false`): When set to true, this option will enable debug output of the read configuration and the generated CSS class names. This can be helpful for troubleshooting and understanding how the CSS is being generated.

- `file_output`: Enables output of compiled CSS. It expects a structure that contains two values for a single global CSS file or separate CSS files for each compiled SCSS file.

#### The `class_names` Key

- `template` (default: `"class-<id>"`): Specifies the template for generating randomized CSS class names. The template can include placeholders to customize the output:
    - `<id>` will be replaced with a unique identifier for each CSS class name
    - `<original_name>` will be replaced with the original class name from the SCSS file
    - `<name_hash>` will be replaced with the hash of the original class name from the SCSS file
    - `<name_hash_short>` will be replaced with the first 5 characters of the hash of the original class name from the SCSS file
    - `<style_sheet_hash>` will be replaced with the hash of the SCSS file
    - `<style_sheet_hash_short>` will be replaced with the first 8 characters of the hash of the SCSS file

- `excludes`: An array of regex patterns that exclude class names in your SCSS files from the class name uniquification process.

#### The `file_output` Key

- `global_css_file_path`: Specifies the file path for a global CSS file. If set, a CSS file will be created at the provided path, and all compiled styles will be written to this file. This allows you to have a single CSS file containing all the compiled styles.

- `separate_css_files_path`: Specifies the directory path for separate CSS files. If set, all compiled CSS files will be saved in the specified directory. Each compiled SCSS file will have its corresponding CSS file in this directory, allowing for modular CSS management. The file name for inline SCSS style definitions will be a 64 bit hash that is computed from the original SCSS style.

#### Browser Versions

The available browsers are as follows:

- android
- chrome
- edge
- firefox
- ie
- ios_saf
- opera
- safari
- samsung

#### Browser Version Format

Three formats are supported:

| major | major.minor | major.minor.patch |
| :---- | :---------- | :---------------- |
| Use a single integer to specify the major version number. | Use an array `[major, minor]` to specify both the major and minor version numbers. | Use an array `[major, minor, patch]` to specify the major, minor, and patch version numbers. |
| Example: `1` or `[1]` represent version `1.0.0` | Example: `[1, 2]` represents version `1.2.0` | Example: `[1, 2, 3]` represents version `1.2.3`. |

### Additional Macros

turf provides a few additional macros for other use cases.

#### The `style_sheet_values` Macro

In some cases, it may be necessary to have a struct's instance to access the class names (for example when using turf in [askama](https://github.com/djc/askama) templates).
The `turf::style_sheet_values` macro provides an alternative to directly including the resulting CSS and obtaining the associated class names. It returns a tuple of `(style_sheet: &'static str, class_names: struct)`.

**Usage:**

```rust,ignore
let (style_sheet, class_names) = turf::style_sheet_values!("path/to/style.scss");
let some_class_name = class_names.some_class;
```

#### The `inline_style_sheet` Macro

If you don't want your style sheet to live in another file, you can use the `turf::inline_style_sheet` macro. It allows you to write inline SCSS which will then be compiled to CSS.

**Usage:**

```rust,ignore
turf::inline_style_sheet! {
    .TopLevelClass {
        color: red;

        .SomeClass {
            color: blue;
        }
    }
}

// ...

let some_class_name = ClassName::SOME_CLASS;
```

#### The `inline_style_sheet_values` Macro

This macro combines the functionality of both the `style_sheet_values` and `inline_style_sheet` macros. It allows you to write inline SCSS and returns an tuple of `(style_sheet: &'static str, class_names: struct)`.

**Usage:**

```rust,ignore
let (style_sheet, class_names) = turf::inline_style_sheet_values! {
    .TopLevelClass {
        color: red;

        .SomeClass {
            color: blue;
        }
    }
};
let some_class_name = class_names.some_class;
```

## Contributions

Contributions to turf are always welcome! Whether you have ideas for new features or improvements, don't hesitate to open an issue or submit a pull request. 🤝

## License

turf is licensed under the MIT license. For more details, please refer to the LICENSE file. 📄
