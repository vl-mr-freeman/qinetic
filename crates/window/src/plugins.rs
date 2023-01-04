//! Window [`Plugin`]s functionality.

use crate::{
    events::*,
    resources::WindowResource,
    stages::{WindowStage, WindowStageGroup},
    systems::WindowSystem,
};
use qinetic_app::prelude::*;
use qinetic_utils::prelude::*;

#[allow(unused_imports)]
use qinetic_ecs::prelude::*;

/// Window [`Plugin`].
///
/// [`Stage`]s:
/// * [`WindowStageGroup`]
///
/// [`Resource`]s:
/// * [`WindowResource`]
///
/// [`Event`]s:
/// * [`WindowEvent`]
/// * [`CursorEvent`]
///
/// [`System`]s:
/// * [`WindowSystem`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_window::prelude::*;
/// #
/// App::builder()
///     .with_plugin(WindowPlugin::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct WindowPlugin {}

impl Plugin for WindowPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder
            .with_event(WindowEvent::Created)
            .with_event(WindowEvent::Destroyed)
            .with_event(WindowEvent::Resized)
            .with_event(WindowEvent::Focused)
            .with_event(WindowEvent::Moved)
            .with_event(CursorEvent::Moved)
            .with_event(CursorEvent::Entered)
            .with_event(CursorEvent::Left)
            .with_resource(WindowResource::default())
            .with_stage_group(WindowStageGroup::default())
            .with_system(WindowStage::default(), WindowSystem::default());
    }
}
