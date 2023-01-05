#![warn(missing_docs)]

//! Facilitating optional dynamic linking for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Main functionality.

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
    #[cfg(feature = "qinetic_vr")]
    pub use super::vr::prelude::*;

    #[doc(hidden)]
    #[cfg(feature = "qinetic_ar")]
    pub use super::ar::prelude::*;

    #[doc(hidden)]
    pub use super::{
        app::prelude::*, asset::prelude::*, core::prelude::*, ecs::prelude::*, input::prelude::*,
        math::prelude::*, utils::prelude::*, window::prelude::*, DefaultPluginGroup,
        MinimalPluginGroup, RunLoop, RunOnce,
    };
}

pub mod app {
    //! Application functionality.

    pub use qinetic_app::*;
}

pub mod asset {
    //! Asset functionality.

    pub use qinetic_asset::*;
}

pub mod core {
    //! Core functionality.

    pub use qinetic_core::*;
}

pub mod ecs {
    //! Entity-component-system functionality.

    pub use qinetic_ecs::*;
}

pub mod input {
    //! Input functionality.

    pub use qinetic_input::*;
}

pub mod math {
    //! Math functionality.

    pub use qinetic_math::*;
}

pub mod utils {
    //! Utils functionality.

    pub use qinetic_utils::*;
}

pub mod window {
    //! Window functionality.

    pub use qinetic_window::*;
}

#[cfg(feature = "qinetic_ai")]
pub mod ai {
    //! Artificial intelligence functionality.

    pub use qinetic_ai::*;
}

#[cfg(feature = "qinetic_animation")]
pub mod animation {
    //! Animation functionality.

    pub use qinetic_animation::*;
}

#[cfg(feature = "qinetic_audio")]
pub mod audio {
    //! Audio functionality.

    pub use qinetic_audio::*;
}

#[cfg(feature = "qinetic_ar")]
pub mod ar {
    //! Augmented reality functionality.

    pub use qinetic_ar::*;
}

#[cfg(feature = "qinetic_log")]
pub mod log {
    //! Log functionality.

    pub use qinetic_log::*;
}

#[cfg(feature = "qinetic_network")]
pub mod network {
    //! Network functionality.

    pub use qinetic_network::*;
}

#[cfg(feature = "qinetic_physics")]
pub mod physics {
    //! Physics functionality.

    pub use qinetic_physics::*;
}

#[cfg(feature = "qinetic_render")]
pub mod render {
    //! Render functionality.

    pub use qinetic_render::*;
}

#[cfg(feature = "qinetic_ui")]
pub mod ui {
    //! User-interface functionality.
    pub use qinetic_ui::*;
}

#[cfg(feature = "qinetic_vr")]
pub mod vr {
    //! Virtual reality functionality.

    pub use qinetic_vr::*;
}

mod default_plugins;
pub use default_plugins::*;

mod default_runners;
pub use default_runners::*;
