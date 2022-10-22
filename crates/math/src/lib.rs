//! [![](https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_logo.png)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic-math is crate for Qinetic, containing math functionality.

#![doc(
    html_logo_url = "https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_icon.png",
    html_favicon_url = "https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_icon.png"
)]

/// Provides 4-dimensional vector.
pub mod vec4;

/// Provides 4-dimensional boolean vector.
pub mod bvec4;

/// Provides 3-dimensional vector.
pub mod vec3;

/// Provides 3-dimensional boolean vector.
pub mod bvec3;

/// Provides 2-dimensional vector.
pub mod vec2;

/// Provides 2-dimensional boolean vector.
pub mod bvec2;

/// Provides 4x4 column major matrix.
pub mod mat4;

/// Provides 3x3 column major matrix.
pub mod mat3;

/// Provides 2x2 column major matrix.
pub mod mat2;

/// Provides n-dimensional vector.
pub mod vec;

/// Provides n-dimensional boolean vector.
pub mod bvec;

/// Provides nxn column major matrix.
pub mod mat;

pub mod prelude {
    //! Provides main math functionality.
    #[doc(hidden)]
    pub use crate::{
        bvec2::*, bvec3::*, bvec4::*, mat2::*, mat3::*, mat4::*, vec2::*, vec3::*, vec4::*,
    };
}
