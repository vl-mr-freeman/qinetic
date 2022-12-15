//! Virtual reality functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Main virtual reality functionality.

    #[doc(hidden)]
    pub use crate::{VrPlugin, VrStage};
}

use qinetic_app::prelude::*;
use qinetic_ecs::prelude::*;

/// Virtual reality functionality for [`App`].
///
/// [`Stage`]s:
/// * [`VrStage`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_vr::prelude::*;
/// #
/// App::builder().with_plugin(VrPlugin::default()).build().run();
/// ```
#[derive(Default)]
pub struct VrPlugin {}

impl Plugin for VrPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder.with_stage(VrStage::default());
    }
}

/// Virtual reality [`Stage`] for [`App`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ai::prelude::*;
/// #
/// App::builder().with_stage(VrStage::default()).build().run();
/// ```
#[derive(Default, Stage)]
pub struct VrStage {}
