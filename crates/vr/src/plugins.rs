//! Virtual reality [`Plugin`]s functionality.

use qinetic_app::prelude::*;
#[allow(unused_imports)]
use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

use crate::{
    components::{VrController, VrTracker},
    stages::{VrStage, VrStageGroup},
    systems::VrSystem,
};

/// Virtual reality [`Plugin`].
///
/// [`Component`]s:
/// * [`VrController`]
/// * [`VrTracker`]
///
/// [`Stage`]s:
/// * [`VrStageGroup`]
///
/// [`System`]s:
/// * [`VrSystem`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_vr::prelude::*;
/// #
/// App::builder()
///     .with_plugin(VrPlugin::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct VrPlugin {}

impl Plugin for VrPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder
            .with_component(VrController::default())
            .with_component(VrTracker::default())
            .with_stage_group(VrStageGroup::default())
            .with_system(VrStage::default(), VrSystem::default());
    }
}
