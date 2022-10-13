//! [![](https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic-asset is crate for Qinetic, containing asset functionality.

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
    fn build(&self, app: &mut App) {}
}
