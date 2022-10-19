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

mod default_plugins;
pub use default_plugins::*;

mod default_stages;
pub use default_stages::*;

mod default_states;
pub use default_states::*;
