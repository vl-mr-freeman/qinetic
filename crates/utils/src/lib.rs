//! Utils for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Main utils functionality.

    #[doc(hidden)]
    pub use crate::{allocator::*, label::*};
}

pub mod allocator {
    //! Allocator functionality.

    pub use crate::{linear_allocator::*, pool_allocator::*, stack_allocator::*};
}

pub mod label;

mod linear_allocator;
mod pool_allocator;
mod stack_allocator;
