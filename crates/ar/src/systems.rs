//! Augmented reality [`System`]s functionality.

use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

/// Augmented reality [`System`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ar::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(ArStageGroup::default())
///     .with_system(ArStage::default(), ArSystem::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct ArSystem {}

impl System for ArSystem {
    type Data = ();

    #[allow(unused_variables)]
    fn run(&mut self, data: Self::Data) {}
}
