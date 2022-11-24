//! Input functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Provides main input functionality.
    #[doc(hidden)]
    pub use crate::{InputPlugin, InputStage, PlayerController};
}

use qinetic_app::prelude::*;
use qinetic_ecs::prelude::*;

/// Adds input functionality to [`App`]
#[derive(Default)]
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder.with_component(PlayerController::default());
        app_builder.with_stage(InputStage::default());
    }
}

/// [`App`]'s input step of execution cycle.
#[derive(Default)]
pub struct InputStage;

impl Stage for InputStage {}

#[derive(Default, Component)]
/// Player controller component.
pub struct PlayerController {}
