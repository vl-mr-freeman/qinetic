//! Animation [`Stage`]s functionality.

use qinetic_app::prelude::*;
use qinetic_utils::prelude::*;

#[allow(unused_imports)]
use qinetic_core::prelude::*;

/// Animation [`Stage`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_animation::prelude::*;
/// #
/// App::builder()
///     .with_stage(AnimationStage::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, StageLabel)]
pub enum AnimationStage {
    /// The [`Stage`] that runs after [`CoreStage::PreUpdate`].
    PreUpdate,

    /// The [`Stage`] that runs after [`CoreStage::Update`].
    #[default]
    Update,

    /// The [`Stage`] that runs after [`CoreStage::PostUpdate`].
    PostUpdate,
}

/// Animation [`StageGroup`].
///
/// Includes:
/// * [`AnimationStage::PreUpdate`]
/// * [`AnimationStage::Update`]
/// * [`AnimationStage::PostUpdate`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_animation::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(AnimationStageGroup::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct AnimationStageGroup {}

impl StageGroup for AnimationStageGroup {
    fn configure(&mut self, registry: &mut StageRegistry) {
        registry.add_stage_after(
            CoreStage::PreUpdate,
            AnimationStage::PreUpdate,
            ParallelStage::default(),
        );
        registry.add_stage_after(
            CoreStage::Update,
            AnimationStage::Update,
            ParallelStage::default(),
        );
        registry.add_stage_after(
            CoreStage::PostUpdate,
            AnimationStage::PostUpdate,
            ParallelStage::default(),
        );
    }
}
