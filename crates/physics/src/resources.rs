//! Physics [`Resource`]s functionality.

use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

/// Physics [`Resource`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_physics::prelude::*;
/// #
/// App::builder()
///     .with_resource(PhysicsResource::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Debug, Getters, Builder, Resource)]
#[getset(get = "pub")]
#[builder(
    crate = "crate::resources",
    setter(prefix = "with"),
    default,
    derive(Debug, PartialEq, Eq)
)]
pub struct PhysicsResource {}
