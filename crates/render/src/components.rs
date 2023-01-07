//! Render [`Component`]s functionality.

use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;
/// Mesh [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_render::prelude::*;
/// #
/// App::builder()
///     .with_component(Mesh::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, Component)]
pub struct Mesh {}

/// Point light [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app:prelude::*;
/// # use qinetic_render::prelude::*;
/// #
/// App::builder()
///     .with_component(PointLight::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, Component)]
pub struct PointLight {}

/// Spot light [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_render::prelude::*;
/// #
/// App::builder()
///     .with_component(SpotLight::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, Component)]
pub struct SpotLight {}

/// Area light [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_render::prelude::*;
/// #
/// App::builder()
///     .with_component(AreaLight::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, Component)]
pub struct AreaLight {}

/// Directional light [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_render::prelude::*;
/// #
/// App::builder()
///     .with_component(DirectionalLight::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, Component)]
pub struct DirectionalLight {}

/// Camera [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_render::prelude::*;
/// #
/// App::builder()
///     .with_component(Camera::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, Component)]
pub struct Camera {}
