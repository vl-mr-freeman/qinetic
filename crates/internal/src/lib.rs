#![warn(missing_docs)]

//! Facilitating optional dynamic linking for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Provides main functionality.

    #[doc(hidden)]
    #[cfg(feature = "qinetic_ai")]
    pub use super::ai::prelude::*;

    #[doc(hidden)]
    #[cfg(feature = "qinetic_animation")]
    pub use super::animation::prelude::*;

    #[doc(hidden)]
    #[cfg(feature = "qinetic_audio")]
    pub use super::audio::prelude::*;

    #[doc(hidden)]
    #[cfg(feature = "qinetic_log")]
    pub use super::log::prelude::*;

    #[doc(hidden)]
    #[cfg(feature = "qinetic_network")]
    pub use super::network::prelude::*;

    #[doc(hidden)]
    #[cfg(feature = "qinetic_physics")]
    pub use super::physics::prelude::*;

    #[doc(hidden)]
    #[cfg(feature = "qinetic_render")]
    pub use super::render::prelude::*;

    #[doc(hidden)]
    #[cfg(feature = "qinetic_ui")]
    pub use super::ui::prelude::*;

    #[doc(hidden)]
    pub use super::{
        app::prelude::*, asset::prelude::*, core::prelude::*, ecs::prelude::*, input::prelude::*,
        math::prelude::*, window::prelude::*, DefaultPlugins, MinimalPlugins,
    };
}

#[cfg(feature = "qinetic_ai")]
pub use qinetic_ai as ai;
#[cfg(feature = "qinetic_animation")]
pub use qinetic_animation as animation;
pub use qinetic_app as app;
pub use qinetic_asset as asset;
#[cfg(feature = "qinetic_audio")]
pub use qinetic_audio as audio;
pub use qinetic_core as core;
pub use qinetic_ecs as ecs;
pub use qinetic_input as input;
#[cfg(feature = "qinetic_log")]
pub use qinetic_log as log;
pub use qinetic_math as math;
#[cfg(feature = "qinetic_network")]
pub use qinetic_network as network;
#[cfg(feature = "qinetic_physics")]
pub use qinetic_physics as physics;
#[cfg(feature = "qinetic_render")]
pub use qinetic_render as render;
#[cfg(feature = "qinetic_ui")]
pub use qinetic_ui as ui;
pub use qinetic_window as window;

mod default_plugins;
pub use default_plugins::*;
