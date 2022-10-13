//! [![](https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic-ecs is crate for Qinetic, containing entity-component-system functionality.

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
    fn build(&self, app: &mut App) {}
}
