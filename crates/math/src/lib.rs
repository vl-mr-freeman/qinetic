//! Math functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Main math functionality.

    #[doc(hidden)]
    pub use crate::{bvector::*, matrix::*, quaternion::*, vector::*, Transform};
}

pub mod vector {
    //! Vector functionality.

    pub use crate::{vec2::*, vec3::*, vec4::*};
}

pub mod bvector {
    //! Boolean vector functionality.

    pub use crate::{bvec2::*, bvec3::*, bvec4::*};
}

pub mod matrix {
    //! Matrix functionality.

    pub use crate::{mat2::*, mat3::*, mat4::*};
}

pub mod quaternion {
    //! Quaternion functionality.

    pub use crate::quat::*;
}

mod vec2;
mod vec3;
mod vec4;

mod bvec2;
mod bvec3;
mod bvec4;

mod mat2;
mod mat3;
mod mat4;

mod quat;

use crate::vector::Vec3;
use qinetic_app::prelude::*;
use qinetic_ecs::prelude::*;

/// Math functionality for [`App`].
///
/// [`Component`]s:
/// * [`Transform`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_math::prelude::*;
///`#
/// App::builder().with_plugin(MathPlugin::default()).build().run();
/// ```
#[derive(Default)]
pub struct MathPlugin {}

impl Plugin for MathPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder.with_component(Transform::default());
    }
}

/// Transform [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_math::prelude::*;
/// #
/// App::builder().with_component(Transform::default()).build().run();
/// ```
#[derive(Default, Component)]
pub struct Transform {
    /// Position of the [`Entity`] in the [`World`].
    pub position: Vec3,

    /// Rotation of the [`Entity`] in the [`World`].
    pub rotation: Vec3,

    /// Scale of the [`Entity`] in the [`World`].
    pub scale: Vec3,
}
