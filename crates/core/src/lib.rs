//! [![](https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Core functionality for Qinetic.
//!
//! # Examples
//!
//! Here is a simple core application:
//! ```
//! use qinetic_app::prelude::*;
//! use qinetic_core::prelude::*;
//!
//! fn main() {
//!     App::builder()
//!         .with_plugin(CorePlugin::default())
//!         .build()
//!         .unwrap()
//!         .run();
//! }
//! ```

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod components;
pub mod plugins;
pub mod stages;
pub mod systems;

pub mod prelude {
    //! Main core functionality.

    #[doc(hidden)]
    pub use crate::{
        components::{Script, Tag},
        plugins::CorePlugin,
        stages::{CoreStage, CoreStageGroup},
    };
}
