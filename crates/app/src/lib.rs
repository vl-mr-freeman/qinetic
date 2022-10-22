//! [![](https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_logo.png)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic-application is crate for Qinetic, containing application functionality.

#![doc(
    html_logo_url = "https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_icon.png",
    html_favicon_url = "https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_icon.png"
)]

pub mod prelude {
    //! Provides main application functionality.
    #[doc(hidden)]
    pub use crate::{app::*, plugin::*};
}

/// Provides application.
pub mod app;

/// Provides application plugin.
pub mod plugin;
