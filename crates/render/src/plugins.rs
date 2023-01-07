//! Render [`Plugin`]s functionality.

use crate::{
    components::{AreaLight, Camera, DirectionalLight, Mesh, PointLight, SpotLight},
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
/// [`Resource`]s:
/// * [`RenderResource`]
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
            .with_resource(RenderResource::default())
            .with_component(Mesh::default())
            .with_component(Camera::default())
            .with_component(PointLight::default())
            .with_component(AreaLight::default())
            .with_component(SpotLight::default())
            .with_component(DirectionalLight::default())
            .with_stage_group(RenderStageGroup::default())
            .with_system(RenderStage::default(), RenderSystem::default());
    }
}
