//! Audio [`Component`]s functionality.

use qinetic_ecs::prelude::*;

/// Sound [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_audio::prelude::*;
/// #
/// App::builder()
///     .with_component(Sound::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Component)]
pub struct Sound {}

/// Listener [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_audio::prelude::*;
/// #
/// App::builder()
///     .with_component(Listener::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Component)]
pub struct Listener {}
