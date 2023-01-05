//! Audio [`Plugin`]s functionality.

use crate::{
    components::{Listener, Sound},
    stages::{AudioStage, AudioStageGroup},
    systems::AudioSystem,
};
use qinetic_app::prelude::*;
use qinetic_utils::prelude::*;

#[allow(unused_imports)]
use qinetic_ecs::prelude::*;

/// Audio [`Plugin`].
///
/// [`Component`]s:
/// * [`Listener`]
/// * [`Sound`]
///
/// [`Stage`]s:
/// * [`AudioStageGroup`]
///
/// [`System`]s:
/// * [`AudioSystem`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_audio::prelude::*;
/// #
/// App::builder()
///     .with_plugin(AudioPlugin::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct AudioPlugin {}

impl Plugin for AudioPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder
            .with_component(Listener::default())
            .with_component(Sound::default())
            .with_stage_group(AudioStageGroup::default())
            .with_system(AudioStage::default(), AudioSystem::default());
    }
}
