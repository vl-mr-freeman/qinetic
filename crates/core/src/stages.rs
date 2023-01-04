//! Core [`Stage`]s functionality.

use qinetic_app::prelude::*;
use qinetic_utils::prelude::*;

/// Core [`Stage`].
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_core::prelude::*;
/// #
/// App::builder()
///     .with_stage(CoreStage::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, StageLabel)]
pub enum CoreStage {
    /// The [`Stage`] that runs before [`CoreStage::Update`].
    PreUpdate,

    /// The [`Stage`] that runs on [`updating`](App::update).
    #[default]
    Update,

    /// The [`Stage`] that runs after [`CoreStage::Update`].
    PostUpdate,
}

/// Core [`StageGroup`].
///
/// Includes:
/// * [`CoreStage::PreUpdate`]
/// * [`CoreStage::Update`]
/// * [`CoreStage::PostUpdate`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_core::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(CoreStageGroup::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct CoreStageGroup {}

impl StageGroup for CoreStageGroup {
    fn configure(&mut self, registry: &mut StageRegistry) {
        registry.add_stage(CoreStage::PreUpdate, ParallelStage::default());
        registry.add_stage(CoreStage::Update, ParallelStage::default());
        registry.add_stage(CoreStage::PostUpdate, ParallelStage::default());
    }
}
