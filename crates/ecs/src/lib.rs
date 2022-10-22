//! [![](https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic-ecs is crate for Qinetic, containing entity-component-system functionality.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Provides main entity-component-system functionality.
    #[doc(hidden)]
    pub use crate::EcsPlugin;
}

use qinetic_app::prelude::*;

/// Adds entity-component-system functionality to [`App`]
#[derive(Default)]
pub struct EcsPlugin {}

impl Plugin for EcsPlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {}
}
