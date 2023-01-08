//! Input [`Plugin`]s functionality.

use crate::{
    components::PlayerController,
    stages::{InputStage, InputStageGroup},
    systems::InputSystem,
};
use qinetic_app::prelude::*;
use qinetic_utils::prelude::*;

#[allow(unused_imports)]
use qinetic_ecs::prelude::*;

/// Input [`Plugin`].
///
/// [`Component`]s:
/// * [`PlayerController`]
///
/// [`Stage`]s:
/// * [`InputStageGroup`]
///
/// [`System`]s:
/// * [`InputSystem`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_input::prelude::*;
/// #
/// App::builder()
///     .with_plugin(InputPlugin::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct InputPlugin {}

impl Plugin for InputPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder
            .with_component(PlayerController::default())
            .with_stage_group(InputStageGroup::default())
            .with_system(InputStage::default(), InputSystem::default());
    }
}
