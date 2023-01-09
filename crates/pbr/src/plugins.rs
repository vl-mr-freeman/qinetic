//! Physically based render [`Plugin`]s functionality.

use qinetic_app::prelude::*;
#[allow(unused_imports)]
use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

use crate::{
    components::{AreaLight, DirectionalLight, PointLight, SpotLight},
    resources::PbrResource,
    stages::{PbrStage, PbrStageGroup},
    systems::PbrSystem,
};

/// Physically based render [`Plugin`].
///
/// [`Component`]s:
/// * [`PointLight`]
/// * [`SpotLight`]
/// * [`AreaLight`]
/// * [`DirectionalLight`]
///
/// [`Stage`]s:
/// * [`PbrStageGroup`]
///
/// [`Resource`]s:
/// * [`PbrResource`]
///
/// [`System`]s:
/// * [`PbrSystem`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_pbr::prelude::*;
/// #
/// App::builder()
///     .with_plugin(PbrPlugin::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct PbrPlugin {}

impl Plugin for PbrPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder
            .with_component(PointLight::default())
            .with_component(AreaLight::default())
            .with_component(SpotLight::default())
            .with_component(DirectionalLight::default())
            .with_resource(PbrResource::default())
            .with_stage_group(PbrStageGroup::default())
            .with_system(PbrStage::default(), PbrSystem::default());
    }
}
