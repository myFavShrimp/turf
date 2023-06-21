# 0.4.0 [unreleased]
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
