//! [![](https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic-ui is crate for Qinetic, containing user-interface functionality.

pub mod prelude {
    //! Provides main user-interface functionality.
    #[doc(hidden)]
    pub use crate::UiPlugin;
}

use qinetic_app::prelude::*;

/// Adds ui functionality to [`App`]
#[derive(Default)]
pub struct UiPlugin {}

impl Plugin for UiPlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {}
}
