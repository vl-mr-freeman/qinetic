//! Core [`Plugin`]s functionality.

use crate::{
    components::{Script, Tag},
    stages::{CoreStage, CoreStageGroup},
    systems::CoreSystem,
};

use qinetic_app::prelude::*;

#[allow(unused_imports)]
use qinetic_ecs::prelude::*;

/// Core [`Plugin`].
///
/// [`Component`]s:
/// * [`Tag`]
/// * [`Script`]
///
/// [`Stage`]s:
/// * [`CoreStage`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_core::prelude::*;
/// #
/// App::builder().with_plugin(CorePlugin::default()).build().run();
/// ```
#[derive(Default)]
pub struct CorePlugin {}

impl Plugin for CorePlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder.with_component(Tag::default());
        app_builder.with_component(Script::default());
        app_builder.with_stage_group(CoreStageGroup::default());
    }
}
