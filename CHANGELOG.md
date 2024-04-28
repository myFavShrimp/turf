# 0.9.0

- Updated dependencies
- Renamed the `inline_style_sheet` macro to `style_sheet_values` to avoid confusion
- Added the `inline_style_sheet` and `inline_style_sheet_values` macros to allow inline SCSS style definitions
- Changed `randomized_class_name` to use base64 ids rather than numeric ids for randomizing class ids
- Added support for `any`, `has`, `host`, `is`, `not`, `slotted` and `where` CSS pseudo classes.

# 0.8.0

- Updated dependencies
- Removed `once_cell` feature flag
- Removed support for Rust 1.65

# 0.7.1
- Fixed compilation on minimum supported Rust version by pinning dependency versions
- Restructured project to allow specifying dependency from git repo

# 0.7.0
- Added optional configurable file output of the resulting CSS
- Added the alternative `inline_style_sheet` macro which directly returns the CSS style sheet and a class names struct
- The class name configuration is now located under the `class_names` key
- Added the `excludes` configuration option for excluding class names from the uniquification process using regex
- The minimum supported Rust version has been bumped to 1.65.0

# 0.6.2
- Fixed failing builds due to a badly specified dependency in one of turf's dependencies (thank you @xeho91 for offering a quick fix)
- Updated lightningcss

# 0.6.1
- Fixed an error with the new path resolution which resulted in incorrect paths being used for the file tracking

# 0.6.0
- Added tracking of style sheets and files in `load_paths` (SCSS recompilation on file changes)
- `load_paths` are now relative to the project directory they are specified in when using workspaces

# 0.5.0
- Updated grass to `0.13` (see [here](https://github.com/connorskees/grass/blob/master/CHANGELOG.md))
- Added instructions to trigger recompilation on SCSS style changes
- `ClassName` is now `pub` (thank you @xeho91 for creating a pull request)
- `STYLE_SHEET` is now `pub`

# 0.4.1
- Improved messages of errors with the SCCS input file path
- Fixed a misleading file path in error messages when using Cargo workspaces

# 0.4.0
- Minimum supported Rust version is now 1.70.0
- New `once_cell` feature flag for backward compatibility down to Rust version 1.64.0
- Added `[package.metadata.turf-dev]` profile for separate development and production build settings
- The configuration is now cached to avoid reading it repeatedly from the config for every macro invocation
- Added a `debug` configuration setting for debug output
- Improved the SCSS compilation error message by providing the file path to the SCSS file that caused the error

# 0.3.2
- pinned version of `lightningcss` and `lightningcss-derive` to prevent incompatible releases from being used

# 0.3.1
- fixed an issue that resulted in a compile error (thank you @xeho91 for reporting the issue and creating a pr!) #1 #2

# 0.3.0
- [lightningcss](https://github.com/parcel-bundler/lightningcss) integration for minifying and optimizing CSS 
- configurable class name generation with unique and dynamic names
- support for specifying browser targets and versions for CSS compatibility
- improved documentation and examples

# 0.2.1
- Updated description / README

# 0.2.0
- removed configured_style_sheet
- the style_sheet macro now reads the configuration from Cargo.toml like configured_style_sheet did and uses default settings as fallback

# 0.1.1

# 0.1.0
