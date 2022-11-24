//! Animation functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Provides main animation functionality.
    #[doc(hidden)]
    pub use crate::{AnimationPlugin, AnimationStage, SkeletalMesh};
}

use qinetic_app::prelude::*;
use qinetic_ecs::prelude::*;

/// Adds animation functionality to [`App`]
#[derive(Default)]
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder.with_component(SkeletalMesh::default());
        app_builder.with_stage(AnimationStage::default());
    }
}

/// [`App`]'s animation step of execution cycle.
#[derive(Default)]
pub struct AnimationStage;

impl Stage for AnimationStage {}

#[derive(Default, Component)]
/// Skeletal mesh [`Component`].
pub struct SkeletalMesh {}
