//! [![](https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_logo.png)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic-asset is crate for Qinetic, containing asset functionality.

#![doc(
    html_logo_url = "https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_icon.png",
    html_favicon_url = "https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_icon.png"
)]

pub mod prelude {
    //! Provides main asset functionality.
    #[doc(hidden)]
    pub use crate::AssetPlugin;
}

use qinetic_app::prelude::*;

/// Adds asset functionality to [`App`]
#[derive(Default)]
pub struct AssetPlugin {}

impl Plugin for AssetPlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {}
}
