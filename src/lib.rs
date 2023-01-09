//! [![](https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Free, cross-platform, open-source game engine, designed to be fast, simple and modular.
//!
//! # Examples
//!
//! Here is a simple application:
//! ```
//! use qinetic::prelude::*;
//!
//! fn main() {
//!     App::builder()
//!         .with_runner(RunOnce::default())
//!         .with_plugin_group(DefaultPluginGroup::default())
//!         .build()
//!         .unwrap()
//!         .run();
//! }
//! ```

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

#[cfg(feature = "dynamic")]
#[allow(unused_imports)]
use qinetic_dylib;
pub use qinetic_internal::*;
