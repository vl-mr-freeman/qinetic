//! Animation [`Plugin`]s functionality.

use qinetic_app::prelude::*;
#[allow(unused_imports)]
use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

use crate::{
    components::{AnimationClip, Animator, SkeletalMesh},
    stages::{AnimationStage, AnimationStageGroup},
    systems::AnimationSystem,
};

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
///     .unwrap();
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
