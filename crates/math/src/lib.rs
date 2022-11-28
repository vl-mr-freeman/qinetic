//! Math functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Provides main math functionality.
    #[doc(hidden)]
    pub use crate::{bvector::*, matrix::*, quaternion::*, vector::*, Transform};
}

pub mod vector {
    //! Provides vector.
    pub use crate::{vec2::*, vec3::*, vec4::*};
}

pub mod bvector {
    //! Provides boolean vector.
    pub use crate::{bvec2::*, bvec3::*, bvec4::*};
}

pub mod matrix {
    //! Provides matrix.
    pub use crate::{mat2::*, mat3::*, mat4::*};
}

pub mod quaternion {
    //! Provides quaternion.
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

/// Adds math functionality to [`App`].
#[derive(Default)]
pub struct MathPlugin;

impl Plugin for MathPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder.with_component(Transform::default());
    }
}

/// Transform [`Component`].
#[derive(Default, Component)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}
