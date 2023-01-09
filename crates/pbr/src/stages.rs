//! Physically based render [`Stage`]s functionality.

use qinetic_app::prelude::*;
#[allow(unused_imports)]
use qinetic_core::prelude::*;
use qinetic_utils::prelude::*;

/// Pbr [`Stage`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_pbr::prelude::*;
/// #
/// App::builder()
///     .with_stage(PbrStage::default(), ParallelStage::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, StageLabel)]
pub enum PbrStage {
    /// The [`Stage`] that runs after [`CoreStage::PreUpdate`].
    PreUpdate,

    /// The [`Stage`] that runs after [`CoreStage::Update`].
    #[default]
    Update,

    /// The [`Stage`] that runs after [`CoreStage::PostUpdate`].
    PostUpdate,
}

/// Pbr [`StageGroup`].
///
/// Includes:
/// * [`PbrStage::PreUpdate`]
/// * [`PbrStage::Update`]
/// * [`PbrStage::PostUpdate`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_pbr::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(PbrStageGroup::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct PbrStageGroup {}

impl StageGroup for PbrStageGroup {
    fn configure(&mut self, registry: &mut StageRegistry) {
        registry
            .add_stage_after(
                CoreStage::PreUpdate,
                PbrStage::PreUpdate,
                ParallelStage::default(),
            )
            .add_stage_after(
                CoreStage::Update,
                PbrStage::Update,
                ParallelStage::default(),
            )
            .add_stage_after(
                CoreStage::PostUpdate,
                PbrStage::PostUpdate,
                ParallelStage::default(),
            );
    }
}
