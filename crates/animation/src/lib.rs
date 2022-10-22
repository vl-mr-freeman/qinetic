//! [![](https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_logo.png)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic-animation is crate for Qinetic, containing animation functionality.

#![doc(
    html_logo_url = "https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_icon.png",
    html_favicon_url = "https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_icon.png"
)]

pub mod prelude {
    //! Provides main animation functionality.
    #[doc(hidden)]
    pub use crate::AnimationPlugin;
}

use qinetic_app::prelude::*;

/// Adds animation functionality to [`App`]
#[derive(Default)]
pub struct AnimationPlugin {}

impl Plugin for AnimationPlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {}
}
