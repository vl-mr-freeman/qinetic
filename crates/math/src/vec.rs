pub use std::fmt;
pub use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign,
};

/// A n-dimensional vector.
pub trait Vec:
    Add
    + AddAssign
    + Div
    + DivAssign
    + Mul
    + MulAssign
    + Rem
    + RemAssign
    + Sub
    + SubAssign
    + fmt::Display
    + Sized
{
}
