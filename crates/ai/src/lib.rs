//! [![](https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Artificial intelligence functionality for Qinetic.
//!
//! # Examples
//!
//! Here is a simple artificial intelligence application:
//! ```
//! use qinetic_ai::prelude::*;
//! use qinetic_app::prelude::*;
//!
//! fn main() {
//!     App::builder()
//!         .with_plugin(AiPlugin::default())
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
    //! Main artificial intelligence functionality.

    #[doc(hidden)]
    pub use crate::{
        components::{AiController, AiNavBoundsVolume},
        plugins::AiPlugin,
        stages::{AiStage, AiStageGroup},
        systems::AiSystem,
    };
}
