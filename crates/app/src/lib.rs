//! [![](https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic-application is crate for Qinetic, containing application functionality.

pub mod prelude {
    //! Provides main application functionality.
    #[doc(hidden)]
    pub use crate::{app::*, plugin::*};
}

/// Provides application.
pub mod app;

/// Provides application plugin.
pub mod plugin;
