//! Winit [`Stage`]s functionality.

use qinetic_app::prelude::*;
#[allow(unused_imports)]
use qinetic_core::prelude::*;
use qinetic_utils::prelude::*;

/// Winit [`Stage`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_input::prelude::*;
/// #
/// App::builder()
///     .with_stage(WinitStage::default(), ParallelStage::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, StageLabel)]
pub enum WinitStage {
    /// The [`Stage`] that runs after [`CoreStage::PreUpdate`].
    PreUpdate,

    /// The [`Stage`] that runs after [`CoreStage::Update`].
    #[default]
    Update,

    /// The [`Stage`] that runs after [`CoreStage::PostUpdate`].
    PostUpdate,
}

/// Winit [`StageGroup`].
///
/// Includes:
/// * [`WinitStage::PreUpdate`]
/// * [`WinitStage::Update`]
/// * [`WinitStage::PostUpdate`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_input::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(WinitStageGroup::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct WinitStageGroup {}

impl StageGroup for WinitStageGroup {
    fn configure(&mut self, registry: &mut StageRegistry) {
        registry
            .add_stage_after(
                CoreStage::PreUpdate,
                WinitStage::PreUpdate,
                ParallelStage::default(),
            )
            .add_stage_after(
                CoreStage::Update,
                WinitStage::Update,
                ParallelStage::default(),
            )
            .add_stage_after(
                CoreStage::PostUpdate,
                WinitStage::PostUpdate,
                ParallelStage::default(),
            );
    }
}
