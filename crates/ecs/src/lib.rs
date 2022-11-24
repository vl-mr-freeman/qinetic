//! Entity-component-system functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Provides main entity-component-system functionality.
    #[doc(hidden)]
    pub use crate::{component::*, entity::*, system::*, world::*};

    #[doc(hidden)]
    pub use qinetic_ecs_macros::*;
}

/// Provides ecs world.
pub mod world;

/// Provides ecs entity.
pub mod entity;

/// Provides ecs componenen.
pub mod component;

/// Provides ecs system.
pub mod system;
