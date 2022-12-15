//! Physics functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Main physics functionality.

    #[doc(hidden)]
    pub use crate::{
        BoxCollider, CapsuleCollider, Cloth, MeshCollider, PhysicsPlugin, PhysicsStage, Rigidbody,
        Softbody, SphereCollider,
    };
}

use qinetic_app::prelude::*;
use qinetic_ecs::prelude::*;

/// Physics [`Plugin`] for [`App`]
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
/// [`Stage`]s:
/// * [`PhysicsStage`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_physics::prelude::*;
/// #
/// App::builder().with_plugin(PhysicsPlugin::default()).build().run();
/// ```
#[derive(Default)]
pub struct PhysicsPlugin {}

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

/// Physics [`Stage`] for [`App`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_physics::prelude::*;
/// #
/// App::builder().with_stage(PhysicsStage::default()).build().run();
/// ```
#[derive(Default, Stage)]
pub struct PhysicsStage;

/// Box collider [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_physics::prelude::*;
/// #
/// App::builder().with_component(BoxCollider::default()).build().run();
/// ```
#[derive(Default, Component)]
pub struct BoxCollider {}

/// Capsule collider [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_physics::prelude::*;
/// #
/// App::builder().with_component(CapsuleCollider::default()).build().run();
/// ```
#[derive(Default, Component)]
pub struct CapsuleCollider {}

/// Mesh collider [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_physics::prelude::*;
/// #
/// App::builder().with_component(MeshCollider::default()).build().run();
/// ```
#[derive(Default, Component)]
pub struct MeshCollider {}

/// Sphere collider [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_physics::prelude::*;
/// #
/// App::builder().with_component(SphereCollider::default()).build().run();
/// ```
#[derive(Default, Component)]
pub struct SphereCollider {}

/// Rigidbody [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_physics::prelude::*;
/// #
/// App::builder().with_component(Rigidbody::default()).build().run();
/// ```
#[derive(Default, Component)]
pub struct Rigidbody {}

/// Softbody [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_physics::prelude::*;
/// #
/// App::builder().with_component(Softbody::default()).build().run();
/// ```
#[derive(Default, Component)]
pub struct Softbody {}

/// Cloth [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_physics::prelude::*;
/// #
/// App::builder().with_component(Cloth::default()).build().run();
/// ```
#[derive(Default, Component)]
pub struct Cloth {}
