//! [![](https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Winit functionality for Qinetic.
//!
//! # Examples
//!
//! Here is a simple winit application:
//! ```
//! use qinetic_app::prelude::*;
//! use qinetic_winit::prelude::*;
//!
//! fn main() {
//!     App::builder()
//!         .with_plugin(WinitPlugin::default())
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
pub mod runners;
pub mod stages;
pub mod systems;

pub mod prelude {
    //! Main winit functionality.

    pub use crate::{
        plugins::WinitPlugin,
        runners::WinitRunner,
        stages::{WinitStage, WinitStageGroup},
        systems::WinitSystem,
    };
}
