//! Render functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Provides main render functionality.
    #[doc(hidden)]
    pub use crate::RenderPlugin;
}

use qinetic_app::prelude::*;

/// Adds render functionality to [`App`]
#[derive(Default)]
pub struct RenderPlugin {}

impl Plugin for RenderPlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {}
}
