//! Asset functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Provides main asset functionality.
    #[doc(hidden)]
    pub use crate::{AssetPlugin, AssetStage};
}

use qinetic_app::prelude::*;

/// Adds asset functionality to [`App`]
#[derive(Default)]
pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder.with_stage(AssetStage::default());
    }
}

/// [`App`]'s asset step of execution cycle.
#[derive(Default)]
pub struct AssetStage;

impl Stage for AssetStage {}
