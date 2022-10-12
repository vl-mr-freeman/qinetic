//! [![](https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_logo2.png)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic-internal separated into its own crate to enable simple dynamic linking for Qinetic, and should not be used directly.

pub mod prelude {
    //! Provides main functionality.
    #[doc(hidden)]
    pub use crate::{core::prelude::*, math::prelude::*};
}

pub mod core {
    //! Provides core functionality.
    pub use qinetic_core::*;
}

pub mod math {
    //! Provides math functionality.
    pub use qinetic_math::*;
}
