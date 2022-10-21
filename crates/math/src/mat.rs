#[doc(hidden)]
pub use std::fmt;
#[doc(hidden)]
pub use std::iter::{Product, Sum};
#[doc(hidden)]
pub use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem, RemAssign, Sub,
    SubAssign,
};

/// A nxn column major matrix.
pub trait Mat:
    Clone
    + Copy
    + Default
    + PartialEq
    + PartialOrd
    + Add
    + Add<f32>
    + AddAssign
    + AddAssign<f32>
    + Div
    + Div<f32>
    + DivAssign
    + DivAssign<f32>
    + Mul
    + Mul<f32>
    + MulAssign
    + MulAssign<f32>
    + Rem
    + Rem<f32>
    + RemAssign
    + RemAssign<f32>
    + Sub
    + Sub<f32>
    + SubAssign
    + SubAssign<f32>
    + Neg
    + Index<usize>
    + IndexMut<usize>
    + fmt::Display
    + fmt::Debug
    + Sum
    + Product
    + Sized
{
}
