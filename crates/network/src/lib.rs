//! Network functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Provides main network functionality.
    #[doc(hidden)]
    pub use crate::NetworkPlugin;
}

use qinetic_app::prelude::*;

/// Adds network functionality to [`App`]
#[derive(Default)]
pub struct NetworkPlugin {}

impl Plugin for NetworkPlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {}
}
