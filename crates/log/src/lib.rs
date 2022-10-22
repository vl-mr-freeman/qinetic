//! [![](https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_logo.png)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic-log is crate for Qinetic, containing log functionality.

#![doc(
    html_logo_url = "https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_icon.png",
    html_favicon_url = "https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_icon.png"
)]

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
