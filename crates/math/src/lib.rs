//! Math functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Main math functionality.

    #[doc(hidden)]
    pub use crate::{matrix::*, quaternion::*, vector::*, Transform};
}

pub mod matrix;
pub mod point;
pub mod quaternion;
pub mod vector;

mod macros;

pub(crate) use macros::*;

use crate::vector::Vector3;
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
    pub position: Vector3<f32>,

    /// Rotation of the [`Entity`] in the [`World`].
    pub rotation: Vector3<f32>,

    /// Scale of the [`Entity`] in the [`World`].
    pub scale: Vector3<f32>,
}

use num_traits::{Float, Num};

pub trait Digit: Copy + Sized {}

macro_rules! impl_digit {
    ($t:ty) => {
        impl Digit for $t {}
    };
}

pub trait DigitNum: Digit + Num {}

macro_rules! impl_digit_num {
    ($t:ty) => {
        impl DigitNum for $t {}
    };
}

pub trait DigitFloat: DigitNum + Float {}

macro_rules! impl_digit_float {
    ($t:ty) => {
        impl DigitFloat for $t {}
    };
}

impl_digit!(usize);
impl_digit_num!(usize);

impl_digit!(u8);
impl_digit_num!(u8);

impl_digit!(u16);
impl_digit_num!(u16);

impl_digit!(u32);
impl_digit_num!(u32);

impl_digit!(u64);
impl_digit_num!(u64);

impl_digit!(isize);
impl_digit_num!(isize);

impl_digit!(i8);
impl_digit_num!(i8);

impl_digit!(i16);
impl_digit_num!(i16);

impl_digit!(i32);
impl_digit_num!(i32);

impl_digit!(i64);
impl_digit_num!(i64);

impl_digit!(f32);
impl_digit_num!(f32);
impl_digit_float!(f32);

impl_digit!(f64);
impl_digit_num!(f64);
impl_digit_float!(f64);

impl_digit!(bool);
