//! Input functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

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
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {}
}
