//! Log functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
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
