//! Asset [`Stage`]s functionality.

use qinetic_app::prelude::*;
use qinetic_utils::prelude::*;

#[allow(unused_imports)]
use qinetic_core::prelude::*;

/// Asset [`Stage`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_asset::prelude::*;
/// #
/// App::builder()
///     .with_stage(AssetStage::default(), ParallelStage::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, StageLabel)]
pub enum AssetStage {
    /// The [`Stage`] that runs after [`CoreStage::PreUpdate`].
    #[default]
    Loads,

    /// The [`Stage`] that runs after [`CoreStage::PostUpdate`].
    Events,
}

/// Asset [`StageGroup`].
///
/// Includes:
/// * [`AssetStage::Loads`]
/// * [`AssetStage::Events`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_asset::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(AssetStageGroup::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct AssetStageGroup {}

impl StageGroup for AssetStageGroup {
    fn configure(&mut self, registry: &mut StageRegistry) {
        registry
            .add_stage_after(
                CoreStage::PreUpdate,
                AssetStage::Loads,
                ParallelStage::default(),
            )
            .add_stage_after(
                CoreStage::PostUpdate,
                AssetStage::Events,
                ParallelStage::default(),
            );
    }
}
