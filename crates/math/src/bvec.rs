#[doc(hidden)]
pub use std::fmt;

#[doc(hidden)]
pub use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

/// A n-dimensional boolean vector.
pub trait BVec:
    Clone
    + Copy
    + Default
    + PartialEq
    + BitAnd
    + BitAnd<bool>
    + BitAndAssign
    + BitAndAssign<bool>
    + BitOr
    + BitOr<bool>
    + BitOrAssign
    + BitOrAssign<bool>
    + BitXor
    + BitXor<bool>
    + BitXorAssign
    + BitXorAssign<bool>
    + Not
    + fmt::Display
    + fmt::Debug
{
    /// Returns a bitmask with the lowest 2 bits set from the elements of `self`.
    fn bitmask(self) -> u32;

    /// Any values of `self` are true.
    fn any(self) -> bool;

    /// All values of `self` are true.
    fn all(self) -> bool;
}
