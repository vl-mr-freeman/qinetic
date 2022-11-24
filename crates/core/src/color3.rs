use std::fmt;

/// A color representation.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color3 {
    /// Red channel of color.
    pub r: u8,

    /// Green channel of color.
    pub g: u8,

    /// Blue channel of color.
    pub b: u8,
}

impl Color3 {
    /// Only red channel of [`Color3`].
    pub const RED: Self = Self::new(255, 0, 0);

    /// Only green channel of [`Color3`].
    pub const GREEN: Self = Self::new(0, 255, 0);

    /// Only blue channel of [`Color3`].
    pub const BLUE: Self = Self::new(0, 0, 255);

    /// Black color of [`Color3`].
    pub const BLACK: Self = Self::splat(0);

    /// White color of [`Color3`].
    pub const WHITE: Self = Self::splat(255);

    /// Returns a [`Color3`] with given `u8` values.
    #[inline(always)]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Returns a [`Color3`] with all values set to `c`.
    #[inline(always)]
    pub const fn splat(c: u8) -> Self {
        Self { r: c, g: c, b: c }
    }

    /// Returns a [`Color3`] converted from `hex string`, like `#cc241d`.
    #[inline]
    pub fn from_hex(hex: &str) -> Option<Self> {
        Some(Self::new(0, 0, 0))
    }

    /// Returns a [`Color3`] converted from `f32`.
    #[inline]
    pub fn from_f32(r: f32, g: f32, b: f32) -> Option<Self> {
        Some(Self::new(0, 0, 0))
    }

    /// Returns a [`Color3`] converted from array.
    #[inline]
    pub const fn from_array(a: [u8; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }

    /// Returns array converted from [`Color3`].
    #[inline]
    pub const fn to_array(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }

    /// Returns a [`Color3`] converted from slice.
    #[inline]
    pub const fn from_slice(s: &[u8]) -> Self {
        Self::new(s[0], s[1], s[2])
    }

    /// Converts [`Color3`] `self` to slice.
    #[inline]
    pub fn to_slice(self, s: &mut [u8]) {
        s[0] = self.r;
        s[1] = self.g;
        s[2] = self.b;
    }
}

impl fmt::Display for Color3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.g, self.b)
    }
}

impl fmt::Debug for Color3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Vec4))
            .field(&self.r)
            .field(&self.g)
            .field(&self.b)
            .finish()
    }
}

impl From<[u8; 3]> for Color3 {
    #[inline]
    fn from(a: [u8; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }
}

impl From<Color3> for [u8; 3] {
    #[inline]
    fn from(v: Color3) -> Self {
        [v.r, v.g, v.b]
    }
}

impl From<(u8, u8, u8)> for Color3 {
    #[inline]
    fn from(t: (u8, u8, u8)) -> Self {
        Self::new(t.0, t.1, t.2)
    }
}

impl From<Color3> for (u8, u8, u8) {
    #[inline]
    fn from(v: Color3) -> Self {
        (v.r, v.g, v.b)
    }
}

impl AsRef<[u8; 3]> for Color3 {
    #[inline]
    fn as_ref(&self) -> &[u8; 3] {
        unsafe { &*(self as *const Color3 as *const [u8; 3]) }
    }
}

impl AsMut<[u8; 3]> for Color3 {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8; 3] {
        unsafe { &mut *(self as *mut Color3 as *mut [u8; 3]) }
    }
}
