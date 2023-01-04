//! Virtual reality [`System`]s functionality.

use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

/// Virtual reality [`System`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_vr::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(VrStageGroup::default())
///     .with_system(VrStage::Update, VrSystem, VrSystem::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct VrSystem {}

impl System for VrSystem {
    type Data = ();

    fn run(&mut self, data: Self::Data) {}
}
