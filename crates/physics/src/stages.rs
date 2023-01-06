//! Physics [`Stage`]s functionality.

use qinetic_app::prelude::*;
use qinetic_core::prelude::*;
use qinetic_utils::prelude::*;

/// Physics [`Stage`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_physics::prelude::*;
/// #
/// App::builder()
///     .with_stage(PhysicsStage::default(), ParallelStage::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, StageLabel)]
pub enum PhysicsStage {
    /// The [`Stage`] that runs after [`CoreStage::PreUpdate`].
    PreUpdate,

    /// The [`Stage`] that runs after [`CoreStage::Update`].
    #[default]
    Update,

    /// The [`Stage`] that runs after [`CoreStage::PostUpdate`].
    PostUpdate,
}

/// Physics [`StageGroup`].
///
/// Includes:
/// * [`PhysicsStage::PreUpdate`]
/// * [`PhysicsStage::Update`]
/// * [`PhysicsStage::PostUpdate`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_physics::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(PhysicsStageGroup::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct PhysicsStageGroup {}

impl StageGroup for PhysicsStageGroup {
    fn configure(&mut self, registry: &mut StageRegistry) {
        registry
            .add_stage_after(
                CoreStage::PreUpdate,
                PhysicsStage::PreUpdate,
                ParallelStage::default(),
            )
            .add_stage_after(
                CoreStage::Update,
                PhysicsStage::Update,
                ParallelStage::default(),
            )
            .add_stage_after(
                CoreStage::PostUpdate,
                PhysicsStage::PostUpdate,
                ParallelStage::default(),
            );
    }
}
