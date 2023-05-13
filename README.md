# turf 🌱

`turf` allows you to build SCSS to CSS during compile time and inject those styles into your binary.

[![Crates.io][crates-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
[![Build Status][actions-badge]][actions-url]
[![MIT licensed][lic-badge]][lic-url]

[crates-badge]: https://img.shields.io/crates/v/turf.svg?logo=docsdotrs
[crates-url]: https://crates.io/crates/turf
[docs-badge]: https://img.shields.io/docsrs/turf/latest.svg?logo=rust
[docs-url]: https://docs.rs/turf
[actions-badge]: https://github.com/myFavShrimp/turf/actions/workflows/rust.yml/badge.svg
[actions-url]: https://github.com/myFavShrimp/turf/actions/workflows/rust.yml
[lic-url]: https://github.com/myFavShrimp/turf/blob/master/LICENSE
[lic-badge]: https://img.shields.io/badge/license-MIT-blue.svg

## Features

**turf will:**

- 🌿 transform your SCSS files into CSS with [grass](https://github.com/connorskees/grass/), right at compilation time.
- 🪴 simplify your workflow with [stylist](https://github.com/futursolo/stylist-rs/), generating unique and dynamic class names for your CSS during compilation.
- 🎨 inject the generated CSS into your binary, guaranteeing quick access to your styles whenever you need them.

## Usage

For a complete runnable example project, you can check out the [leptos-example](https://github.com/myFavShrimp/turf/tree/main/examples/leptos-example).

1. Create SCSS styles for your application.

```scss
// file at scss/file/path.scss

:root {
    color: red;

    .SomeClass {
        color: blue;
    }
}
```
> By following the link [here](https://docs.rs/stylist/latest/stylist/struct.Style.html#style-scoping-and-substitution-rule-for-current-selector) you can gain a deeper understanding of how stylist processes selectors. By understanding these rules, you can effectively utilize the dynamic class names generated and apply styles to specific elements or components in your application.

2. Use the `style_sheet` macro to include the resulting css in your code

```rust
turf::style_sheet!("scss/file/path.scss");
```

The macro from the above example will expand to the following code:

```rust
static CLASS_NAME: &'static str = "<class_name>";
static STYLE_SHEET: &'static str = "<style_sheet>";
```

### Configuration

The configuration for turf's `configured_style_sheet` macro can be specified in the Cargo.toml file using the `[package.metadata.turf]` key. This allows you to conveniently manage your SCSS compilation settings within your project's manifest.

The following configuration options are available:

- `load_paths` (array of directories): Specifies the directories where SCSS files should be searched during compilation. This option allows you to include SCSS files from different locations, such as external libraries or custom directories.
- `output_style` (string): Defines the format of the generated CSS output. This option supports two values: `expanded` and `compressed`. Use "expanded" if you prefer a more readable and indented CSS output, or "compressed" for a minified and compact version.

Example configuration:

```toml
[package.metadata.turf]
load_paths = ["path/to/scss/files", "another/path"]
output_style = "compressed"
```

## Contributions

Contributions to turf are always welcome! Whether you have ideas for new features or improvements, don't hesitate to open an issue or submit a pull request. Let's collaborate and make turf even better together. 🤝

## License

turf is licensed under the MIT license. For more details, please refer to the LICENSE file. 📄
