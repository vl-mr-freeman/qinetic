//! Asset [`Resource`]s functionality.

use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

/// Asset [`Resource`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_physics::prelude::*;
/// #
/// App::builder()
///     .with_resource(AssetResource::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Debug, Getters, Builder, Resource)]
#[getset(get = "pub")]
#[builder(setter(prefix = "with"), default, derive(Debug, PartialEq, Eq))]
pub struct AssetResource {}
