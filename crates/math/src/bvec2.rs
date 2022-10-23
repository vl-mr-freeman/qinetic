use crate::{bvec3::BVec3, bvec4::BVec4};
use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

/// A 2-dimensional boolean vector.
#[derive(Clone, Copy, PartialEq, Default)]
pub struct BVec2 {
    pub x: bool,
    pub y: bool,
}

impl BVec2 {
    /// All values of [`BVec2`] are false.
    pub const FALSE: Self = Self::splat(false);

    /// All values of [`BVec2`] are true.
    pub const TRUE: Self = Self::splat(true);

    /// Returns a [`BVec2`] with given values.
    #[inline(always)]
    pub const fn new(x: bool, y: bool) -> Self {
        Self { x: x, y: y }
    }

    /// Returns a [`BVec2`] with all values set to `v`.
    #[inline(always)]
    pub const fn splat(v: bool) -> Self {
        Self { x: v, y: v }
    }

    /// Returns a [`BVec2`] converted from array.
    #[inline]
    pub const fn from_array(a: [bool; 2]) -> Self {
        Self::new(a[0], a[1])
    }

    /// Returns array converted from [`BVec2`].
    #[inline]
    pub const fn to_array(&self) -> [bool; 2] {
        [self.x, self.y]
    }

    /// Returns a [`BVec2`] converted from slice.
    #[inline]
    pub const fn from_slice(s: &[bool]) -> Self {
        Self::new(s[0], s[1])
    }

    /// Converts [`BVec2`] `self` to slice.
    #[inline]
    pub fn to_slice(self, s: &mut [bool]) {
        s[0] = self.x;
        s[1] = self.y;
    }

    /// Returns a bitmask with the lowest 2 bits set from the elements of [`BVec2`].
    #[inline]
    pub fn bitmask(self) -> u32 {
        (self.x as u32) | (self.y as u32) << 1
    }

    /// Returns `true` if any values of `self` are true.
    #[inline]
    pub fn any(self) -> bool {
        self.x || self.y
    }

    /// Returns `true` if all values of `self` are true.
    #[inline]
    pub fn all(self) -> bool {
        self.x && self.y
    }
}

impl BitAnd for BVec2 {
    type Output = Self;

    #[inline]
    fn bitand(self, other: Self) -> Self {
        Self {
            x: self.x & other.x,
            y: self.y & other.y,
        }
    }
}

impl BitAnd<bool> for BVec2 {
    type Output = Self;

    #[inline]
    fn bitand(self, other: bool) -> Self {
        Self {
            x: self.x & other,
            y: self.y & other,
        }
    }
}

impl BitAndAssign for BVec2 {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x & other.x,
            y: self.y & other.y,
        }
    }
}

impl BitAndAssign<bool> for BVec2 {
    #[inline]
    fn bitand_assign(&mut self, other: bool) {
        *self = Self {
            x: self.x & other,
            y: self.y & other,
        }
    }
}

impl BitOr for BVec2 {
    type Output = Self;

    #[inline]
    fn bitor(self, other: Self) -> Self {
        Self {
            x: self.x | other.x,
            y: self.y | other.y,
        }
    }
}

impl BitOr<bool> for BVec2 {
    type Output = Self;

    #[inline]
    fn bitor(self, other: bool) -> Self {
        Self {
            x: self.x | other,
            y: self.y | other,
        }
    }
}

impl BitOrAssign for BVec2 {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x | other.x,
            y: self.y | other.y,
        }
    }
}

impl BitOrAssign<bool> for BVec2 {
    #[inline]
    fn bitor_assign(&mut self, other: bool) {
        *self = Self {
            x: self.x | other,
            y: self.y | other,
        }
    }
}

impl BitXor for BVec2 {
    type Output = Self;

    #[inline]
    fn bitxor(self, other: Self) -> Self {
        Self {
            x: self.x ^ other.x,
            y: self.y ^ other.y,
        }
    }
}

impl BitXor<bool> for BVec2 {
    type Output = Self;

    #[inline]
    fn bitxor(self, other: bool) -> Self {
        Self {
            x: self.x ^ other,
            y: self.y ^ other,
        }
    }
}

impl BitXorAssign for BVec2 {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x ^ other.x,
            y: self.y ^ other.y,
        }
    }
}

impl BitXorAssign<bool> for BVec2 {
    #[inline]
    fn bitxor_assign(&mut self, other: bool) {
        *self = Self {
            x: self.x ^ other,
            y: self.y ^ other,
        }
    }
}

impl Not for BVec2 {
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
        }
    }
}

impl fmt::Debug for BVec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Display for BVec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(BVec2))
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

impl From<BVec3> for BVec2 {
    #[inline]
    fn from(v: BVec3) -> Self {
        Self::new(v.x, v.y)
    }
}

impl From<BVec4> for BVec2 {
    #[inline]
    fn from(v: BVec4) -> Self {
        Self::new(v.x, v.y)
    }
}

impl From<[bool; 2]> for BVec2 {
    #[inline]
    fn from(a: [bool; 2]) -> Self {
        Self::new(a[0], a[1])
    }
}

impl From<BVec2> for [bool; 2] {
    #[inline]
    fn from(v: BVec2) -> Self {
        [v.x, v.y]
    }
}

impl From<(bool, bool)> for BVec2 {
    #[inline]
    fn from(t: (bool, bool)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl From<BVec2> for (bool, bool) {
    #[inline]
    fn from(v: BVec2) -> Self {
        (v.x, v.y)
    }
}

impl AsRef<[bool; 2]> for BVec2 {
    #[inline]
    fn as_ref(&self) -> &[bool; 2] {
        unsafe { &*(self as *const BVec2 as *const [bool; 2]) }
    }
}

impl AsMut<[bool; 2]> for BVec2 {
    #[inline]
    fn as_mut(&mut self) -> &mut [bool; 2] {
        unsafe { &mut *(self as *mut BVec2 as *mut [bool; 2]) }
    }
}
