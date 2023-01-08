//! Animation [`System`]s functionality.

use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

/// Animation [`System`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_animation::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(AnimationStageGroup::default())
///     .with_system(AnimationStage::default(), AnimationSystem::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct AnimationSystem {}

impl System for AnimationSystem {
    type Data = ();

    #[allow(unused_variables)]
    fn run(&mut self, data: Self::Data) {}
}
