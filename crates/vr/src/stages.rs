//! Virtual reality [`Stage`]s functionality.

use qinetic_app::prelude::*;
#[allow(unused_imports)]
use qinetic_core::prelude::*;
use qinetic_utils::prelude::*;

/// Virtual reality [`Stage`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_vr::prelude::*;
/// #
/// App::builder()
///     .with_stage(VrStage::default(), ParallelStage::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, StageLabel)]
pub enum VrStage {
    /// The [`Stage`] that runs after [`CoreStage::PreUpdate`].
    PreUpdate,

    /// The [`Stage`] that runs after [`CoreStage::Update`].
    #[default]
    Update,

    /// The [`Stage`] that runs after [`CoreStage::PostUpdate`].
    PostUpdate,
}

/// Virtual reality [`StageGroup`].
///
/// Includes:
/// * [`VrStage::PreUpdate`]
/// * [`VrStage::Update`]
/// * [`VrStage::PostUpdate`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_vr::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(VrStageGroup::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct VrStageGroup {}

impl StageGroup for VrStageGroup {
    fn configure(&mut self, registry: &mut StageRegistry) {
        registry
            .add_stage_after(
                CoreStage::PreUpdate,
                VrStage::PreUpdate,
                ParallelStage::default(),
            )
            .add_stage_after(CoreStage::Update, VrStage::Update, ParallelStage::default())
            .add_stage_after(
                CoreStage::PostUpdate,
                VrStage::PostUpdate,
                ParallelStage::default(),
            );
    }
}
