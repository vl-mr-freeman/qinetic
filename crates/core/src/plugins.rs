//! Core [`Plugin`]s functionality.

use qinetic_app::prelude::*;
#[allow(unused_imports)]
use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

use crate::{
    components::{Script, Tag},
    stages::{CoreStage, CoreStageGroup},
    systems::CoreSystem,
};

/// Core [`Plugin`].
///
/// [`Component`]s:
/// * [`Tag`]
/// * [`Script`]
///
/// [`Stage`]s:
/// * [`CoreStageGroup`]
///
/// [`System`]s:
/// * [`CoreSystem`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_core::prelude::*;
/// #
/// App::builder()
///     .with_plugin(CorePlugin::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct CorePlugin {}

impl Plugin for CorePlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder
            .with_component(Tag::default())
            .with_component(Script::default())
            .with_stage_group(CoreStageGroup::default())
            .with_system(CoreStage::default(), CoreSystem::default());
    }
}
