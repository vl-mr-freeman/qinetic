//! [![](https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic-ui is crate for Qinetic, containing user-interface functionality.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

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
