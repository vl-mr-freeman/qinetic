//! Render [`System`]s functionality.

use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

/// Render [`System`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_window::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(RenderStageGroup::default())
///     .with_system(RenderStage::Update, RenderSystem, RenderSystem::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct RenderSystem {}

impl System for RenderSystem {
    type Data = ();

    fn run(&mut self, data: Self::Data) {}
}
