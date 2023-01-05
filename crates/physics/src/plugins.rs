//! Physics [`Plugin`]s functionality.

use crate::{
    components::{
        BoxCollider, CapsuleCollider, Cloth, MeshCollider, Rigidbody, Softbody, SphereCollider,
    },
    resources::PhysicsResource,
    stages::{PhysicsStage, PhysicsStageGroup},
    systems::PhysicsSystem,
};

use qinetic_app::prelude::*;
use qinetic_utils::prelude::*;

#[allow(unused_imports)]
use qinetic_ecs::prelude::*;

/// Physics [`Plugin`]
///
/// [`Component`]s:
/// * [`BoxCollider`]
/// * [`CapsuleCollider`]
/// * [`MeshCollider`]
/// * [`SphereCollider`]
/// * [`Rigidbody`]
/// * [`Softbody`]
/// * [`Cloth`]
///
/// [`Resource`]s:
/// * [`PhysicsResource`]
///
/// [`Stage`]s:
/// * [`PhysicsStageGroup`]
///
/// [`System`]s:
/// * [`PhysicsSystem`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_physics::prelude::*;
/// #
/// App::builder()
///     .with_plugin(PhysicsPlugin::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct PhysicsPlugin {}

impl Plugin for PhysicsPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder
            .with_component(BoxCollider::default())
            .with_component(CapsuleCollider::default())
            .with_component(MeshCollider::default())
            .with_component(SphereCollider::default())
            .with_component(Rigidbody::default())
            .with_component(Softbody::default())
            .with_component(Cloth::default())
            .with_resource(PhysicsResource::default())
            .with_stage_group(PhysicsStageGroup::default())
            .with_system(PhysicsStage::default(), PhysicsSystem::default());
    }
}
