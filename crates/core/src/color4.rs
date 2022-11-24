use std::fmt;

/// A color representation.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color4 {
    /// Red channel of color.
    pub r: u8,

    /// Green channel of color.
    pub g: u8,

    /// Blue channel of color.
    pub b: u8,

    /// Alpha channel of color.
    pub a: u8,
}

impl Color4 {
    /// Only red channel of [`Color4`].
    pub const RED: Self = Self::new(255, 0, 0, 0);

    /// Only green channel of [`Color4`].
    pub const GREEN: Self = Self::new(0, 255, 0, 0);

    /// Only blue channel of [`Color4`].
    pub const BLUE: Self = Self::new(0, 0, 255, 0);

    /// Black color of [`Color4`].
    pub const BLACK: Self = Self::splat(0);

    /// White color of [`Color4`].
    pub const WHITE: Self = Self::splat(255);

    /// Returns a [`Color4`] with given `u8` values.
    #[inline(always)]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Returns a [`Color4`] with all values set to `c`, except `a` which 255.
    #[inline(always)]
    pub const fn splat(c: u8) -> Self {
        Self {
            r: c,
            g: c,
            b: c,
            a: 255,
        }
    }

    /// Returns a [`Color4`] converted from `hex string`, like `#cc241daa`.
    #[inline]
    pub const fn from_hex(hex: &str) -> Option<Self> {
        todo!()
    }

    /// Returns a [`Color4`] converted from `f32`.
    #[inline]
    pub const fn from_f32(r: f32, g: f32, b: f32, a: f32) -> Self {
        todo!()
    }

    /// Returns a [`Color4`] converted from array.
    #[inline]
    pub const fn from_array(a: [u8; 4]) -> Self {
        Self::new(a[0], a[1], a[2], a[3])
    }

    /// Returns array converted from [`Color4`].
    #[inline]
    pub const fn to_array(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }

    /// Returns a [`Color4`] converted from slice.
    #[inline]
    pub const fn from_slice(s: &[u8]) -> Self {
        Self::new(s[0], s[1], s[2], s[3])
    }

    /// Converts [`Color4`] `self` to slice.
    #[inline]
    pub fn to_slice(self, s: &mut [u8]) {
        s[0] = self.r;
        s[1] = self.g;
        s[2] = self.b;
        s[3] = self.a;
    }
}

impl fmt::Display for Color4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

impl fmt::Debug for Color4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Vec4))
            .field(&self.r)
            .field(&self.g)
            .field(&self.b)
            .field(&self.a)
            .finish()
    }
}

impl From<[u8; 4]> for Color4 {
    #[inline]
    fn from(a: [u8; 4]) -> Self {
        Self::new(a[0], a[1], a[2], a[3])
    }
}

impl From<Color4> for [u8; 4] {
    #[inline]
    fn from(v: Color4) -> Self {
        [v.r, v.g, v.b, v.a]
    }
}

impl From<(u8, u8, u8, u8)> for Color4 {
    #[inline]
    fn from(t: (u8, u8, u8, u8)) -> Self {
        Self::new(t.0, t.1, t.2, t.3)
    }
}

impl From<Color4> for (u8, u8, u8, u8) {
    #[inline]
    fn from(v: Color4) -> Self {
        (v.r, v.g, v.b, v.a)
    }
}

impl AsRef<[u8; 4]> for Color4 {
    #[inline]
    fn as_ref(&self) -> &[u8; 4] {
        unsafe { &*(self as *const Color4 as *const [u8; 4]) }
    }
}

impl AsMut<[u8; 4]> for Color4 {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8; 4] {
        unsafe { &mut *(self as *mut Color4 as *mut [u8; 4]) }
    }
}
