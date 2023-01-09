//! Audio [`Stage`]s functionality.

use qinetic_app::prelude::*;
#[allow(unused_imports)]
use qinetic_core::prelude::*;
use qinetic_utils::prelude::*;

/// Audio [`Stage`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_audio::prelude::*;
/// #
/// App::builder()
///     .with_stage(AudioStage::default(), ParallelStage::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, StageLabel)]
pub enum AudioStage {
    /// The [`Stage`] that runs after [`CoreStage::PreUpdate`].
    PreUpdate,

    /// The [`Stage`] that runs after [`CoreStage::Update`].
    #[default]
    Update,

    /// The [`Stage`] that runs after [`CoreStage::PostUpdate`].
    PostUpdate,
}

/// Audio [`StageGroup`].
///
/// Includes:
/// * [`AudioStage::PreUpdate`]
/// * [`AudioStage::Update`]
/// * [`AudioStage::PostUpdate`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_audio::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(AudioStageGroup::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct AudioStageGroup {}

impl StageGroup for AudioStageGroup {
    fn configure(&mut self, registry: &mut StageRegistry) {
        registry
            .add_stage_after(
                CoreStage::PreUpdate,
                AudioStage::PreUpdate,
                ParallelStage::default(),
            )
            .add_stage_after(
                CoreStage::Update,
                AudioStage::Update,
                ParallelStage::default(),
            )
            .add_stage_after(
                CoreStage::PostUpdate,
                AudioStage::PostUpdate,
                ParallelStage::default(),
            );
    }
}
