//! User-interface [`Stage`]s functionality.

use qinetic_app::prelude::*;
use qinetic_core::prelude::*;
use qinetic_utils::prelude::*;

/// User-interface [`Stage`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ui:prelude::*;
///`#
/// App::builder()
///     .with_stage(Uitage::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, StageLabel)]
pub enum UiStage {
    /// The [`Stage`] that runs after [`CoreStage::PreUpdate`].
    PreUpdate,

    /// The [`Stage`] that runs after [`CoreStage::Update`].
    #[default]
    Update,

    /// The [`Stage`] that runs after [`CoreStage::PostUpdate`].
    PostUpdate,
}

/// Ui [`StageGroup`].
///
/// Includes:
/// * [`UiStage::PreUpdate`]
/// * [`UiStage::Update`]
/// * [`UiStage::PostUpdate`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_window::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(UiStageGroup::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct UiStageGroup {}

impl StageGroup for UiStageGroup {
    fn configure(&mut self, registry: &mut StageRegistry) {
        registry.add_stage_after(
            CoreStage::PreUpdate,
            UiStage::PreUpdate,
            ParallelStage::default(),
        );
        registry.add_stage_after(CoreStage::Update, UiStage::Update, ParallelStage::default());
        registry.add_stage_after(
            CoreStage::PostUpdate,
            UiStage::PostUpdate,
            ParallelStage::default(),
        );
    }
}
