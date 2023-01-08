//! [![](https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Utils for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

extern crate proc_macro;

pub mod allocator;
pub mod label;
pub mod manifest;

pub mod prelude {
    //! Main utils functionality.

    #[doc(hidden)]
    pub use crate::{allocator::*, label::*};

    #[doc(hidden)]
    pub use getset::{CopyGetters, Getters, MutGetters, Setters};

    #[doc(hidden)]
    pub use derive_builder::*;

    #[doc(hidden)]
    pub use derive_more::*;

    #[doc(hidden)]
    pub use derive_new::new;

    #[doc(hidden)]
    pub use smart_default::SmartDefault;

    #[doc(hidden)]
    pub use downcast_rs::{impl_downcast, Downcast, DowncastSync};

    #[doc(hidden)]
    pub use static_assertions::{
        assert_cfg, assert_eq_align, assert_eq_size, assert_eq_size_ptr, assert_eq_size_val,
        assert_fields, assert_impl_all, assert_impl_any, assert_impl_one, assert_not_impl_all,
        assert_not_impl_any, assert_obj_safe, assert_trait_sub_all, assert_trait_super_all,
        assert_type_eq_all, assert_type_ne_all, const_assert, const_assert_eq, const_assert_ne,
    };

    #[doc(hidden)]
    pub use dyn_clone::{clone_trait_object, DynClone};

    #[doc(hidden)]
    pub use rgb::*;

    #[doc(hidden)]
    pub use derivative::Derivative;
}
