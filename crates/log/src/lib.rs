//! [![](https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic-log is crate for Qinetic, containing log functionality.

pub mod prelude {
    //! Provides main log functionality.
    #[doc(hidden)]
    pub use crate::LogPlugin;
}

use qinetic_app::prelude::*;

/// Adds log functionality to [`App`]
#[derive(Default)]
pub struct LogPlugin {}

impl Plugin for LogPlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {}
}
