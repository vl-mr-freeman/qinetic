//! Math [`Component`]s functionality.

use crate::vector::Vector3;
use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

/// Transform [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_math::prelude::*;
/// #
/// App::builder()
///     .with_component(Transform::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Component)]
pub struct Transform {
    /// Position of the [`Entity`] in the [`World`].
    pub position: Vector3<f32>,

    /// Rotation of the [`Entity`] in the [`World`].
    pub rotation: Vector3<f32>,

    /// Scale of the [`Entity`] in the [`World`].
    pub scale: Vector3<f32>,
}
