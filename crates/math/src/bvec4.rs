use crate::bvec::*;

/// A 3-dimensional boolean vector.
#[derive(Clone, Copy, PartialEq, Default)]
pub struct BVec4 {
    pub x: bool,
    pub y: bool,
    pub z: bool,
    pub w: bool,
}

impl BVec for BVec4 {
    fn bitmask(self) -> u32 {
        (self.x as u32) | (self.y as u32) << 1 | (self.z as u32) << 2 | (self.w as u32) << 3
    }

    #[inline]
    fn any(self) -> bool {
        self.x || self.y || self.z || self.w
    }

    #[inline]
    fn all(self) -> bool {
        self.x && self.y && self.z && self.w
    }
}

impl BVec4 {
    /// All false.
    pub const FALSE: Self = Self::splat(false);

    /// All true.
    pub const TRUE: Self = Self::splat(true);

    /// Creates a [`BVec4`].
    #[inline(always)]
    pub const fn new(x: bool, y: bool, z: bool, w: bool) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    /// Creates a [`BVec4`] with all elements set to v.
    #[inline(always)]
    pub const fn splat(v: bool) -> Self {
        Self {
            x: v,
            y: v,
            z: v,
            w: v,
        }
    }

    /// Converts a [`BVec4`] from array.
    #[inline]
    pub const fn from_array(a: [bool; 4]) -> Self {
        Self::new(a[0], a[1], a[2], a[3])
    }

    /// Converts a [`BVec4`] to array.
    #[inline]
    pub const fn to_array(&self) -> [bool; 4] {
        [self.x, self.y, self.z, self.w]
    }

    /// Converts a [`BVec4`] from slice.
    #[inline]
    pub const fn from_slice(s: &[bool]) -> Self {
        Self::new(s[0], s[1], s[2], s[3])
    }

    /// Converts a [`BVec4`] to slice.
    #[inline]
    pub fn to_slice(self, s: &mut [bool]) {
        s[0] = self.x;
        s[1] = self.y;
        s[2] = self.z;
        s[3] = self.w;
    }
}

impl BitAnd for BVec4 {
    type Output = Self;

    #[inline]
    fn bitand(self, other: Self) -> Self {
        Self {
            x: self.x & other.x,
            y: self.y & other.y,
            z: self.z & other.z,
            w: self.w & other.w,
        }
    }
}

impl BitAndAssign for BVec4 {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x & other.x,
            y: self.y & other.y,
            z: self.z & other.z,
            w: self.w & other.w,
        }
    }
}

impl BitOr for BVec4 {
    type Output = Self;

    #[inline]
    fn bitor(self, other: Self) -> Self {
        Self {
            x: self.x | other.x,
            y: self.y | other.y,
            z: self.z | other.z,
            w: self.w | other.w,
        }
    }
}

impl BitOrAssign for BVec4 {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x | other.x,
            y: self.y | other.y,
            z: self.z | other.z,
            w: self.w | other.w,
        }
    }
}

impl BitXor for BVec4 {
    type Output = Self;

    #[inline]
    fn bitxor(self, other: Self) -> Self {
        Self {
            x: self.x ^ other.x,
            y: self.y ^ other.y,
            z: self.z ^ other.z,
            w: self.w ^ other.w,
        }
    }
}

impl BitXorAssign for BVec4 {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x ^ other.x,
            y: self.y ^ other.y,
            z: self.z ^ other.z,
            w: self.w ^ other.w,
        }
    }
}

impl Not for BVec4 {
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        Self {
            x: !self.x,
            y: !self.y,
            z: !self.z,
            w: !self.w,
        }
    }
}

impl fmt::Debug for BVec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl fmt::Display for BVec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(BVec4))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .field(&self.w)
            .finish()
    }
}

impl From<[bool; 4]> for BVec4 {
    #[inline]
    fn from(a: [bool; 4]) -> Self {
        Self::new(a[0], a[1], a[2], a[3])
    }
}

impl From<BVec4> for [bool; 4] {
    #[inline]
    fn from(v: BVec4) -> Self {
        [v.x, v.y, v.z, v.w]
    }
}

impl From<(bool, bool, bool, bool)> for BVec4 {
    #[inline]
    fn from(t: (bool, bool, bool, bool)) -> Self {
        Self::new(t.0, t.1, t.2, t.3)
    }
}

impl From<BVec4> for (bool, bool, bool, bool) {
    #[inline]
    fn from(v: BVec4) -> Self {
        (v.x, v.y, v.z, v.w)
    }
}

impl AsRef<[bool; 4]> for BVec4 {
    #[inline]
    fn as_ref(&self) -> &[bool; 4] {
        unsafe { &*(self as *const BVec4 as *const [bool; 4]) }
    }
}

impl AsMut<[bool; 4]> for BVec4 {
    #[inline]
    fn as_mut(&mut self) -> &mut [bool; 4] {
        unsafe { &mut *(self as *mut BVec4 as *mut [bool; 4]) }
    }
}
