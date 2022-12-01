//! Input functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Main input functionality.

    #[doc(hidden)]
    pub use crate::{InputPlugin, InputStage, PlayerController};
}

use qinetic_app::prelude::*;
use qinetic_ecs::prelude::*;

/// Input functionality for [`App`]
///
/// [`Component`]s:
/// * [`PlayerController`]
///
/// [`Stage`]s:
/// * [`InputStage`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_input::prelude::*:
/// #
/// App::builder().with_plugin(InputPlugin::default()).build().run();
/// ```
#[derive(Default)]
pub struct InputPlugin {}

impl Plugin for InputPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder.with_component(PlayerController::default());
        app_builder.with_stage(InputStage::default());
    }
}

/// Input [`Stage`] for [`App`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_input::prelude::*;
/// #
/// App::builder().with_stage(InputStage::default()).build().run();
/// ```
#[derive(Default, Stage)]
pub struct InputStage;

/// Player controller component.
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_player::prelude::*;
/// #
/// App::builder().with_component(PlayerController::default()).build().run();
/// ```
#[derive(Default, Component)]
pub struct PlayerController {}
