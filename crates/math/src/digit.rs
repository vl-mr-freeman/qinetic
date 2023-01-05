//! Digti functionality.

use num_traits::{Float, Num};

pub trait Digit: Default + std::fmt::Display + std::fmt::Debug + Copy + Sized {}

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
