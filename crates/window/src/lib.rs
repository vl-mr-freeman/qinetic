//! [![](https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Window functionality for Qinetic.
//!
//! # Examples
//!
//! Here is a simple window application:
//! ```
//! use qinetic_app::prelude::*;
//! use qinetic_window::prelude::*;
//!
//! fn main() {
//!     App::builder()
//!         .with_plugin(WindowPlugin::default())
//!         .build()
//!         .unwrap()
//!         .run();
//! }
//! ```

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod event_loop;
pub mod events;
pub mod plugins;
pub mod resources;
pub mod stages;
pub mod systems;
pub mod window;

pub mod prelude {
    //! Main window functionality.

    #[doc(hidden)]
    pub use crate::{
        event_loop::*,
        events::{CursorEvent, WindowEvent},
        plugins::WindowPlugin,
        resources::{
            WindowMode,
            WindowMonitor,
            WindowPosition,
            WindowResource,
            WindowResourceBuilder,
            WindowResourceBuilderError,
            WindowSizeConstraints,
        },
        stages::{WindowStage, WindowStageGroup},
        systems::WindowSystem,
        window::*,
    };
}
