//! [![](https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_logo.png)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic-network is crate for Qinetic, containing network functionality.

#![doc(
    html_logo_url = "https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_icon.png",
    html_favicon_url = "https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_icon.png"
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
