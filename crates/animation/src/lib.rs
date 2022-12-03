//! Animation functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Main animation functionality.

    #[doc(hidden)]
    pub use crate::{Animation, AnimationPlugin, AnimationStage, Animator, SkeletalMesh};
}

use qinetic_app::prelude::*;
use qinetic_ecs::prelude::*;

/// Animation functionality for [`App`].
///
/// [`Component`]s:
/// * [`SkeletalMesh`]
/// * [`Animation`]
/// * [`Animator`]
///
/// [`Stage`]s:
/// * [`AnimationStage`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_animation::prelude::*;
/// #
/// App::builder().with_plugin(AnimationPlugin::default()).build().run();
/// ```
#[derive(Default)]
pub struct AnimationPlugin {}

impl Plugin for AnimationPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder.with_component(SkeletalMesh::default());
        app_builder.with_component(Animation::default());
        app_builder.with_component(Animator::default());
        app_builder.with_stage(AnimationStage::default());
    }
}

/// Animation [`Stage`] for [`App`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_animation::prelude::*;
/// #
/// App::builder().with_stage(AnimationStage::default()).build().run();
/// ```
#[derive(Default, Stage)]
pub struct AnimationStage {}

/// Skeletal mesh [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_animation::prelude::*;
/// #
/// App::builder().with_component(SkeletalMesh::default()).build().run();
/// ```
#[derive(Default, Component)]
pub struct SkeletalMesh {}

/// Animation [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_animation::prelude::*;
/// #
/// App::builder().with_component(Animation::default()).build().run();
/// ```
#[derive(Default, Component)]
pub struct Animation {}

/// Animator [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_animation::prelude::*;
/// #
/// App::builder().with_component(Animator::default()).build().run();
/// ```
#[derive(Default, Component)]
pub struct Animator {}
