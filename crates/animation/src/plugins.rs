//! Animation [`Plugin`]s functionality.

use crate::{
    components::{AnimationClip, Animator, SkeletalMesh},
    stages::{AnimationStage, AnimationStageGroup},
    systems::AnimationSystem,
};
use qinetic_app::prelude::*;
use qinetic_utils::prelude::*;

#[allow(unused_imports)]
use qinetic_ecs::prelude::*;

/// Animation [`Plugin`].
///
/// [`Component`]s:
/// * [`SkeletalMesh`]
/// * [`AnimationClip`]
/// * [`Animator`]
///
/// [`Stage`]s:
/// * [`AnimationStageGroup`]
///
/// [`System`]s:
/// * [`AnimationSystem`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_animation::prelude::*;
/// #
/// App::builder()
///     .with_plugin(AnimationPlugin::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct AnimationPlugin {}

impl Plugin for AnimationPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder
            .with_component(SkeletalMesh::default())
            .with_component(AnimationClip::default())
            .with_component(Animator::default())
            .with_stage_group(AnimationStageGroup::default())
            .with_system(AnimationStage::default(), AnimationSystem::default());
    }
}
