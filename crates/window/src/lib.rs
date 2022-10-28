//! Window functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
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
