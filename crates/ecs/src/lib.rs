//! Entity-component-system functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Main entity-component-system functionality.

    #[doc(hidden)]
    pub use crate::{
        component::*, entity::*, event::*, resource::*, state::*, system::*, world::*,
    };

    #[doc(hidden)]
    pub use qinetic_ecs_macros::*;
}

pub mod component;
pub mod entity;
pub mod event;
pub mod resource;
pub mod state;
pub mod system;
pub mod world;
