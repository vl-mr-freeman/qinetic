use crate::{mat::*, mat3::Mat3, mat4::Mat4, vec2::Vec2};

/// A 2x2 column major matrix.
#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Mat2 {
    pub x_axis: Vec2,
    pub y_axis: Vec2,
}

impl Mat for Mat2 {}

impl Mat2 {
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0.0);

    /// All ones.
    pub const ONE: Self = Self::splat(1.0);

    /// All negative ones.
    pub const NEG_ONE: Self = Self::splat(-1.0);

    /// All NAN.
    pub const NAN: Self = Self::splat(f32::NAN);

    /// All diagonal elements are `1`, and all off-diagonal elements are `0`.
    pub const IDENTITY: Self = Self::from_cols(Vec2::X, Vec2::Y);

    /// Creates a [`Mat2`].
    #[inline(always)]
    pub const fn new(m00: f32, m01: f32, m10: f32, m11: f32) -> Self {
        Self {
            x_axis: Vec2::new(m00, m01),
            y_axis: Vec2::new(m10, m11),
        }
    }

    /// Creates a [`Mat2`] from 2x[`Vec2`].
    #[inline(always)]
    pub const fn from_cols(x_axis: Vec2, y_axis: Vec2) -> Self {
        Self {
            x_axis: x_axis,
            y_axis: y_axis,
        }
    }

    /// Creates a [`Mat2`] with all elements set to m.
    #[inline(always)]
    pub const fn splat(m: f32) -> Self {
        Self {
            x_axis: Vec2::splat(m),
            y_axis: Vec2::splat(m),
        }
    }

    /// Converts a [`Mat2`] from array.
    #[inline]
    pub const fn from_array(a: [f32; 4]) -> Self {
        Self::new(a[0], a[1], a[2], a[3])
    }

    /// Converts a [`Mat2`] to array.
    #[inline]
    pub const fn to_array(&self) -> [f32; 4] {
        [self.x_axis.x, self.x_axis.y, self.y_axis.x, self.y_axis.y]
    }

    /// Converts a [`Mat2`] from slice.
    #[inline]
    pub const fn from_slice(s: &[f32]) -> Self {
        Self::new(s[0], s[1], s[2], s[3])
    }

    /// Converts a [`Mat2`] to slice.
    #[inline]
    pub fn to_slice(self, s: &mut [f32]) {
        s[0] = self.x_axis.x;
        s[1] = self.x_axis.y;

        s[2] = self.y_axis.x;
        s[3] = self.y_axis.y;
    }
}

impl Add for Mat2 {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x_axis: self.x_axis + other.x_axis,
            y_axis: self.y_axis + other.y_axis,
        }
    }
}

impl Add<f32> for Mat2 {
    type Output = Self;

    #[inline]
    fn add(self, other: f32) -> Self {
        Self {
            x_axis: self.x_axis + other,
            y_axis: self.y_axis + other,
        }
    }
}

impl AddAssign for Mat2 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x_axis: self.x_axis + other.x_axis,
            y_axis: self.y_axis + other.y_axis,
        };
    }
}

impl AddAssign<f32> for Mat2 {
    #[inline]
    fn add_assign(&mut self, other: f32) {
        *self = Self {
            x_axis: self.x_axis + other,
            y_axis: self.y_axis + other,
        };
    }
}

impl Div for Mat2 {
    type Output = Self;

    #[inline]
    fn div(self, other: Self) -> Self {
        Self {
            x_axis: self.x_axis / other.x_axis,
            y_axis: self.y_axis / other.y_axis,
        }
    }
}

impl Div<f32> for Mat2 {
    type Output = Self;

    #[inline]
    fn div(self, other: f32) -> Self {
        Self {
            x_axis: self.x_axis / other,
            y_axis: self.y_axis / other,
        }
    }
}

impl DivAssign for Mat2 {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x_axis: self.x_axis / other.x_axis,
            y_axis: self.y_axis / other.y_axis,
        };
    }
}

impl DivAssign<f32> for Mat2 {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        *self = Self {
            x_axis: self.x_axis / other,
            y_axis: self.y_axis / other,
        };
    }
}

impl Mul for Mat2 {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self {
        Self {
            x_axis: self.x_axis * other.x_axis,
            y_axis: self.y_axis * other.y_axis,
        }
    }
}

impl Mul<f32> for Mat2 {
    type Output = Self;

    #[inline]
    fn mul(self, other: f32) -> Self {
        Self {
            x_axis: self.x_axis * other,
            y_axis: self.y_axis * other,
        }
    }
}

impl MulAssign for Mat2 {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x_axis: self.x_axis * other.x_axis,
            y_axis: self.y_axis * other.y_axis,
        };
    }
}

impl MulAssign<f32> for Mat2 {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        *self = Self {
            x_axis: self.x_axis * other,
            y_axis: self.y_axis * other,
        };
    }
}

impl Rem for Mat2 {
    type Output = Self;

    #[inline]
    fn rem(self, other: Self) -> Self {
        Self {
            x_axis: self.x_axis % other.x_axis,
            y_axis: self.y_axis % other.y_axis,
        }
    }
}

impl Rem<f32> for Mat2 {
    type Output = Self;

    #[inline]
    fn rem(self, other: f32) -> Self {
        Self {
            x_axis: self.x_axis % other,
            y_axis: self.y_axis % other,
        }
    }
}

impl RemAssign for Mat2 {
    #[inline]
    fn rem_assign(&mut self, other: Self) {
        *self = Self {
            x_axis: self.x_axis % other.x_axis,
            y_axis: self.y_axis % other.y_axis,
        };
    }
}

impl RemAssign<f32> for Mat2 {
    #[inline]
    fn rem_assign(&mut self, other: f32) {
        *self = Self {
            x_axis: self.x_axis % other,
            y_axis: self.y_axis % other,
        };
    }
}

impl Sub for Mat2 {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x_axis: self.x_axis - other.x_axis,
            y_axis: self.y_axis - other.y_axis,
        }
    }
}

impl Sub<f32> for Mat2 {
    type Output = Self;

    #[inline]
    fn sub(self, other: f32) -> Self {
        Self {
            x_axis: self.x_axis - other,
            y_axis: self.y_axis - other,
        }
    }
}

impl SubAssign for Mat2 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x_axis: self.x_axis - other.x_axis,
            y_axis: self.y_axis - other.y_axis,
        };
    }
}

impl SubAssign<f32> for Mat2 {
    #[inline]
    fn sub_assign(&mut self, other: f32) {
        *self = Self {
            x_axis: self.x_axis - other,
            y_axis: self.y_axis - other,
        };
    }
}

impl Neg for Mat2 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self {
            x_axis: -self.x_axis,
            y_axis: -self.y_axis,
        }
    }
}

impl Index<usize> for Mat2 {
    type Output = f32;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x_axis.x,
            1 => &self.x_axis.y,
            2 => &self.y_axis.x,
            3 => &self.y_axis.y,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Mat2 {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x_axis.x,
            1 => &mut self.x_axis.y,
            2 => &mut self.y_axis.x,
            3 => &mut self.y_axis.y,
            _ => panic!("index out of bounds"),
        }
    }
}

impl fmt::Display for Mat2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x_axis, self.y_axis)
    }
}

impl fmt::Debug for Mat2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(Mat2))
            .field("x_axis", &self.x_axis)
            .field("y_axis", &self.y_axis)
            .finish()
    }
}

impl From<Mat4> for Mat2 {
    #[inline]
    fn from(m: Mat4) -> Self {
        Self::from_cols(Vec2::from(m.x_axis), Vec2::from(m.y_axis))
    }
}

impl From<Mat3> for Mat2 {
    #[inline]
    fn from(m: Mat3) -> Self {
        Self::from_cols(Vec2::from(m.x_axis), Vec2::from(m.y_axis))
    }
}

impl From<[f32; 4]> for Mat2 {
    #[inline]
    fn from(a: [f32; 4]) -> Self {
        Self::new(a[0], a[1], a[2], a[3])
    }
}

impl From<Mat2> for [f32; 4] {
    #[inline]
    fn from(m: Mat2) -> Self {
        [m.x_axis.x, m.x_axis.y, m.y_axis.x, m.y_axis.y]
    }
}

impl From<(f32, f32, f32, f32)> for Mat2 {
    #[inline]
    fn from(t: (f32, f32, f32, f32)) -> Self {
        Self::new(t.0, t.1, t.2, t.3)
    }
}

impl From<Mat2> for (f32, f32, f32, f32) {
    #[inline]
    fn from(m: Mat2) -> Self {
        (m.x_axis.x, m.x_axis.y, m.y_axis.x, m.y_axis.y)
    }
}

impl AsRef<[f32; 4]> for Mat2 {
    #[inline]
    fn as_ref(&self) -> &[f32; 4] {
        unsafe { &*(self as *const Mat2 as *const [f32; 4]) }
    }
}

impl AsMut<[f32; 4]> for Mat2 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 4] {
        unsafe { &mut *(self as *mut Mat2 as *mut [f32; 4]) }
    }
}

impl Sum for Mat2 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<'a> Sum<&'a Self> for Mat2 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl Product for Mat2 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ONE, Self::mul)
    }
}

impl<'a> Product<&'a Self> for Mat2 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, |a, &b| Self::mul(a, b))
    }
}
