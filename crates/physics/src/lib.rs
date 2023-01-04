//! [![](https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Physics functionality for Qinetic.
//!
//! # Examples
//!
//! Here is a simple physics application:
//! ```
//! use qinetic_app::prelude::*;
//! use qinetic_physics::prelude::*;
//!
//! fn main() {
//!     App::builder()
//!         .with_plugin(PhysicsPlugin::default())
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
pub mod resources;
pub mod stages;
pub mod systems;

pub mod prelude {
    //! Main physics functionality.

    #[doc(hidden)]
    pub use crate::{
        components::{
            BoxCollider, CapsuleCollider, Cloth, MeshCollider, Rigidbody, Softbody, SphereCollider,
        },
        plugins::PhysicsPlugin,
        resources::{PhysicsResource, PhysicsResourceBuilder, PhysicsResourceBuilderError},
        stages::{PhysicsStage, PhysicsStageGroup},
        systems::PhysicsSystem,
    };
}
