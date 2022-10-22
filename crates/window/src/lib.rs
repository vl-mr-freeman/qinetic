//! [![](https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_logo.png)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic-window is crate for Qinetic, containing window functionality.

#![doc(
    html_logo_url = "https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_icon.png",
    html_favicon_url = "https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_icon.png"
)]

pub mod prelude {
    //! Provides main window functionality.
    #[doc(hidden)]
    pub use crate::WindowPlugin;
}

use qinetic_app::prelude::*;

/// Adds window functionality to [`App`]
#[derive(Default)]
pub struct WindowPlugin {}

impl Plugin for WindowPlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {}
}
