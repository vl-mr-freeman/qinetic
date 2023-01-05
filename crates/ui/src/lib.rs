//! [![](https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! User-interface functionality for Qinetic.
//!
//! # Examples
//!
//! Here is a simple user-interface application:
//! ```
//! use qinetic_app::prelude::*;
//! use qinetic_ui::prelude::*;
//!
//! fn main() {
//!     App::builder()
//!         .with_plugin(UiPlugin::default())
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
pub mod font;
pub mod plugins;
pub mod resources;
pub mod stages;
pub mod systems;
pub mod theme;

pub mod prelude {
    //! Main user-interface functionality.

    #[doc(hidden)]
    pub use crate::{
        components::{Button, Image, RectTransform, Scrollbar, Slider, Text},
        font::*,
        plugins::UiPlugin,
        resources::UiResource,
        stages::{UiStage, UiStageGroup},
        systems::UiSystem,
        theme::*,
    };
}
