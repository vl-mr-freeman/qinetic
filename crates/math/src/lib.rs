//! [![](https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Math functionality for Qinetic.
//!
//! # Examples
//!
//! Here is a simple math application:
//! ```
//! use qinetic_app::prelude::*;
//! use qinetic_math::prelude::*;
//!
//! fn main() {
//!     App::builder()
//!         .with_plugin(MathPlugin::default())
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
pub mod digit;
pub mod matrix;
pub mod plugins;
pub mod point;
pub mod quaternion;
pub mod vector;

pub mod prelude {
    //! Main math functionality.

    #[doc(hidden)]
    pub use crate::{
        components::Transform, matrix::*, plugins::MathPlugin, point::*, quaternion::*, vector::*,
    };
}

pub(crate) use crate::macros::*;

mod macros;
