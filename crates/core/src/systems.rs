//! Core [`System`]s functionality.

use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

/// Core [`System`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_core::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(CoreStageGroup::default())
///     .with_system(CoreStage::default(), CoreSystem::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct CoreSystem {}

impl System for CoreSystem {
    type Data = ();

    fn run(&mut self, data: Self::Data) {}
}
