//! [![](https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_logo.png)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic is free, cross-platform, open-source game engine, designed to be fast, simple and modular.
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
    html_logo_url = "https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_icon.png",
    html_favicon_url = "https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_icon.png"
)]

pub use qinetic_internal::*;

#[cfg(feature = "dynamic")]
#[allow(unused_imports)]
use qinetic_dylib;
