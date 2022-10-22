//! [![](https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_logo.png)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic-input is crate for Qinetic, containing input functionality.

#![doc(
    html_logo_url = "https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_icon.png",
    html_favicon_url = "https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_icon.png"
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
