//! [![](https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Application functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod app;
pub mod plugin;
pub mod runner;
pub mod schedule;
pub mod stage;

pub mod prelude {
    //! Main application functionality.

    #[doc(hidden)]
    pub use crate::{app::*, plugin::*, runner::*, schedule::*, stage::*};

    #[doc(hidden)]
    pub use qinetic_app_macros::*;
}
