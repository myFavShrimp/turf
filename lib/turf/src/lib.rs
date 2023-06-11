//! # turf 🌱
//!
//! `turf` allows you to build SCSS to CSS during compile time and inject those styles into your binary.
//!
//! [![Crates.io][crates-badge]][crates-url]
//! [![Docs.rs][docs-badge]][docs-url]
//! [![Build Status][actions-badge]][actions-url]
//! [![MIT licensed][lic-badge]][lic-url]
//!
//! [crates-badge]: https://img.shields.io/crates/v/turf.svg?logo=docsdotrs
//! [crates-url]: https://crates.io/crates/turf
//! [docs-badge]: https://img.shields.io/docsrs/turf/latest.svg?logo=rust
//! [docs-url]: https://docs.rs/turf
//! [actions-badge]: https://github.com/myFavShrimp/turf/actions/workflows/rust.yml/badge.svg
//! [actions-url]: https://github.com/myFavShrimp/turf/actions/workflows/rust.yml
//! [lic-url]: https://github.com/myFavShrimp/turf/blob/master/LICENSE
//! [lic-badge]: https://img.shields.io/badge/license-MIT-blue.svg
//!
//! ## Features
//!
//! **turf will:**
//!
//! - 🌿 transform your SCSS files into CSS with [grass](https://github.com/connorskees/grass/), right at compilation time
//! - 🪴 generate unique and dynamic class names for your CSS during compilation
//! - 🔬 minify and optimize your CSS using [lightningcss](https://github.com/parcel-bundler/lightningcss), ensuring compatibility with various browser targets
//! - 🎨 inject the generated CSS into your binary, guaranteeing quick access to your styles whenever you need them
//!
//! ## Usage
//!
//! For a complete runnable example project, you can check out one of the examples:
//!
//! | [leptos-example](https://github.com/myFavShrimp/turf/tree/main/examples/leptos-example) | [yew-example](https://github.com/myFavShrimp/turf/tree/main/examples/yew-example) | [dioxus-example](https://github.com/myFavShrimp/turf/tree/main/examples/dioxus-example) |
//! | --------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
//!
//! ### 1. Create SCSS styles for your application.
//!
//! ```scss
//! // file at scss/file/path.scss
//!
//! .TopLevelClass {
//!     color: red;
//!
//!     .SomeClass {
//!         color: blue;
//!     }
//! }
//! ```
//!
//! ### 2. Use the `style_sheet` macro to include the resulting CSS in your code
//!
//! ```rust,ignore
//! turf::style_sheet!("scss/file/path.scss");
//! ```
//!
//! The macro from the above example will expand to the following code:
//!
//! ```rust
//! static STYLE_SHEET: &'static str = "<style_sheet>";
//! struct ClassName;
//! impl ClassName {
//!     pub const TOP_LEVEL_CLASS: &'static str = "<unique_class_name>";
//!     pub const SOME_CLASS: &'static str = "<another_unique_class_name>";
//! }
//! ```
//!
//! To access the generated class names, use the `ClassName` struct and its associated constants:
//!
//! ```rust,ignore
//! let top_level_class_name = ClassName::TOP_LEVEL_CLASS;
//! let some_class_name = ClassName::SOME_CLASS;
//! ```
//!
//! ### 3. Configuration
//!
//! The configuration for turf can be specified in the Cargo.toml file using the `[package.metadata.turf]` key. This allows you to conveniently manage your SCSS compilation settings within your project's manifest.
//!
//! Example configuration:
//!
//! ```toml
//! [package.metadata.turf]
//! minify = true
//! load_paths = ["path/to/scss/files", "path/to/other/scss/files"]
//! class_name_template = "custom-<id>-<original_name>"
//!
//! [package.metadata.turf.browser_targets]
//! chrome = [80, 81, 82]
//! firefox = 65
//! safari = [12, 13]
//! ```
//!
//! The following configuration options are available:
//!
//! - `minify` (default: `true`): Specifies whether the generated CSS should be minified or not. If set to true, the CSS output will be compressed and optimized for reduced file size. If set to false, the CSS output will be formatted with indentation and line breaks for improved readability.
//!
//! - `load_paths`: Specifies additional paths to search for SCSS files to include during compilation. It accepts a list of string values, where each value represents a directory path to be included. This option allows you to import SCSS files from multiple directories.
//!
//! - `browser_targets`: Defines the target browser versions for compatibility when generating CSS. It expects a structure that includes specific versions for different browsers. Each browser can have its own version specified.
//!
//! - `class_name_template` (default: `"class-<id>"`): Specifies the template for generating randomized CSS class names. The template can include placeholders to customize the output. `<id>` will be replaced with a unique identifier for each CSS class name and `<original_name>` will be replaced with the original class name from the SCSS file.
//!
//! #### 3.1 Browser Versions
//!
//! The available browsers are as follows:
//!
//! - android
//! - chrome
//! - edge
//! - firefox
//! - ie
//! - ios_saf
//! - opera
//! - safari
//! - samsung
//!
//! #### 3.2 Browser Version Format
//!
//! Three formats are supported:
//!
//! | major | major.minor | major.minor.patch |
//! | :---- | :---------- | :---------------- |
//! | Use a single integer to specify the major version number. | Use an array `[major, minor]` to specify both the major and minor version numbers. | Use an array `[major, minor, patch]` to specify the major, minor, and patch version numbers. |
//! | Example: `1` or `[1]` represent version `1` | Example: `[1, 2]` represents version `1.2` | Example: `[1, 2, 3]` represents version `1.2.3`. |

/// generates the static variable `STYLE_SHEET` and the `ClassName` struct with default settings or the settings specified in the `Cargo.toml`
pub use turf_macros::style_sheet;
