//! Window [`Stage`]s functionality.

use qinetic_app::prelude::*;
use qinetic_utils::prelude::*;

#[allow(unused_imports)]
use qinetic_core::prelude::*;

/// Window [`Stage`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_window:prelude::*;
/// #
/// App::builder()
///     .with_stage(WindowStage::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, StageLabel)]
pub enum WindowStage {
    /// The [`Stage`] that runs after [`CoreStage::PreUpdate`].
    PreUpdate,

    /// The [`Stage`] that runs after [`CoreStage::Update`].
    #[default]
    Update,

    /// The [`Stage`] that runs after [`CoreStage::PostUpdate`].
    PostUpdate,
}

/// Window [`StageGroup`].
///
/// Includes:
/// * [`WindowStage::PreUpdate`]
/// * [`WindowStage::Update`]
/// * [`WindowStage::PostUpdate`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_window::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(WindowStageGroup::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct WindowStageGroup {}

impl StageGroup for WindowStageGroup {
    fn configure(&mut self, registry: &mut StageRegistry) {
        registry.add_stage_after(
            CoreStage::PreUpdate,
            WindowStage::PreUpdate,
            ParallelStage::default(),
        );
        registry.add_stage_after(
            CoreStage::Update,
            WindowStage::Update,
            ParallelStage::default(),
        );
        registry.add_stage_after(
            CoreStage::PostUpdate,
            WindowStage::PostUpdate,
            ParallelStage::default(),
        );
    }
}
