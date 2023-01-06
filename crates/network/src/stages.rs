//! Network [`Stage`]s functionality.

use qinetic_app::prelude::*;
use qinetic_utils::prelude::*;

#[allow(unused_imports)]
use qinetic_core::prelude::*;

/// Network [`Stage`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_network::prelude::*;
/// #
/// App::builder()
///     .with_stage(NetworkStage::default(), ParallelStage::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, StageLabel)]
pub enum NetworkStage {
    /// The [`Stage`] that runs after [`CoreStage::PreUpdate`].
    PreUpdate,

    /// The [`Stage`] that runs after [`CoreStage::Update`].
    #[default]
    Update,

    /// The [`Stage`] that runs after [`CoreStage::PostUpdate`].
    PostUpdate,
}

/// Network [`StageGroup`].
///
/// Includes:
/// * [`NetworkStage::PreUpdate`]
/// * [`NetworkStage::Update`]
/// * [`NetworkStage::PostUpdate`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_network::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(NetworkStageGroup::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct NetworkStageGroup {}

impl StageGroup for NetworkStageGroup {
    fn configure(&mut self, registry: &mut StageRegistry) {
        registry
            .add_stage_after(
                CoreStage::PreUpdate,
                NetworkStage::PreUpdate,
                ParallelStage::default(),
            )
            .add_stage_after(
                CoreStage::Update,
                NetworkStage::Update,
                ParallelStage::default(),
            )
            .add_stage_after(
                CoreStage::PostUpdate,
                NetworkStage::PostUpdate,
                ParallelStage::default(),
            );
    }
}
