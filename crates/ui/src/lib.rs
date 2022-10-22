//! [![](https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_logo.png)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic-ui is crate for Qinetic, containing user-interface functionality.

#![doc(
    html_logo_url = "https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_icon.png",
    html_favicon_url = "https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_icon.png"
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
