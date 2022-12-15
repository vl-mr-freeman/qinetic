//! Artificial Intelligence functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Main artificial intelligence functionality.

    #[doc(hidden)]
    pub use crate::{AiController, AiPlugin, AiStage};
}

use qinetic_app::prelude::*;
use qinetic_ecs::prelude::*;

/// Artificial intelligence [`Plugin`] for [`App`].
///
/// [`Component`]s:
/// * [`AiController`]
///
/// [`Stage`]s:
/// * [`AiStage`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ai::prelude::*;
/// #
/// App::builder().with_plugin(AiPlugin::default()).build().run();
/// ```
#[derive(Default)]
pub struct AiPlugin {}

impl Plugin for AiPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder.with_component(AiController::default());
        app_builder.with_stage(AiStage::default());
    }
}

/// Artificial intelligence [`Stage`] for [`App`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ai::prelude::*;
/// #
/// App::builder().with_stage(AiStage::default()).build().run();
/// ```
#[derive(Default, Stage)]
pub struct AiStage {}

/// Artificial intelligence controller [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ai::prelude::*;
/// #
/// App::builder().with_component(AiController::default()).build().run();
/// ```
#[derive(Default, Component)]
pub struct AiController {}
