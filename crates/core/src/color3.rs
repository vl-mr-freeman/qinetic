//! Color(rgb) functionality.

use crate::color4::Color4;
use std::fmt;

/// A color representation with r, g, b channels.
///
/// # Examples
/// ```
/// # use qinetic_core::prelude::*;
/// #
/// let yellow = Color3::new(255, 255, 0);
/// ```
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
    /// Only [red](Color3::r) channel of [`Color3`].
    pub const RED: Self = Self::new(255, 0, 0);

    /// Only [green](Color3::g) channel of [`Color3`].
    pub const GREEN: Self = Self::new(0, 255, 0);

    /// Only [blue](Color3::b) channel of [`Color3`].
    pub const BLUE: Self = Self::new(0, 0, 255);

    /// Black color of [`Color3`].
    /// All channels of [`Color3`] is `0`.
    pub const BLACK: Self = Self::splat(0);

    /// White color of [`Color3`].
    /// All channels of [`Color3`] is `255`.
    pub const WHITE: Self = Self::splat(255);

    /// Returns a [`Color3`] with given `u8` values.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_core::prelude::*;
    /// #
    /// let yellow = Color3::new(255, 255, 0);
    /// assert_eq!(Color3 { r: 255, g: 255, b: 0}, yellow);
    /// ```
    #[inline(always)]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Returns a [`Color3`] with all values set to `c`.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_core::prelude::*;
    ///
    /// let black = Color3::splat(0);
    /// assert_eq!(Color3 { r: 0, g: 0, b: 0}, black);
    #[inline(always)]
    pub const fn splat(c: u8) -> Self {
        Self { r: c, g: c, b: c }
    }

    /// Returns a [`Color3`] converted from `hex string`, like `#cc241d`.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_core::prelude::*;
    ///
    /// let red = Color3::from_hex("#FF0000").unwrap();
    /// assert_eq!(Color3 { r: 255, g: 0, b: 0 }, red);
    /// ```
    ///
    /// ```
    /// # use qinetic_core::prelude::*;
    ///
    /// let green = Color3::from_hex("#00FF00").unwrap();
    /// assert_eq!(Color3 { r: 0, g: 255, b: 0 }, green);
    /// ```
    ///
    /// ```
    /// # use qinetic_core::prelude::*;
    ///
    /// let blue = Color3::from_hex("#0000FF").unwrap();
    /// assert_eq!(Color3 { r: 0, g: 0, b: 255 }, blue);
    /// ```
    ///
    /// ```
    /// # use qinetic_core::prelude::*;
    ///
    /// let yellow = Color3::from_hex("#FFFF00").unwrap();
    /// assert_eq!(Color3 { r: 255, g: 255, b: 0 }, yellow);
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

        Some(Self::new(r, g, b))
    }

    /// Returns a [`Color3`] converted from `f32`.
    /// # Examples
    /// ```
    /// # use qinetic_core::prelude::*;
    /// #
    /// let red = Color3::from_f32(1.0, 0.0, 0.0).unwrap();
    /// assert_eq!(Color3 { r: 255, g: 0, b: 0 }, red);
    /// ```
    ///
    /// ```
    /// # use qinetic_core::prelude::*;
    /// #
    /// let green = Color3::from_f32(0.0, 1.0, 0.0).unwrap();
    /// assert_eq!(Color3 { r: 0, g: 255, b: 0 }, green);
    /// ```
    ///
    /// ```
    /// # use qinetic_core::prelude::*;
    /// #
    /// let blue = Color3::from_f32(0.0, 0.0, 1.0).unwrap();
    /// assert_eq!(Color3 { r: 0, g: 0, b: 255 }, blue);
    /// ```
    ///
    /// ```
    /// # use qinetic_core::prelude::*;
    /// #
    /// let yellow = Color3::from_f32(1.0, 1.0, 0.0).unwrap();
    /// assert_eq!(Color3 { r: 255, g: 255, b: 0 }, yellow);
    /// ```
    #[inline]
    pub fn from_f32(r: f32, g: f32, b: f32) -> Option<Self> {
        if [r, g, b].iter().all(|c| *c < 0.0 || *c > 1.0) {
            return None;
        }
        Some(Self {
            r: (r * 255.0) as u8,
            g: (g * 255.0) as u8,
            b: (b * 255.0) as u8,
        })
    }
}

impl fmt::Display for Color3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#({}, {}, {})", self.r, self.g, self.b)
    }
}

impl fmt::Debug for Color3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Color3))
            .field(&self.r)
            .field(&self.g)
            .field(&self.b)
            .finish()
    }
}

impl From<Color4> for Color3 {
    #[inline]
    fn from(c: Color4) -> Self {
        Self::new(c.r, c.g, c.b)
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
