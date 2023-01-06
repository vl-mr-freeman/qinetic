//! Augmented reality [`Stage`]s functioanality.

use qinetic_app::prelude::*;
use qinetic_utils::prelude::*;

#[allow(unused_imports)]
use qinetic_core::prelude::*;

/// Augmented reality [`Stage`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ar::prelude::*;
/// #
/// App::builder()
///     .with_stage(ArStage::default(), ParallelStage::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, StageLabel)]
pub enum ArStage {
    /// The [`Stage`] that runs after [`CoreStage::PreUpdate`].
    PreUpdate,

    /// The [`Stage`] that runs after [`CoreStage::Update`].
    #[default]
    Update,

    /// The [`Stage`] that runs after [`CoreStage::PostUpdate`].
    PostUpdate,
}

/// Augmented reality [`StageGroup`].
///
/// Includes:
/// * [`ArStage::PreUpdate`]
/// * [`ArStage::Update`]
/// * [`ArStage::PostUpdate`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ar::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(ArStageGroup::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct ArStageGroup {}

impl StageGroup for ArStageGroup {
    fn configure(&mut self, registry: &mut StageRegistry) {
        registry
            .add_stage_after(
                CoreStage::PreUpdate,
                ArStage::PreUpdate,
                ParallelStage::default(),
            )
            .add_stage_after(CoreStage::Update, ArStage::Update, ParallelStage::default())
            .add_stage_after(
                CoreStage::PostUpdate,
                ArStage::PostUpdate,
                ParallelStage::default(),
            );
    }
}
