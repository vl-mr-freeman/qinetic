//! [![](https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic-physics is crate for Qinetic, containing physics functionality.

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
