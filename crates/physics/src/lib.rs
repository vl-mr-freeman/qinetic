//! Physics functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Provides main physics functionality.
    #[doc(hidden)]
    pub use crate::{
        BoxCollider, CapsuleCollider, Cloth, MeshCollider, PhysicsPlugin, PhysicsStage, Rigidbody,
        Softbody, SphereCollider,
    };
}

use qinetic_app::prelude::*;
use qinetic_ecs::prelude::*;

/// Adds physics functionality to [`App`]
#[derive(Default)]
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder.with_component(BoxCollider::default());
        app_builder.with_component(CapsuleCollider::default());
        app_builder.with_component(MeshCollider::default());
        app_builder.with_component(SphereCollider::default());
        app_builder.with_component(Rigidbody::default());
        app_builder.with_component(Softbody::default());
        app_builder.with_component(Cloth::default());
        app_builder.with_stage(PhysicsStage::default());
    }
}

/// [`App`]'s physics step of execution cycle.
#[derive(Default, Stage)]
pub struct PhysicsStage;

#[derive(Default, Component)]
/// Box collider [`Component`].
pub struct BoxCollider {}

#[derive(Default, Component)]
/// Capsule collider [`Component`].
pub struct CapsuleCollider {}

#[derive(Default, Component)]
/// Mesh collider [`Component`].
pub struct MeshCollider {}

#[derive(Default, Component)]
/// Sphere collider [`Component`].
pub struct SphereCollider {}

#[derive(Default, Component)]
/// Rigidbody [`Component`].
pub struct Rigidbody {}

#[derive(Default, Component)]
/// Softbody [`Component`].
pub struct Softbody {}

#[derive(Default, Component)]
/// Cloth [`Component`].
pub struct Cloth {}
