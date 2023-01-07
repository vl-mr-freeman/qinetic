//! [![](https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Physically based render functionality for Qinetic.
//!
//! # Examples
//!
//! Here is a simple physically based render pplication:
//! ```
//! use qinetic_app::prelude::*;
//! use qinetic_pbr::prelude::*;
//! use qinetic_render::prelude::*;
//!
//! fn main() {
//!     App::builder()
//!         .with_plugin(RenderPlugin::default())
//!         .with_plugin(PbrPlugin::default())
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
        components::{AreaLight, DirectionalLight, PointLight, SpotLight},
        plugins::PbrPlugin,
        resources::{PbrResource, PbrResourceBuilder, PbrResourceBuilderError},
        stages::{PbrStage, PbrStageGroup},
        systems::PbrSystem,
    };
}
