//! Artificial intelligence [`Stage`]s functionality.

use qinetic_app::prelude::*;
use qinetic_utils::prelude::*;

#[allow(unused_imports)]
use qinetic_core::prelude::*;

/// Artificial intelligence [`Stage`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ai::prelude::*;
/// #
/// App::builder()
///     .with_stage(AiStage::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, StageLabel)]
pub enum AiStage {
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
/// * [`AiStage::PreUpdate`]
/// * [`AiStage::Update`]
/// * [`AiStage::PostUpdate`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ai::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(AiStageGroup::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct AiStageGroup {}

impl StageGroup for AiStageGroup {
    fn configure(&mut self, registry: &mut StageRegistry) {
        registry.add_stage_after(
            CoreStage::PreUpdate,
            AiStage::PreUpdate,
            ParallelStage::default(),
        );
        registry.add_stage_after(CoreStage::Update, AiStage::Update, ParallelStage::default());
        registry.add_stage_after(
            CoreStage::PostUpdate,
            AiStage::PostUpdate,
            ParallelStage::default(),
        );
    }
}
