//! Artificial intelligence [`System`]s functionality.

use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

/// Artificial intelligence [`System`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ai::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(AiStageGroup::default())
///     .with_system(AiStage::Update, AiSystem, AiSystem::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct AiSystem {}

impl System for AiSystem {
    type Data = ();

    fn run(&mut self, data: Self::Data) {}
}
