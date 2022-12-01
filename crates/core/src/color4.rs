//! Color(rgba) functionality.

use crate::color3::Color3;
use std::fmt;

/// A color representation with r, g, b, a channels.
///
/// # Exampless
/// ```
/// # use qinetic_core::prelude::*;
/// #
/// let yellow = Color4::new(255, 255, 0, 255);
/// ```
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
    /// Only [red](Color4::r) channel of [`Color4`].
    pub const RED: Self = Self::new(255, 0, 0, 0);

    /// Only [green](Color4::g) channel of [`Color4`].
    pub const GREEN: Self = Self::new(0, 255, 0, 0);

    /// Only [blue](Color4::b) channel of [`Color4`].
    pub const BLUE: Self = Self::new(0, 0, 255, 0);

    /// Black color of [`Color4`].
    /// All channels of [`Color4`] is `0`, except [alpha](Color4::a).
    pub const BLACK: Self = Self::splat(0);

    /// White color of [`Color4`].
    /// All channels of [`Color4`] is `255`.
    pub const WHITE: Self = Self::splat(255);

    /// Returns a [`Color4`] with given `u8` values.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_core::prelude::*;
    /// #
    /// let yellow = Color4::new(255, 255, 0, 255);
    /// assert_eq!(Color4 { r: 255, g: 255, b: 0, a: 255}, yellow);
    /// ```
    #[inline(always)]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Returns a [`Color4`] with all values set to `c`, except `a` which 255.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_core::prelude::*;
    /// #
    /// let black = Color4::splat(0);
    /// assert_eq!(Color4 { r: 0, g: 0, b: 0, a: 255}, black);
    /// ```
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
    ///
    /// # Examples
    /// ```
    /// # use qinetic_core::prelude::*;
    /// #
    ///     let red = Color4::from_hex("#FF0000FF").unwrap();
    ///     assert_eq!(Color4 { r: 255, g: 0, b: 0, a: 255 }, red);
    ///
    ///     let green = Color4::from_hex("#00FF00FF").unwrap();
    ///     assert_eq!(Color4 { r: 0, g: 255, b: 0, a: 255 }, green);
    ///
    ///     let blue = Color4::from_hex("#0000FFFF").unwrap();
    ///     assert_eq!(Color4 { r: 0, g: 0, b: 255, a: 255 }, blue);
    ///
    ///     let black = Color4::from_hex("#000000FF").unwrap();
    ///     assert_eq!(Color4 { r: 0, g: 0, b: 0, a: 255 }, black);
    ///
    ///     let yellow = Color4::from_hex("#FFFF00FF").unwrap();
    ///     assert_eq!(Color4 { r: 255, g: 255, b: 0, a: 255 }, yellow);
    /// ```
    #[inline]
    pub fn from_hex(hex: &str) -> Option<Self> {
        let mut bytes = hex.bytes();
        match bytes.next() {
            Some(b'#') => { /* just skip*/ }
            Some(_) => return None,
            None => return None,
        }
        let r = crate::color::parse_double_hex_value(&mut bytes);
        let r = match r {
            Some(r) => r,
            None => return None,
        };

        let g = crate::color::parse_double_hex_value(&mut bytes);
        let g = match g {
            Some(g) => g,
            None => return None,
        };

        let b = crate::color::parse_double_hex_value(&mut bytes);
        let b = match b {
            Some(b) => b,
            None => return None,
        };

        let a = crate::color::parse_double_hex_value(&mut bytes);
        let a = match a {
            Some(a) => a,
            None => return None,
        };

        Some(Self::new(r, g, b, a))
    }

    /// Returns a [`Color4`] converted from `f32`.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_core::prelude::*;
    /// #
    /// let red = Color4::from_f32(1.0, 0.0, 0.0, 1.0).unwrap();
    /// assert_eq!(Color4 { r: 255, g: 0, b: 0, a: 255 }, red);
    ///```
    ///
    /// ```
    /// # use qinetic_core::prelude::*;
    /// #
    /// let green = Color4::from_f32(0.0, 1.0, 0.0, 1.0).unwrap();
    /// assert_eq!(Color4 { r: 0, g: 255, b: 0, a: 255 }, green);
    /// ```
    ///
    /// ```
    /// # use qinetic_core::prelude::*;
    /// #
    /// let blue = Color4::from_f32(0.0, 0.0, 1.0, 1.0).unwrap();
    /// assert_eq!(Color4 { r: 0, g: 0, b: 255, a: 255 }, blue);
    /// ```
    ///
    /// ```
    /// # use qinetic_core::prelude::*;
    /// #
    /// let black = Color4::from_f32(0.0, 0.0, 0.0, 1.0).unwrap();
    /// assert_eq!(Color4 { r: 0, g: 0, b: 0, a: 255 }, black);
    /// ```
    ///
    /// ```
    /// # use qinetic_core::prelude::*;
    /// #
    /// let yellow = Color4::from_f32(1.0, 1.0, 0.0, 1.0).unwrap();
    /// assert_eq!(Color4 { r: 255, g: 255, b: 0, a: 255 }, yellow);
    /// ```
    #[inline]
    pub fn from_f32(r: f32, g: f32, b: f32, a: f32) -> Option<Self> {
        if [r, g, b, a].iter().all(|c| *c >= 0.0 && *c <= 1.0) {
            return None;
        }
        Some(Self {
            r: (r * 255.0) as u8,
            g: (g * 255.0) as u8,
            b: (b * 255.0) as u8,
            a: (a * 255.0) as u8,
        })
    }
}

impl fmt::Display for Color4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#({}, {}, {}, {})", self.r, self.g, self.b, self.a)
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

impl From<Color3> for Color4 {
    #[inline]
    fn from(c: Color3) -> Self {
        Self::new(c.r, c.g, c.b, 255)
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
