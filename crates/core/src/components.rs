//! Core [`Component`]s functionality.

use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

/// Tag [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_core::prelude::*;
/// #
/// App::builder()
///     .with_component(Tag::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Debug, PartialEq, Eq, Component)]
pub struct Tag {
    pub tag: String,
}

/// Script [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_core::prelude::*;
/// #
/// App::builder()
///     .with_component(Script::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Debug, PartialEq, Eq, Component)]
pub struct Script {
    pub name: String,
}
