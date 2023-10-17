# turf 🌱

> **Warning** | The repository might reflect the current development state, which may differ from the officially released version. While the repository provides insights into ongoing development and potential upcoming features, it may not necessarily represent the release available through crates.io or other distribution channels.

`turf` allows you to build SCSS to CSS during compile time and inject those styles into your binary.

[![Rust 1.70.0][rust-version-badge]][rust-version-url]
[![Crates.io][crates-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
[![Build Status][actions-badge]][actions-url]
[![MIT licensed][lic-badge]][lic-url]

[rust-version-badge]: https://img.shields.io/badge/Rust-1.70.0-orange?logo=rust
[rust-version-url]: https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html
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

### 1. Create SCSS styles for your application

```scss
// file at scss/file/path.scss

.TopLevelClass {
    color: red;

    .SomeClass {
        color: blue;
    }
}
```

### 2. Use the `style_sheet` macro to include the resulting CSS in your code

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

To access the generated class names, use the `ClassName` struct and its associated constants:

```rust,ignore
let top_level_class_name = ClassName::TOP_LEVEL_CLASS;
let some_class_name = ClassName::SOME_CLASS;
```

### 3. Configuration

The configuration for turf can be specified in the Cargo.toml file using the `[package.metadata.turf]` and `[package.metadata.turf-dev]` keys. This allows you to conveniently manage your SCSS compilation settings for both development and production builds within your project's manifest.

Both profiles offer the exact same configuration options. However, if you haven't specified a `[package.metadata.turf-dev]` profile, the `[package.metadata.turf]` settings will also be applied to debug builds. This ensures consistency in the compilation process across different build types unless you explicitly define a separate configuration for the development profile.

Example configuration:

```toml
[package.metadata.turf]
minify = true
load_paths = ["path/to/scss/files", "path/to/other/scss/files"]
class_name_template = "custom-<id>-<original_name>"

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

- `browser_targets`: Defines the target browser versions for compatibility when generating CSS. It expects a structure that includes specific versions for different browsers. Each browser can have its own version specified.

- `class_name_template` (default: `"class-<id>"`): Specifies the template for generating randomized CSS class names. The template can include placeholders to customize the output. `<id>` will be replaced with a unique identifier for each CSS class name and `<original_name>` will be replaced with the original class name from the SCSS file.

- `debug` (default: `false`): When set to true, this option will enable debug output of the read configuration and the generated CSS class names. This can be helpful for troubleshooting and understanding how the CSS is being generated.

- `file_output`: Enables output of compiled CSS. It expects a structure that includes two values for a single global CSS file or separate CSS files for each compiled SCSS file.

#### 3.1 File Output

- `global_css_file_path`: Specifies the file path for a global CSS file. If set, a CSS file will be created at the provided path, and all compiled styles will be written to this file. This allows you to have a single CSS file containing all the compiled styles.

- `separate_css_files_path`: Specifies the directory path for separate CSS files. If set, all compiled CSS files will be saved in the specified directory. Each compiled SCSS file will have its corresponding CSS file in this directory, allowing for modular CSS management.

#### 3.2 Browser Versions

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

#### 3.3 Browser Version Format

Three formats are supported:

| major | major.minor | major.minor.patch |
| :---- | :---------- | :---------------- |
| Use a single integer to specify the major version number. | Use an array `[major, minor]` to specify both the major and minor version numbers. | Use an array `[major, minor, patch]` to specify the major, minor, and patch version numbers. |
| Example: `1` or `[1]` represent version `1.0.0` | Example: `[1, 2]` represents version `1.2.0` | Example: `[1, 2, 3]` represents version `1.2.3`. |

#### 3.4 The `turf::inline_style_sheet` Macro

In some cases, it may be necessary to have a struct's instance (for example when using turf in [askama](https://github.com/djc/askama) templates).
The `turf_macros::inline_style_sheet` macro provides an alternative to directly including the resulting CSS and obtaining the associated class names. It returns a tuple of `(style_sheet: &'static str, class_names: struct)`.

**Usage:**

```rust,ignore
let (style_sheet, class_names) = turf::inline_style_sheet!("path/to/style.scss");
let some_class_name = class_names.some_class;
```

#### 3.5 Features

| Feature | Description |
| ------------ | ----------- |
| `once_cell` | As of 0.4.0, the minimum supported Rust version is 1.70.0 by default. This can be circumvented by using the `once_cell` feature flag, which will lower the minimum supported version to 1.64.0. |

## Contributions

Contributions to turf are always welcome! Whether you have ideas for new features or improvements, don't hesitate to open an issue or submit a pull request. 🤝

## License

turf is licensed under the MIT license. For more details, please refer to the LICENSE file. 📄
