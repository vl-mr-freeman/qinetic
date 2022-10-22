//! [![](https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic-render is crate for Qinetic, containing render functionality.

pub mod prelude {
    //! Provides main render functionality.
    #[doc(hidden)]
    pub use crate::RenderPlugin;
}

use qinetic_app::prelude::*;

/// Adds render functionality to [`App`]
#[derive(Default)]
pub struct RenderPlugin {}

impl Plugin for RenderPlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {}
}
