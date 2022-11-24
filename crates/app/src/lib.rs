//! Application functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Provides main application functionality.
    #[doc(hidden)]
    pub use crate::{app::*, event::*, plugin::*, resource::*, schedule::*, stage::*, state::*};

    #[doc(hidden)]
    pub use qinetic_app_macros::*;
}

/// Provides application.
pub mod app;

/// Provides application plugin.
pub mod plugin;

/// Provides application stage.
pub mod stage;

/// Provides application state.
pub mod state;

/// Provides application resource.
pub mod resource;

/// Provides application stage's schedule.
pub mod schedule;

/// Provides application event.
pub mod event;
