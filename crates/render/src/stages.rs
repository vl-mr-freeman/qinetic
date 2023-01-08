//! Render [`Stage`]s functionality.

use qinetic_app::prelude::*;
use qinetic_utils::prelude::*;

#[allow(unused_imports)]
use qinetic_core::prelude::*;

/// Render [`Stage`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_render::prelude::*;
/// #
/// App::builder()
///     .with_stage(RenderStage::default(), ParallelStage::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, StageLabel)]
pub enum RenderStage {
    /// The [`Stage`] that runs after [`CoreStage::PreUpdate`].
    PreUpdate,

    /// The [`Stage`] that runs after [`CoreStage::Update`].
    #[default]
    Update,

    /// The [`Stage`] that runs after [`CoreStage::PostUpdate`].
    PostUpdate,
}

/// Render [`StageGroup`].
///
/// Includes:
/// * [`RenderStage::PreUpdate`]
/// * [`RenderStage::Update`]
/// * [`RenderStage::PostUpdate`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_render::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(RenderStageGroup::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct RenderStageGroup {}

impl StageGroup for RenderStageGroup {
    fn configure(&mut self, registry: &mut StageRegistry) {
        registry
            .add_stage_after(
                CoreStage::PreUpdate,
                RenderStage::PreUpdate,
                ParallelStage::default(),
            )
            .add_stage_after(
                CoreStage::Update,
                RenderStage::Update,
                ParallelStage::default(),
            )
            .add_stage_after(
                CoreStage::PostUpdate,
                RenderStage::PostUpdate,
                ParallelStage::default(),
            );
    }
}
