//! Audio [`System`]s functionality.

use qinetic_ecs::prelude::*;

/// Audio [`System`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_audio::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(AudioStageGroup::default())
///     .with_system(AudioStage::default(), AudioSystem::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(Default, Clone, Debug)]
pub struct AudioSystem {}

impl System for AudioSystem {
    type Data = ();

    fn run(&mut self, data: Self::Data) {}
}
