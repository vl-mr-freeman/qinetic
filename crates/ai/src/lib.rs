//! Artificial Intelligence functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    /// Provides main artificial intelligence functionality.
    #[doc(hidden)]
    pub use crate::{AiController, AiPlugin, AiStage};
}

use qinetic_app::prelude::*;
use qinetic_ecs::prelude::*;

/// Adds artificial intelligence functionality to [`App`].
#[derive(Default)]
pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder.with_component(AiController::default());
        app_builder.with_stage(AiStage::default());
    }
}

/// [`App`]'s ai step of execution cycle.
#[derive(Default)]
pub struct AiStage;

impl Stage for AiStage {}

#[derive(Default, Component)]
/// Artificial intelligence controller [`Component`].
pub struct AiController {}
