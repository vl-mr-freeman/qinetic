//! Augmented reality [`Plugin`]s functionality.

use qinetic_app::prelude::*;
#[allow(unused_imports)]
use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

use crate::{
    components::{ArController, ArTracker},
    stages::{ArStage, ArStageGroup},
    systems::ArSystem,
};

/// Augmented reality [`Plugin`].
///
/// [`Component`]s:
/// * [`ArController`]
/// * [`ArTracker`]
///
/// [`Stage`]s:
/// * [`ArStageGroup`]
///
/// [`System`]s:
/// * [`ArSystem`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ar::prelude::*;
/// #
/// App::builder()
///     .with_plugin(ArPlugin::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct ArPlugin {}

impl Plugin for ArPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder
            .with_component(ArController::default())
            .with_component(ArTracker::default())
            .with_stage_group(ArStageGroup::default())
            .with_system(ArStage::default(), ArSystem::default());
    }
}
