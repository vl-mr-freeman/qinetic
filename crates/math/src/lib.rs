//! [![](https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic-math is crate for Qinetic, containing math functionality.

/// Provides 4-dimensional vector.
pub mod vec4;

/// Provides 3-dimensional vector.
pub mod vec3;

/// Provides 2-dimensional vector.
pub mod vec2;

/// Provides n-dimensional vector.
pub mod vec;

pub mod prelude {
    //! Provides main math functionality.
    #[doc(hidden)]
    pub use crate::{vec2::*, vec3::*, vec4::*};
}
