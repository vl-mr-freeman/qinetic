#![warn(missing_docs)]

//! [![](https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic-internal is crate for Qinetic, facilitating optional dynamic linking.

pub mod prelude {
    //! Provides main functionality.
    #[doc(hidden)]
    #[cfg(feature = "animation")]
    pub use crate::animation::prelude;

    #[doc(hidden)]
    #[cfg(feature = "audio")]
    pub use crate::audio::prelude;

    #[doc(hidden)]
    #[cfg(feature = "network")]
    pub use crate::network::prelude;

    #[doc(hidden)]
    #[cfg(feature = "physics")]
    pub use crate::physics::prelude;

    #[doc(hidden)]
    #[cfg(feature = "render")]
    pub use crate::render::prelude;

    #[doc(hidden)]
    #[cfg(feature = "ui")]
    pub use crate::ui::prelude;

    #[doc(hidden)]
    pub use crate::{
        app::prelude::*, asset::prelude::*, core::prelude::*, ecs::prelude::*, input::prelude::*,
        log::prelude::*, math::prelude::*, window::prelude::*, DefaultPluginGroup,
        MinimalPluginGroup,
    };
}

#[cfg(feature = "animation")]
pub mod animation {
    //! Provides animation functionality.
    pub use qinetic_animation::*;
}

pub mod app {
    //! Provides application functionality.
    pub use qinetic_app::*;
}

pub mod asset {
    //! Provides asset functionality.
    pub use qinetic_asset::*;
}

#[cfg(feature = "audio")]
pub mod audio {
    //! Provides audio functionality.
    pub use qinetic_audio::*;
}

pub mod core {
    //! Provides core functionality.
    pub use qinetic_core::*;
}

pub mod ecs {
    //! Provides entity-component-system functionality.
    pub use qinetic_ecs::*;
}

pub mod input {
    //! Provides input functionality.
    pub use qinetic_input::*;
}

pub mod log {
    //! Provides log functionality.
    pub use qinetic_log::*;
}

pub mod math {
    //! Provides math functionality.
    pub use qinetic_math::*;
}

#[cfg(feature = "network")]
pub mod network {
    //! Provides network functionality.
    pub use qinetic_network::*;
}

#[cfg(feature = "physics")]
pub mod physics {
    //! Provides physics functionality.
    pub use qinetic_physics::*;
}

#[cfg(feature = "render")]
pub mod render {
    //! Provides render functionality.
    pub use qinetic_render::*;
}

#[cfg(feature = "ui")]
pub mod ui {
    //! Provides user-interface functionality.
    pub use qinetic_ui::*;
}

pub mod window {
    //! Provides window functionality.
    pub use qinetic_window::*;
}

#[cfg(feature = "animation")]
use qinetic_animation::AnimationPlugin;

use qinetic_app::plugin::{PluginGroup, PluginGroupBuilder};

use qinetic_asset::AssetPlugin;

#[cfg(feature = "audio")]
use qinetic_audio::AudioPlugin;

use qinetic_core::CorePlugin;

use qinetic_ecs::EcsPlugin;

use qinetic_input::InputPlugin;

use qinetic_log::LogPlugin;

#[cfg(feature = "network")]
use qinetic_network::NetworkPlugin;

#[cfg(feature = "physics")]
use qinetic_physics::PhysicsPlugin;

#[cfg(feature = "render")]
use qinetic_render::RenderPlugin;

#[cfg(feature = "ui")]
use qinetic_ui::UiPlugin;

use qinetic_window::WindowPlugin;

/// Minimal plugin group includes:
/// * [`CorePlugin`]
///
/// See also [`DefaultPluginGroup`] for a more complete set of plugins.
pub struct MinimalPluginGroup;

impl PluginGroup for MinimalPluginGroup {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(qinetic_core::CorePlugin::default());
    }
}

/// Default plugin group includes:
/// * [`AnimationPlugin`](../qinetic_animation/struct.AnimationPlugin.html) - feature = `animation`
/// * [`AssetPlugin`]
/// * [`AudioPlugin`](../qinetic_audio/struct.AudioPlugin.html) - feature = `audio`
/// * [`CorePlugin`]
/// * [`EcsPlugin`]
/// * [`InputPlugin`]
/// * [`LogPlugin`]
/// * [`NetworkPlugin`](../qinetic_network/struct.NetworkPlugin.html) - feature = `network`
/// * [`PhysicsPlugin`](../qinetic_physics/struct.PhysicsPlugin.html) - feature = `physics`
/// * [`RenderPlugin`](../qinetic_render/struct.RenderPlugin.html) - feature = `render`
/// * [`UiPlugin`](../qinetic_ui/struct.UiPlugin.html) - feature = `ui`
/// * [`WindowPlugin`]
///
/// See also [`MinimalPluginGroup`] for a slimmed down option.
pub struct DefaultPluginGroup;

impl PluginGroup for DefaultPluginGroup {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        #[cfg(feature = "animation")]
        group.add(AnimationPlugin::default());

        group.add(AssetPlugin::default());

        #[cfg(feature = "audio")]
        group.add(AudioPlugin::default());

        group.add(CorePlugin::default());

        group.add(EcsPlugin::default());

        group.add(InputPlugin::default());

        group.add(LogPlugin::default());

        #[cfg(feature = "physics")]
        group.add(PhysicsPlugin::default());

        #[cfg(feature = "render")]
        group.add(RenderPlugin::default());

        #[cfg(feature = "ui")]
        group.add(UiPlugin::default());

        group.add(WindowPlugin::default());
    }
}
