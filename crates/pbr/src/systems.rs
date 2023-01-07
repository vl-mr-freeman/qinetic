//! Physically based render [`System`]s functionality.

use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

/// Physically based render [`System`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_pbr::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(PbrStageGroup::default())
///     .with_system(PbrStage::default(), PbrSystem::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct PbrSystem {}

impl System for PbrSystem {
    type Data = ();

    #[allow(unused_variables)]
    fn run(&mut self, data: Self::Data) {}
}
