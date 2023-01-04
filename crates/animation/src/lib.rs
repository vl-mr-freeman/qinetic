//! [![](https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Animation functionality for Qinetic.
//!
//! # Examples
//!
//! Here is a simple animation application:
//! ```
//! use qinetic_app::prelude::*;
//! use qinetic_animation::prelude::*;
//!
//! fn main() {
//!     App::builder()
//!         .with_plugin(AnimationPlugin::default())
//!         .build()
//!         .unwrap()
//!         .run();
//! }
//! ```

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod components;
pub mod plugins;
pub mod stages;
pub mod systems;

pub mod prelude {
    //! Main animation functionality.

    #[doc(hidden)]
    pub use crate::{
        components::{AnimationClip, Animator, SkeletalMesh},
        plugins::AnimationPlugin,
        stages::{AnimationStage, AnimationStageGroup},
        systems::AnimationSystem,
    };
}
