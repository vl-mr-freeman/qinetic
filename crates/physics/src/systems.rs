//! Physics [`System`]s functionality.

use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

/// Physics [`System`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_physics::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(PhysicsStageGroup::default())
///     .with_system(PhysicsStage::Update, PhysicsSystem, PhysicsSystem::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct PhysicsSystem {}

impl System for PhysicsSystem {
    type Data = ();

    fn run(&mut self, data: Self::Data) {}
}
