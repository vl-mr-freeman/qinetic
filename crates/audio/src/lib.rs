//! [![](https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_logo.png)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic-audio is crate for Qinetic, containing audio functionality.

#![doc(
    html_logo_url = "https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_icon.png",
    html_favicon_url = "https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_icon.png"
)]

pub mod prelude {
    //! Provides main audio functionality.
    #[doc(hidden)]
    pub use crate::AudioPlugin;
}

use qinetic_app::prelude::*;

/// Adds audio functionality to [`App`]
#[derive(Default)]
pub struct AudioPlugin {}

impl Plugin for AudioPlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {}
}
