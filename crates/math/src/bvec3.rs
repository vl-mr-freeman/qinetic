use crate::{bvec::*, bvec4::BVec4};

/// A 3-dimensional boolean vector.
#[derive(Clone, Copy, PartialEq, Default)]
pub struct BVec3 {
    pub x: bool,
    pub y: bool,
    pub z: bool,
}

impl BVec for BVec3 {
    fn bitmask(self) -> u32 {
        (self.x as u32) | (self.y as u32) << 1 | (self.z as u32) << 2
    }

    #[inline]
    fn any(self) -> bool {
        self.x || self.y || self.z
    }

    #[inline]
    fn all(self) -> bool {
        self.x && self.y && self.z
    }
}

impl BVec3 {
    /// All false.
    pub const FALSE: Self = Self::splat(false);

    /// All true.
    pub const TRUE: Self = Self::splat(true);

    /// Creates a [`BVec3`].
    #[inline(always)]
    pub const fn new(x: bool, y: bool, z: bool) -> Self {
        Self { x: x, y: y, z: z }
    }

    /// Creates a [`BVec3`] with all elements set to v.
    #[inline(always)]
    pub const fn splat(v: bool) -> Self {
        Self { x: v, y: v, z: v }
    }

    /// Converts a [`BVec3`] from array.
    #[inline]
    pub const fn from_array(a: [bool; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }

    /// Converts a [`BVec3`] to array.
    #[inline]
    pub const fn to_array(&self) -> [bool; 3] {
        [self.x, self.y, self.z]
    }

    /// Converts a [`BVec3`] from slice.
    #[inline]
    pub const fn from_slice(s: &[bool]) -> Self {
        Self::new(s[0], s[1], s[2])
    }

    /// Converts a [`BVec3`] to slice.
    #[inline]
    pub fn to_slice(self, s: &mut [bool]) {
        s[0] = self.x;
        s[1] = self.y;
        s[2] = self.z;
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
