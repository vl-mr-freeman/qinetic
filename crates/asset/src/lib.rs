//! [![](https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Asset functionality for Qinetic.
//!
//! # Examples
//!
//! Here is a simple asset application:
//! ```
//! use qinetic_app::prelude::*;
//! use qinetic_asset::prelude::*;
//!
//! fn main() {
//!     App::builder()
//!         .with_plugin(AssetPlugin::default())
//!         .build()
//!         .unwrap()
//!         .run();
//! }
//! ```

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod plugins;
pub mod stages;
pub mod systems;

pub mod prelude {
    //! Main asset functionality.

    #[doc(hidden)]
    pub use crate::{
        plugins::AssetPlugin,
        stages::{AssetStage, AssetStageGroup},
        systems::AssetSystem,
    };
}
