//! [![](https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Audio functionality for Qinetic.
//!
//! # Examples
//!
//! Here is a simple audio application:
//! ```
//! use qinetic_app::prelude::*;
//! use qinetic_audio::prelude::*;
//!
//! fn main() {
//!     App::builder()
//!         .with_plugin(AudioPlugin::default())
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
    //! Main audio functionality.

    #[doc(hidden)]
    pub use crate::{
        components::{Listener, Sound},
        plugins::AudioPlugin,
        stages::{AudioStage, AudioStageGroup},
        systems::AudioSystem,
    };
}
