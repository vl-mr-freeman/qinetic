//! [![](https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Free, cross-platform, open-source game engine, designed to be fast, simple and modular.
//!
//! ## Example
//!  Here is a simple application:
//!  ```
//!  use qinetic::prelude::*;
//!
//!  fn main() {
//!     Application::new()
//!         .add_plugin_group(DefaultPluginGroup)
//!         .run();
//!  }
//! ```

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub use qinetic_internal::*;

#[cfg(feature = "dynamic")]
#[allow(unused_imports)]
use qinetic_dylib;
