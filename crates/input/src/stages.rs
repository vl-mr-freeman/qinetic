//! Input [`Stage`]s functionality.

use qinetic_app::prelude::*;
use qinetic_utils::prelude::*;

#[allow(unused_imports)]
use qinetic_core::prelude::*;

/// Input [`Stage`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_input::prelude::*;
/// #
/// App::builder()
///     .with_stage(InputStage::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, StageLabel)]
pub enum InputStage {
    /// The [`Stage`] that runs after [`CoreStage::PreUpdate`].
    PreUpdate,

    /// The [`Stage`] that runs after [`CoreStage::Update`].
    #[default]
    Update,

    /// The [`Stage`] that runs after [`CoreStage::PostUpdate`].
    PostUpdate,
}

/// Artificial intelligence [`StageGroup`].
///
/// Includes:
/// * [`InputStage::PreUpdate`]
/// * [`InputStage::Update`]
/// * [`InputStage::PostUpdate`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ai::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(InputStageGroup::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct InputStageGroup {}

impl StageGroup for InputStageGroup {
    fn configure(&mut self, registry: &mut StageRegistry) {
        registry.add_stage_after(
            CoreStage::PreUpdate,
            InputStage::PreUpdate,
            ParallelStage::default(),
        );
        registry.add_stage_after(
            CoreStage::Update,
            InputStage::Update,
            ParallelStage::default(),
        );
        registry.add_stage_after(
            CoreStage::PostUpdate,
            InputStage::PostUpdate,
            ParallelStage::default(),
        );
    }
}
