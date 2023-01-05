//! [![](https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Augmented reality functionality for Qinetic.
//!
//! # Examples
//!
//! Here is a simple augmented reality application:
//! ```
//! use qinetic_app::prelude::*;
//! use qinetic_ar::prelude::*;
//!
//! fn main() {
//!     App::builder()
//!         .with_plugin(ArPlugin::default())
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
    //! Main augmented reality functionality.

    #[doc(hidden)]
    pub use crate::{
        components::{ArController, ArTracker},
        plugins::ArPlugin,
        stages::{ArStage, ArStageGroup},
        systems::ArSystem,
    };
}
