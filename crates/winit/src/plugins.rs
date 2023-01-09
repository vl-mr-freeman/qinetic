//! Winit [`Plugin`]s functionality.

use qinetic_app::prelude::*;
#[allow(unused_imports)]
use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

use crate::{
    runners::WinitRunner,
    stages::{WinitStage, WinitStageGroup},
    systems::WinitSystem,
};

/// Winit [`Plugin`].
///
/// [`Runner`]s:
/// * [`WinitRunner`]
///
/// [`Stage`]s:
/// * [`WinitStageGroup`]
///
/// [`System`]s:
/// * [`WinitSystem`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude:*;
/// # use qinetic_winit::prelude::*;
/// #
/// App::builder()
///     .with_plugin(WinitPlugin::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct WinitPlugin {}

impl Plugin for WinitPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder
            .with_runner(WinitRunner::default())
            .with_stage_group(WinitStageGroup::default())
            .with_system(WinitStage::default(), WinitSystem::default());
    }
}
