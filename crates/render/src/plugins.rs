//! Render [`Plugin`]s functionality.

use crate::{
    components::{Camera, Mesh},
    resources::RenderResource,
    stages::{RenderStage, RenderStageGroup},
    systems::RenderSystem,
};

use qinetic_app::prelude::*;
use qinetic_utils::prelude::*;

#[allow(unused_imports)]
use qinetic_ecs::prelude::*;

/// Render [`Plugin`]
///
/// [`Component`]s:
/// * [`Mesh`]
/// * [`Camera`]
///
/// [`Stage`]s:
/// * [`RenderStageGroup`]
///
/// [`Resource`]s:
/// * [`RenderResource`]
///
/// [`System`]s:
/// * [`RenderSystem`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_render::prelude::*;
/// #
/// App::builder()
///     .with_plugin(RenderPlugin::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct RenderPlugin {}

impl Plugin for RenderPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder
            .with_component(Mesh::default())
            .with_component(Camera::default())
            .with_resource(RenderResource::default())
            .with_stage_group(RenderStageGroup::default())
            .with_system(RenderStage::default(), RenderSystem::default());
    }
}
