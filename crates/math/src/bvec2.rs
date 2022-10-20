use crate::bvec::*;

/// A 2-dimensional boolean vector.
#[derive(Clone, Copy, PartialEq, Default)]
pub struct BVec2 {
    pub x: bool,
    pub y: bool,
}

impl BVec for BVec2 {
    fn bitmask(self) -> u32 {
        (self.x as u32) | (self.y as u32) << 1
    }

    #[inline]
    fn any(self) -> bool {
        self.x || self.y
    }

    #[inline]
    fn all(self) -> bool {
        self.x && self.y
    }
}

impl BVec2 {
    /// All false.
    pub const FALSE: Self = Self::splat(false);

    /// All true.
    pub const TRUE: Self = Self::splat(true);

    /// Creates a [`BVec2`].
    #[inline(always)]
    pub const fn new(x: bool, y: bool) -> Self {
        Self { x: x, y: y }
    }

    /// Creates a [`BVec2`] with all elements set to v.
    #[inline(always)]
    pub const fn splat(v: bool) -> Self {
        Self { x: v, y: v }
    }

    /// Converts a [`BVec2`] from array.
    #[inline]
    pub const fn from_array(a: [bool; 2]) -> Self {
        Self::new(a[0], a[1])
    }

    /// Converts a [`BVec2`] to array.
    #[inline]
    pub const fn to_array(&self) -> [bool; 2] {
        [self.x, self.y]
    }

    /// Converts a [`BVec2`] from slice.
    #[inline]
    pub const fn from_slice(s: &[bool]) -> Self {
        Self::new(s[0], s[1])
    }

    /// Converts a [`BVec2`] to slice.
    #[inline]
    pub fn to_slice(self, s: &mut [bool]) {
        s[0] = self.x;
        s[1] = self.y;
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

impl BitAndAssign for BVec2 {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x & other.x,
            y: self.y & other.y,
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

impl BitOrAssign for BVec2 {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x | other.x,
            y: self.y | other.y,
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

impl BitXorAssign for BVec2 {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x ^ other.x,
            y: self.y ^ other.y,
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
