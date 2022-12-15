//! Augmented reality functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Main augmented reality functionality.

    #[doc(hidden)]
    pub use crate::{ArController, ArPlugin, ArStage, ArTracker};
}

use qinetic_app::prelude::*;
use qinetic_ecs::prelude::*;

/// Augmented reality [`Plugin`] for [`App`].
///
/// [`Component`]s:
/// * [`ArController`]
/// * [`ArTracker`]
///
/// [`Stage`]s:
/// * [`ArStage`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_vr::prelude::*;
/// #
/// App::builder().with_plugin(ArPlugin::default()).build().run();
/// ```
#[derive(Default)]
pub struct ArPlugin {}

impl Plugin for ArPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder.with_stage(ArStage::default());
    }
}

/// Virtual reality [`Stage`] for [`App`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ai::prelude::*;
/// #
/// App::builder().with_stage(ArStage::default()).build().run();
/// ```
#[derive(Default, Stage)]
pub struct ArStage {}

/// Virtual reality controller [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_vr::prelude::*;
/// #
/// App::builder().with_component(ArController::default()).build().run();
/// ```
pub struct ArController {}

/// Virtual reality tracker [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_vr::prelude::*;
/// #
/// App::builder().with_component(Tracker::default()).build().run();
/// ```
#[derive(Default, Component)]
pub struct ArTracker {}
