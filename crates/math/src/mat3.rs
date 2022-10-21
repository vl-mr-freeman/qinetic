use crate::{mat::*, vec3::*};

/// A 3x3 column major matrix.
#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Mat3 {
    pub x_axis: Vec3,
    pub y_axis: Vec3,
    pub z_axis: Vec3,
}

impl Mat for Mat3 {}

impl Mat3 {
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0.0);

    /// All ones.
    pub const ONE: Self = Self::splat(1.0);

    /// All negative ones.
    pub const NEG_ONE: Self = Self::splat(-1.0);

    /// All NAN.
    pub const NAN: Self = Self::splat(f32::NAN);

    /// All diagonal elements are `1`, and all off-diagonal elements are `0`.
    pub const IDENTITY: Self = Self::from_cols(Vec3::X, Vec3::Y, Vec3::Z);

    /// Creates a [`Mat3`].
    #[inline(always)]
    pub const fn new(
        m00: f32,
        m01: f32,
        m02: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m20: f32,
        m21: f32,
        m22: f32,
    ) -> Self {
        Self {
            x_axis: Vec3::new(m00, m01, m02),
            y_axis: Vec3::new(m10, m11, m12),
            z_axis: Vec3::new(m20, m21, m22),
        }
    }

    /// Creates a [`Mat3`] from 3x[`Vec3`].
    #[inline(always)]
    pub const fn from_cols(x_axis: Vec3, y_axis: Vec3, z_axis: Vec3) -> Self {
        Self {
            x_axis: x_axis,
            y_axis: y_axis,
            z_axis: z_axis,
        }
    }

    /// Creates a [`Mat3`] with all elements set to m.
    #[inline(always)]
    pub const fn splat(m: f32) -> Self {
        Self {
            x_axis: Vec3::splat(m),
            y_axis: Vec3::splat(m),
            z_axis: Vec3::splat(m),
        }
    }

    /// Converts a [`Mat3`] from array.
    #[inline]
    pub const fn from_array(a: [f32; 9]) -> Self {
        Self::new(a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8])
    }

    /// Converts a [`Mat3`] to array.
    #[inline]
    pub const fn to_array(&self) -> [f32; 9] {
        [
            self.x_axis.x,
            self.x_axis.y,
            self.x_axis.z,
            self.y_axis.x,
            self.y_axis.y,
            self.y_axis.z,
            self.z_axis.x,
            self.z_axis.y,
            self.z_axis.z,
        ]
    }

    /// Converts a [`Mat3`] from slice.
    #[inline]
    pub const fn from_slice(s: &[f32]) -> Self {
        Self::new(s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7], s[8])
    }

    /// Converts a [`Mat3`] to slice.
    #[inline]
    pub fn to_slice(self, s: &mut [f32]) {
        s[0] = self.x_axis.x;
        s[1] = self.x_axis.y;
        s[2] = self.x_axis.z;

        s[3] = self.y_axis.x;
        s[4] = self.y_axis.y;
        s[5] = self.y_axis.z;

        s[6] = self.z_axis.x;
        s[7] = self.z_axis.y;
        s[8] = self.z_axis.z;
    }
}

impl Add for Mat3 {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x_axis: self.x_axis + other.x_axis,
            y_axis: self.y_axis + other.y_axis,
            z_axis: self.z_axis + other.z_axis,
        }
    }
}

impl Add<f32> for Mat3 {
    type Output = Self;

    #[inline]
    fn add(self, other: f32) -> Self {
        Self {
            x_axis: self.x_axis + other,
            y_axis: self.y_axis + other,
            z_axis: self.z_axis + other,
        }
    }
}

impl AddAssign for Mat3 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x_axis: self.x_axis + other.x_axis,
            y_axis: self.y_axis + other.y_axis,
            z_axis: self.z_axis + other.z_axis,
        };
    }
}

impl AddAssign<f32> for Mat3 {
    #[inline]
    fn add_assign(&mut self, other: f32) {
        *self = Self {
            x_axis: self.x_axis + other,
            y_axis: self.y_axis + other,
            z_axis: self.z_axis + other,
        };
    }
}

impl Div for Mat3 {
    type Output = Self;

    #[inline]
    fn div(self, other: Self) -> Self {
        Self {
            x_axis: self.x_axis / other.x_axis,
            y_axis: self.y_axis / other.y_axis,
            z_axis: self.z_axis / other.z_axis,
        }
    }
}

impl Div<f32> for Mat3 {
    type Output = Self;

    #[inline]
    fn div(self, other: f32) -> Self {
        Self {
            x_axis: self.x_axis / other,
            y_axis: self.y_axis / other,
            z_axis: self.z_axis / other,
        }
    }
}

impl DivAssign for Mat3 {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x_axis: self.x_axis / other.x_axis,
            y_axis: self.y_axis / other.y_axis,
            z_axis: self.z_axis / other.z_axis,
        };
    }
}

impl DivAssign<f32> for Mat3 {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        *self = Self {
            x_axis: self.x_axis / other,
            y_axis: self.y_axis / other,
            z_axis: self.z_axis / other,
        };
    }
}

impl Mul for Mat3 {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self {
        Self {
            x_axis: self.x_axis * other.x_axis,
            y_axis: self.y_axis * other.y_axis,
            z_axis: self.z_axis * other.z_axis,
        }
    }
}

impl Mul<f32> for Mat3 {
    type Output = Self;

    #[inline]
    fn mul(self, other: f32) -> Self {
        Self {
            x_axis: self.x_axis * other,
            y_axis: self.y_axis * other,
            z_axis: self.z_axis * other,
        }
    }
}

impl MulAssign for Mat3 {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x_axis: self.x_axis * other.x_axis,
            y_axis: self.y_axis * other.y_axis,
            z_axis: self.z_axis * other.z_axis,
        };
    }
}

impl MulAssign<f32> for Mat3 {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        *self = Self {
            x_axis: self.x_axis * other,
            y_axis: self.y_axis * other,
            z_axis: self.z_axis * other,
        };
    }
}

impl Rem for Mat3 {
    type Output = Self;

    #[inline]
    fn rem(self, other: Self) -> Self {
        Self {
            x_axis: self.x_axis % other.x_axis,
            y_axis: self.y_axis % other.y_axis,
            z_axis: self.z_axis % other.z_axis,
        }
    }
}

impl Rem<f32> for Mat3 {
    type Output = Self;

    #[inline]
    fn rem(self, other: f32) -> Self {
        Self {
            x_axis: self.x_axis % other,
            y_axis: self.y_axis % other,
            z_axis: self.z_axis % other,
        }
    }
}

impl RemAssign for Mat3 {
    #[inline]
    fn rem_assign(&mut self, other: Self) {
        *self = Self {
            x_axis: self.x_axis % other.x_axis,
            y_axis: self.y_axis % other.y_axis,
            z_axis: self.z_axis % other.z_axis,
        };
    }
}

impl RemAssign<f32> for Mat3 {
    #[inline]
    fn rem_assign(&mut self, other: f32) {
        *self = Self {
            x_axis: self.x_axis % other,
            y_axis: self.y_axis % other,
            z_axis: self.z_axis % other,
        };
    }
}

impl Sub for Mat3 {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x_axis: self.x_axis - other.x_axis,
            y_axis: self.y_axis - other.y_axis,
            z_axis: self.z_axis - other.z_axis,
        }
    }
}

impl Sub<f32> for Mat3 {
    type Output = Self;

    #[inline]
    fn sub(self, other: f32) -> Self {
        Self {
            x_axis: self.x_axis - other,
            y_axis: self.y_axis - other,
            z_axis: self.z_axis - other,
        }
    }
}

impl SubAssign for Mat3 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x_axis: self.x_axis - other.x_axis,
            y_axis: self.y_axis - other.y_axis,
            z_axis: self.z_axis - other.z_axis,
        };
    }
}

impl SubAssign<f32> for Mat3 {
    #[inline]
    fn sub_assign(&mut self, other: f32) {
        *self = Self {
            x_axis: self.x_axis - other,
            y_axis: self.y_axis - other,
            z_axis: self.z_axis - other,
        };
    }
}

impl Neg for Mat3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self {
            x_axis: -self.x_axis,
            y_axis: -self.y_axis,
            z_axis: -self.z_axis,
        }
    }
}

impl Index<usize> for Mat3 {
    type Output = f32;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x_axis.x,
            1 => &self.x_axis.y,
            2 => &self.x_axis.z,
            3 => &self.y_axis.x,
            4 => &self.y_axis.y,
            5 => &self.y_axis.z,
            6 => &self.z_axis.x,
            7 => &self.z_axis.y,
            8 => &self.z_axis.z,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Mat3 {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x_axis.x,
            1 => &mut self.x_axis.y,
            2 => &mut self.x_axis.z,
            3 => &mut self.y_axis.x,
            4 => &mut self.y_axis.y,
            5 => &mut self.y_axis.z,
            6 => &mut self.z_axis.x,
            7 => &mut self.z_axis.y,
            8 => &mut self.z_axis.z,
            _ => panic!("index out of bounds"),
        }
    }
}

impl fmt::Display for Mat3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x_axis, self.y_axis, self.z_axis)
    }
}

impl fmt::Debug for Mat3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(Mat3))
            .field("x_axis", &self.x_axis)
            .field("y_axis", &self.y_axis)
            .field("z_axis", &self.z_axis)
            .finish()
    }
}

impl From<[f32; 9]> for Mat3 {
    #[inline]
    fn from(a: [f32; 9]) -> Self {
        Self::new(a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8])
    }
}

impl From<Mat3> for [f32; 9] {
    #[inline]
    fn from(m: Mat3) -> Self {
        [
            m.x_axis.x, m.x_axis.y, m.x_axis.z, m.y_axis.x, m.y_axis.y, m.y_axis.z, m.z_axis.x,
            m.z_axis.y, m.z_axis.z,
        ]
    }
}

impl From<(f32, f32, f32, f32, f32, f32, f32, f32, f32)> for Mat3 {
    #[inline]
    fn from(t: (f32, f32, f32, f32, f32, f32, f32, f32, f32)) -> Self {
        Self::new(t.0, t.1, t.2, t.3, t.4, t.5, t.6, t.7, t.8)
    }
}

impl From<Mat3> for (f32, f32, f32, f32, f32, f32, f32, f32, f32) {
    #[inline]
    fn from(m: Mat3) -> Self {
        (
            m.x_axis.x, m.x_axis.y, m.x_axis.z, m.y_axis.x, m.y_axis.y, m.y_axis.z, m.z_axis.x,
            m.z_axis.y, m.z_axis.z,
        )
    }
}

impl AsRef<[f32; 9]> for Mat3 {
    #[inline]
    fn as_ref(&self) -> &[f32; 9] {
        unsafe { &*(self as *const Mat3 as *const [f32; 9]) }
    }
}

impl AsMut<[f32; 9]> for Mat3 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 9] {
        unsafe { &mut *(self as *mut Mat3 as *mut [f32; 9]) }
    }
}

impl Sum for Mat3 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<'a> Sum<&'a Self> for Mat3 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl Product for Mat3 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ONE, Self::mul)
    }
}

impl<'a> Product<&'a Self> for Mat3 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, |a, &b| Self::mul(a, b))
    }
}
