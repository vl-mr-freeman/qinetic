use crate::bvec4::BVec4;
use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

/// A 3-dimensional boolean vector.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct BVec3 {
    pub x: bool,
    pub y: bool,
    pub z: bool,
}

impl BVec3 {
    /// All values of [`BVec3`] are false.
    pub const FALSE: Self = Self::splat(false);

    /// All values of [`BVec3`] are true.
    pub const TRUE: Self = Self::splat(true);

    /// Returns a [`BVec3`] with given values.
    #[inline(always)]
    pub const fn new(x: bool, y: bool, z: bool) -> Self {
        Self { x, y, z }
    }

    /// Returns a [`BVec3`] with all values set to `v`.
    #[inline(always)]
    pub const fn splat(v: bool) -> Self {
        Self { x: v, y: v, z: v }
    }

    /// Returns a [`BVec3`] converted from array.
    #[inline]
    pub const fn from_array(a: [bool; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }

    /// Returns array converted from [`BVec3`].
    #[inline]
    pub const fn to_array(&self) -> [bool; 3] {
        [self.x, self.y, self.z]
    }

    /// Returns a [`BVec3`] converted from slice.
    #[inline]
    pub const fn from_slice(s: &[bool]) -> Self {
        Self::new(s[0], s[1], s[2])
    }

    /// Converts [`BVec3`] `self` to slice.
    #[inline]
    pub fn to_slice(self, s: &mut [bool]) {
        s[0] = self.x;
        s[1] = self.y;
        s[2] = self.z;
    }

    /// Returns a bitmask with the lowest 3 bits set from the elements of [`BVec3`].
    #[inline]
    pub const fn bitmask(self) -> u32 {
        (self.x as u32) | (self.y as u32) << 1 | (self.z as u32) << 2
    }

    /// Returns `true` if any values of `self` are true.
    #[inline]
    pub const fn any(self) -> bool {
        self.x || self.y || self.z
    }

    /// Returns `true` if all values of `self` are true.
    #[inline]
    pub const fn all(self) -> bool {
        self.x && self.y && self.z
    }
}

impl BitAnd for BVec3 {
    type Output = Self;

    #[inline]
    fn bitand(self, other: Self) -> Self {
        Self {
            x: self.x & other.x,
            y: self.y & other.y,
            z: self.z & other.z,
        }
    }
}

impl BitAnd<bool> for BVec3 {
    type Output = Self;

    #[inline]
    fn bitand(self, other: bool) -> Self {
        Self {
            x: self.x & other,
            y: self.y & other,
            z: self.z & other,
        }
    }
}

impl BitAndAssign for BVec3 {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x & other.x,
            y: self.y & other.y,
            z: self.z & other.z,
        }
    }
}

impl BitAndAssign<bool> for BVec3 {
    #[inline]
    fn bitand_assign(&mut self, other: bool) {
        *self = Self {
            x: self.x & other,
            y: self.y & other,
            z: self.z & other,
        }
    }
}

impl BitOr for BVec3 {
    type Output = Self;

    #[inline]
    fn bitor(self, other: Self) -> Self {
        Self {
            x: self.x | other.x,
            y: self.y | other.y,
            z: self.z | other.z,
        }
    }
}

impl BitOr<bool> for BVec3 {
    type Output = Self;

    #[inline]
    fn bitor(self, other: bool) -> Self {
        Self {
            x: self.x | other,
            y: self.y | other,
            z: self.z | other,
        }
    }
}

impl BitOrAssign for BVec3 {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x | other.x,
            y: self.y | other.y,
            z: self.z | other.z,
        }
    }
}

impl BitOrAssign<bool> for BVec3 {
    #[inline]
    fn bitor_assign(&mut self, other: bool) {
        *self = Self {
            x: self.x | other,
            y: self.y | other,
            z: self.z | other,
        }
    }
}

impl BitXor for BVec3 {
    type Output = Self;

    #[inline]
    fn bitxor(self, other: Self) -> Self {
        Self {
            x: self.x ^ other.x,
            y: self.y ^ other.y,
            z: self.z ^ other.z,
        }
    }
}

impl BitXor<bool> for BVec3 {
    type Output = Self;

    #[inline]
    fn bitxor(self, other: bool) -> Self {
        Self {
            x: self.x ^ other,
            y: self.y ^ other,
            z: self.z ^ other,
        }
    }
}

impl BitXorAssign for BVec3 {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x ^ other.x,
            y: self.y ^ other.y,
            z: self.z ^ other.z,
        }
    }
}

impl BitXorAssign<bool> for BVec3 {
    #[inline]
    fn bitxor_assign(&mut self, other: bool) {
        *self = Self {
            x: self.x ^ other,
            y: self.y ^ other,
            z: self.z ^ other,
        }
    }
}

impl Not for BVec3 {
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
            z: !self.z,
        }
    }
}

impl fmt::Debug for BVec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl fmt::Display for BVec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(BVec3))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

impl From<BVec4> for BVec3 {
    #[inline]
    fn from(v: BVec4) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl From<[bool; 3]> for BVec3 {
    #[inline]
    fn from(a: [bool; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }
}

impl From<BVec3> for [bool; 3] {
    #[inline]
    fn from(v: BVec3) -> Self {
        [v.x, v.y, v.z]
    }
}

impl From<(bool, bool, bool)> for BVec3 {
    #[inline]
    fn from(t: (bool, bool, bool)) -> Self {
        Self::new(t.0, t.1, t.2)
    }
}

impl From<BVec3> for (bool, bool, bool) {
    #[inline]
    fn from(v: BVec3) -> Self {
        (v.x, v.y, v.z)
    }
}

impl AsRef<[bool; 3]> for BVec3 {
    #[inline]
    fn as_ref(&self) -> &[bool; 3] {
        unsafe { &*(self as *const BVec3 as *const [bool; 3]) }
    }
}

impl AsMut<[bool; 3]> for BVec3 {
    #[inline]
    fn as_mut(&mut self) -> &mut [bool; 3] {
        unsafe { &mut *(self as *mut BVec3 as *mut [bool; 3]) }
    }
}
