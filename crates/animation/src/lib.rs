//! Animation functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
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
