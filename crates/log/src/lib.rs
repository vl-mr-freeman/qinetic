//! [![](https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Log functionality for Qinetic.
//!
//! # Examples
//!
//! Here is a simple log application:
//! ```
//! use qinetic_app::prelude::*;
//! use qinetic_log::prelude::*;
//!
//! fn main() {
//!     App::builder()
//!         .with_plugin(LogPlugin::default())
//!         .build()
//!         .unwrap()
//!         .run();
//! }
//! ```

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod plugins;
pub mod resources;

pub mod prelude {
    //! Main log functionality.

    #[doc(hidden)]
    pub use crate::{
        plugins::LogPlugin,
        resources::{LogLevel, LogResource, LogResourceBuilder, LogResourceBuilderError},
    };
}
