//! [![](https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_logo2.png)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic-core is shared library for Qinetic, containing core functionality.

/// Provides application.
pub mod application;

/// Provides application plugins.
pub mod plugin;

pub mod prelude {
    //! Provides main core functionality.
    #[doc(hidden)]
    pub use crate::{application::*, plugin::*};
}
