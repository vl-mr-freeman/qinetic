//! [![](https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Render functionality for Qinetic.
//!
//! # Examples
//!
//! Here is a simple render application:
//! ```
//! use qinetic_app::prelude::*;
//! use qinetic_window::prelude::*;
//! use qinetic_render::prelude::*;
//!
//! fn main() {
//!     App::builder()
//!         .with_plugin(WindowPlugin::default())
//!         .with_plugin(RenderPlugin::default())
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
pub mod resources;
pub mod stages;
pub mod systems;

pub mod prelude {
    //! Main render functionality.

    #[doc(hidden)]
    pub use crate::{
        components::{Camera, Mesh},
        plugins::RenderPlugin,
        resources::{RenderApi, RenderResource, RenderResourceBuilder, RenderResourceBuilderError},
        stages::{RenderStage, RenderStageGroup},
        systems::RenderSystem,
    };
}
