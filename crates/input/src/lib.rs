//! [![](https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Input functionality for Qinetic.
//!
//! # Examples
//!
//! Here is a simple input application:
//! ```
//! use qinetic_app::prelude::*;
//! use qinetic_input::prelude::*;
//!
//! fn main() {
//!     App::builder()
//!         .with_plugin(InputPlugin::default())
//!         .build()
//!         .unwrap()
//!         .run();
//! }
//! ```

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod button;
pub mod components;
pub mod events;
pub mod gamepad;
pub mod keyboard;
pub mod mouse;
pub mod plugins;
pub mod stages;
pub mod systems;

pub mod prelude {
    //! Main input functionality.

    #[doc(hidden)]
    pub use crate::{
        button::*,
        components::PlayerController,
        gamepad::*,
        keyboard::*,
        mouse::*,
        plugins::InputPlugin,
        stages::{InputStage, InputStageGroup},
        systems::InputSystem,
    };
}
