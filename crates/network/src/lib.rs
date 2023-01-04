//! [![](https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Network functionality for Qinetic.
//!
//! # Examples
//!
//! Here is a simple network application:
//! ```
//! use qinetic_app::prelude::*;
//! use qinetic_network::prelude::*;
//!
//! fn main() {
//!     App::builder()
//!         .with_plugin(NetworkPlugin::default())
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
    //! Main network functionality.

    #[doc(hidden)]
    pub use crate::{
        plugins::NetworkPlugin,
        stages::{NetworkStage, NetworkStageGroup},
        systems::NetworkSystem,
    };
}
