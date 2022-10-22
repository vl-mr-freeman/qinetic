//! [![](https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_logo.png)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic-ecs is crate for Qinetic, containing entity-component-system functionality.

#![doc(
    html_logo_url = "https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_icon.png",
    html_favicon_url = "https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_icon.png"
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
