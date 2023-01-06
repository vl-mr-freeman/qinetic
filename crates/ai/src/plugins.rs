//! Artificial intelligence [`Plugin`]s functionality.

use crate::{
    components::{AiController, AiNavBoundsVolume},
    stages::{AiStage, AiStageGroup},
    systems::AiSystem,
};
use qinetic_app::prelude::*;
use qinetic_utils::prelude::*;

#[allow(unused_imports)]
use qinetic_ecs::prelude::*;

/// Artificial intelligence [`Plugin`].
///
/// [`Component`]s:
/// * [`AiController`]
/// * [`AiNavBoundsVolume`]
///
/// [`Stage`]s:
/// * [`AiStageGroup`]
///
/// [`System`]s:
/// * [`AiSystem`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ai::prelude::*;
/// #
/// App::builder()
///     .with_plugin(AiPlugin::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct AiPlugin {}

impl Plugin for AiPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder.with_schedule_fn(|schedule: &mut Schedule| {
            schedule
                .add_stage_group(AiStageGroup::default())
                .add_system(AiStage::default(), AiSystem::default());
        });
    }
}
