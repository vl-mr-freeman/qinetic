//! Physics functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Provides main physics functionality.
    #[doc(hidden)]
    pub use crate::PhysicsPlugin;
}

use qinetic_app::prelude::*;

/// Adds physics functionality to [`App`]
#[derive(Default)]
pub struct PhysicsPlugin {}

impl Plugin for PhysicsPlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {}
}
