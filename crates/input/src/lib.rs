//! [![](https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic-input is crate for Qinetic, containing input functionality.

pub mod prelude {
    //! Provides main input functionality.
    #[doc(hidden)]
    pub use crate::InputPlugin;
}

use qinetic_app::prelude::*;

/// Adds input functionality to [`App`]
#[derive(Default)]
pub struct InputPlugin {}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {}
}
