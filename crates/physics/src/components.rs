//! Physics [`Component`]s functionality.

use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

/// Box collider [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_physics::prelude::*;
/// #
/// App::builder()
///     .with_component(BoxCollider::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, Component)]
pub struct BoxCollider {}

/// Capsule collider [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_physics::prelude::*;
/// #
/// App::builder()
///     .with_component(CapsuleCollider::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, Component)]
pub struct CapsuleCollider {}

/// Mesh collider [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_physics::prelude::*;
/// #
/// App::builder()
///     .with_component(MeshCollider::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, Component)]
pub struct MeshCollider {}

/// Sphere collider [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_physics::prelude::*;
/// #
/// App::builder()
///     .with_component(SphereCollider::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, Component)]
pub struct SphereCollider {}

/// Rigidbody [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_physics::prelude::*;
/// #
/// App::builder()
///     .with_component(Rigidbody::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, Component)]
pub struct Rigidbody {}

/// Softbody [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_physics::prelude::*;
/// #
/// App::builder()
///     .with_component(Softbody::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, Component)]
pub struct Softbody {}

/// Cloth [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_physics::prelude::*;
/// #
/// App::builder()
///     .with_component(Cloth::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, Component)]
pub struct Cloth {}
