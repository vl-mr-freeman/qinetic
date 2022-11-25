//! Render functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Provides main render functionality.
    #[doc(hidden)]
    pub use crate::{Mesh, RenderApi, RenderPlugin, RenderResource, RenderStage};
}

use qinetic_app::prelude::*;
use qinetic_core::color::Color4;
use qinetic_ecs::prelude::*;

/// Adds render functionality to [`App`]
#[derive(Default)]
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder.with_resource(RenderResource::default());
        app_builder.with_component(Mesh::default());
        app_builder.with_stage(RenderStage::default());
    }
}

#[derive(Default)]
pub enum RenderApi {
    #[default]
    Vulkan,
    None,
}

#[derive(Resource)]
pub struct RenderResource {
    pub api: RenderApi,
    pub max_frames_in_flight: u32,
    pub clear_color: Color4,
}

impl Default for RenderResource {
    fn default() -> Self {
        Self {
            api: RenderApi::default(),
            max_frames_in_flight: 2,
            clear_color: Color4::BLACK,
        }
    }
}

/// [`App`]'s render step of execution cycle.
#[derive(Default)]
pub struct RenderStage;

impl Stage for RenderStage {}

#[derive(Default, Component)]
/// Mesh [`Component`].
pub struct Mesh {}
