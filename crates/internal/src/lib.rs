#![warn(missing_docs)]

//! [![](https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic-internal is crate for Qinetic, facilitating optional dynamic linking.

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

#[cfg(any(feature = "qinetic_animation", doc))]
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

#[cfg(any(feature = "qinetic_audio", doc))]
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

#[cfg(any(feature = "qinetic_network", doc))]
pub mod network {
    //! Provides network functionality.
    pub use qinetic_network::*;
}

#[cfg(any(feature = "qinetic_physics", doc))]
pub mod physics {
    //! Provides physics functionality.
    pub use qinetic_physics::*;
}

#[cfg(any(feature = "qinetic_render", doc))]
pub mod render {
    //! Provides render functionality.
    pub use qinetic_render::*;
}

#[cfg(any(feature = "qinetic_ui", doc))]
pub mod ui {
    //! Provides user-interface functionality.
    pub use qinetic_ui::*;
}

pub mod window {
    //! Provides window functionality.
    pub use qinetic_window::*;
}

mod default_plugins;
pub use default_plugins::*;
