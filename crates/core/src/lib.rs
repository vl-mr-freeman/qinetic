//! Core functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Provides main core functionality.
    #[doc(hidden)]
    pub use crate::{color3::*, color4::*, CorePlugin};
}

/// Provides color(r,g,b) facilitate creating.
pub mod color3;

/// Provides color(r,g,b,a) facilitate creating.
pub mod color4;

use qinetic_app::prelude::*;

/// Adds core functionality to [`App`].
#[derive(Default)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app_builder: &mut AppBuilder) {}
}
