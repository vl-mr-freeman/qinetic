//! [![](https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic-core is crate for Qinetic, containing core functionality.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Provides main core functionality.
    #[doc(hidden)]
    pub use crate::CorePlugin;
}

use qinetic_app::prelude::*;

/// Adds core functionality to [`App`].
#[derive(Default)]
pub struct CorePlugin {}

impl Plugin for CorePlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {}
}
