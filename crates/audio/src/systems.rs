//! Audio [`System`]s functionality.

use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

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
#[derive(SmartDefault, Clone, Debug)]
pub struct AudioSystem {}

impl System for AudioSystem {
    type Data = ();

    #[allow(unused_variables)]
    fn run(&mut self, data: Self::Data) {}
}
