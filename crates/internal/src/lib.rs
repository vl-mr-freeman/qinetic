#![warn(missing_docs)]

//! Facilitating optional dynamic linking for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Provides main functionality.

    #[doc(hidden)]
    #[cfg(feature = "qinetic_animation")]
    pub use crate::animation::prelude::*;

    #[doc(hidden)]
    #[cfg(feature = "qinetic_audio")]
    pub use crate::audio::prelude::*;

    #[doc(hidden)]
    #[cfg(feature = "qinetic_network")]
    pub use crate::network::prelude::*;

    #[doc(hidden)]
    #[cfg(feature = "qinetic_physics")]
    pub use crate::physics::prelude::*;

    #[doc(hidden)]
    #[cfg(feature = "qinetic_render")]
    pub use crate::render::prelude::*;

    #[doc(hidden)]
    #[cfg(feature = "qinetic_ui")]
    pub use crate::ui::prelude::*;

    #[doc(hidden)]
    pub use crate::{
        app::prelude::*, asset::prelude::*, core::prelude::*, ecs::prelude::*, input::prelude::*,
        log::prelude::*, math::prelude::*, window::prelude::*, DefaultPluginGroup,
        MinimalPluginGroup,
    };
}

#[cfg(feature = "qinetic_animation")]
pub use qinetic_animation as animation;
pub use qinetic_app as app;
pub use qinetic_asset as asset;
#[cfg(feature = "qinetic_audio")]
pub use qinetic_audio as audio;
pub use qinetic_core as core;
pub use qinetic_ecs as ecs;
pub use qinetic_input as input;
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
