//! [![](https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic-network is crate for Qinetic, containing network functionality.

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
