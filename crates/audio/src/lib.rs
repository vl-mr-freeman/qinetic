//! Audio functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Provides main audio functionality.
    #[doc(hidden)]
    pub use crate::{AudioPlugin, AudioStage, Sound};
}

use qinetic_app::prelude::*;
use qinetic_ecs::prelude::*;

/// Adds audio functionality to [`App`]
#[derive(Default)]
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder.with_component(Sound::default());
        app_builder.with_stage(AudioStage::default());
    }
}

/// [`App`]'s audio step of execution cycle.
#[derive(Default)]
pub struct AudioStage;

impl Stage for AudioStage {}

#[derive(Default, Component)]
/// Sound [`Component`].
pub struct Sound {}
