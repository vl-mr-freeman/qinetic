//! Asset functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Main asset functionality.

    #[doc(hidden)]
    pub use crate::{AssetPlugin, AssetStage};
}

use qinetic_app::prelude::*;
use qinetic_ecs::prelude::*;

/// Asset [`Plugin`] for [`App`]
///
/// [`Stage`]s:
/// * [`AssetStage`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_asset::prelude::*;
/// #
/// App::builder().with_plugin(AssetPlugin::default()).build().run();
/// ```
#[derive(Default)]
pub struct AssetPlugin {}

impl Plugin for AssetPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder.with_stage(AssetStage::default());
    }
}

/// Asset [`Stage`] for [`App`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_asset::prelude::*;
/// #
/// App::builder().with_stage(AssetStage::default()).build().run();
/// ```
#[derive(Default, Stage)]
pub struct AssetStage {}
