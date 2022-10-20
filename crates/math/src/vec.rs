#[doc(hidden)]
pub use std::fmt;
#[doc(hidden)]
pub use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem, RemAssign, Sub,
    SubAssign, 
};
#[doc(hidden)]
pub use std::iter::{Sum, Product};

/// A n-dimensional vector.
pub trait Vec:
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
    /// Absolutes values of `self`.
    fn abs(self) -> Self;

    /// Represents the sign of `self`s values.
    fn signum(self) -> Self;

    /// Minimizes values of `self` with `other`.
    fn min(self, other: Self) -> Self;

    /// Maximizes values of `self` with `other`.
    fn max(self, other: Self) -> Self;

    /// Clamps values of `self` between `min` and `max`.
    fn clamp(self, min: Self, max: Self) -> Self;

    /// Performs a linear interpolation beetween `self` and `other` based on the value `s`.
    fn lerp(self, other: Self, s: f32) -> Self;

    /// Computes the dot product of `self` and `other`.
    fn dot(self, other: Self) -> f32;

    /// Computes the length of `self`.
    #[inline]
    fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    /// Computes the squared length of `self`.
    #[inline]
    fn length_squared(self) -> f32 {
        self.dot(self)
    }

    /// Computes `1.0 / length()`.
    #[inline]
    fn length_recip(self) -> f32 {
        self.length().recip()
    }

    /// Computes the Euclidean distance.
    fn distance(self, other: Self) -> f32;

    /// Computes the squared Euclidean distance.
    fn distance_squared(self, other: Self) -> f32;

    /// Returns a vector containing the nearest integer to a number for values of `self`.
    fn round(self) -> Self;

    /// Returns a vector containing the largest integer less than or equal to a number for values of `self`.
    fn floor(self) -> Self;

    /// Returns a vector containing the smallest integer greater than or equal to a number for values of `self`.
    fn ceil(self) -> Self;

    /// Fractionals values of `self`.
    fn fract(self) -> Self;

    /// Exponential function for values of `self`.
    fn exp(self) -> Self;

    /// Raises values of `self` to the power of `n`.
    fn powf(self, n: f32) -> Self;

    /// Reciprocals values of `self`
    fn recip(self) -> Self;

    /// Normalizes the length of `self`.
    fn normalize(self) -> Self;

    // Returns whether `self` is length `1.0` or not.
    fn is_normalized(self) -> bool;

    ///  Returns a bitmask with the lowest bits set to the sign bits from the elements of `self`.
    fn is_negative_bitmask(self) -> u32;

    /// Returns `true` if, and only if, all elements are finite.
    fn is_finite(self) -> bool;

    /// Returns `true` if any elements are `NaN`.
    fn is_nan(self) -> bool;
}