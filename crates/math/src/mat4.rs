use crate::{mat::*, vec4::*};

/// A 4x4 column major matrix.
#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Mat4 {
    pub x_axis: Vec4,
    pub y_axis: Vec4,
    pub z_axis: Vec4,
    pub w_axis: Vec4,
}

impl Mat for Mat4 {}

impl Mat4 {
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0.0);

    /// All ones.
    pub const ONE: Self = Self::splat(1.0);

    /// All negative ones.
    pub const NEG_ONE: Self = Self::splat(-1.0);

    /// All NAN.
    pub const NAN: Self = Self::splat(f32::NAN);

    /// All diagonal elements are `1`, and all off-diagonal elements are `0`.
    pub const IDENTITY: Self = Self::from_cols(Vec4::X, Vec4::Y, Vec4::Z, Vec4::W);

    /// Creates a [`Mat4`].
    pub const fn new(
        m00: f32,
        m01: f32,
        m02: f32,
        m03: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m13: f32,
        m20: f32,
        m21: f32,
        m22: f32,
        m23: f32,
        m30: f32,
        m31: f32,
        m32: f32,
        m33: f32,
    ) -> Self {
        Self {
            x_axis: Vec4::new(m00, m01, m02, m03),
            y_axis: Vec4::new(m10, m11, m12, m13),
            z_axis: Vec4::new(m20, m21, m22, m23),
            w_axis: Vec4::new(m30, m31, m32, m33),
        }
    }

    /// Creates a [`Mat4`] from 4x[`Vec4`].
    pub const fn from_cols(x_axis: Vec4, y_axis: Vec4, z_axis: Vec4, w_axis: Vec4) -> Self {
        Self {
            x_axis: x_axis,
            y_axis: y_axis,
            z_axis: z_axis,
            w_axis: w_axis,
        }
    }

    /// Creates a [`Mat4`] with all elements set to m.
    #[inline]
    pub const fn splat(m: f32) -> Self {
        Self {
            x_axis: Vec4::splat(m),
            y_axis: Vec4::splat(m),
            z_axis: Vec4::splat(m),
            w_axis: Vec4::splat(m),
        }
    }

    /// Converts a [`Mat4`] from array.
    #[inline]
    pub const fn from_array(a: [f32; 16]) -> Self {
        Self::new(
            a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11], a[12], a[13],
            a[14], a[15],
        )
    }

    /// Converts a [`Mat4`] to array.
    #[inline]
    pub const fn to_array(&self) -> [f32; 16] {
        [
            self.x_axis.x,
            self.x_axis.y,
            self.x_axis.z,
            self.x_axis.w,
            self.y_axis.x,
            self.y_axis.y,
            self.y_axis.z,
            self.y_axis.w,
            self.z_axis.x,
            self.z_axis.y,
            self.z_axis.z,
            self.z_axis.w,
            self.w_axis.x,
            self.w_axis.y,
            self.w_axis.z,
            self.w_axis.w,
        ]
    }

    /// Converts a [`Mat4`] from slice.
    #[inline]
    pub const fn from_slice(s: &[f32]) -> Self {
        Self::new(
            s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7], s[8], s[9], s[10], s[11], s[12], s[13],
            s[14], s[15],
        )
    }

    /// Converts a [`Mat4`] to slice.
    #[inline]
    pub fn to_slice(self: s: &mut [f32]) {
        s[0] = self.x_axis.x;
        s[1] = self.x_axis.y;
        s[2] = self.x_axis.z;
        s[3] = self.x_axis.w;

        s[4] = self.y_axis.x;
        s[5] = self.y_axis.y;
        s[6] = self.y_axis.z;
        s[7] = self.y_axis.w;

        s[8] = self.z_axis.x;
        s[9] = self.z_axis.y;
        s[10] = self.z_axis.z;
        s[11] = self.z_axis.w;

        s[12] = self.w_axis.x;
        s[13] = self.w_axis.y;
        s[14] = self.w_axis.z;
        s[15] = self.w_axis.w;
    }
}

impl Add for Mat4 {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x_axis: self.x_axis + other.x_axis,
            y_axis: self.y_axis + other.y_axis,
            z_axis: self.z_axis + other.z_axis,
            w_axis: self.w_axis + other.w_axis,
        }
    }
}

impl Add<f32> for Mat4 {
    type Output = Self;

    #[inline]
    fn add(self, other: f32) -> Self {
        Self {
            x_axis: self.x_axis + other,
            y_axis: self.y_axis + other,
            z_axis: self.z_axis + other,
            w_axis: self.w_axis + other,
        }
    }
}

impl AddAssign for Mat4 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x_axis: self.x_axis + other.x_axis,
            y_axis: self.y_axis + other.y_axis,
            z_axis: self.z_axis + other.z_axis,
            w_axis: self.w_axis + other.w_axis,
        };
    }
}

impl AddAssign<f32> for Mat4 {
    #[inline]
    fn add_assign(&mut self, other: f32) {
        *self = Self {
            x_axis: self.x_axis + other,
            y_axis: self.y_axis + other,
            z_axis: self.z_axis + other,
            w_axis: self.w_axis + other,
        };
    }
}

impl Div for Mat4 {
    type Output = Self;

    #[inline]
    fn div(self, other: Self) -> Self {
        Self {
            x_axis: self.x_axis / other.x_axis,
            y_axis: self.y_axis / other.y_axis,
            z_axis: self.z_axis / other.z_axis,
            w_axis: self.w_axis / other.w_axis,
        }
    }
}

impl Div<f32> for Mat4 {
    type Output = Self;

    #[inline]
    fn div(self, other: f32) -> Self {
        Self {
            x_axis: self.x_axis / other,
            y_axis: self.y_axis / other,
            z_axis: self.z_axis / other,
            w_axis: self.w_axis / other,
        }
    }
}

impl DivAssign for Mat4 {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x_axis: self.x_axis / other.x_axis,
            y_axis: self.y_axis / other.y_axis,
            z_axis: self.z_axis / other.z_axis,
            w_axis: self.w_axis / other.w_axis,
        };
    }
}

impl DivAssign<f32> for Mat4 {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        *self = Self {
            x_axis: self.x_axis / other,
            y_axis: self.y_axis / other,
            z_axis: self.z_axis / other,
            w_axis: self.w_axis / other,
        };
    }
}

impl Mul for Mat4 {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self {
        Self {
            x_axis: self.x_axis * other.x_axis,
            y_axis: self.y_axis * other.y_axis,
            z_axis: self.z_axis * other.z_axis,
            w_axis: self.w_axis * other.w_axis,
        }
    }
}

impl Mul<f32> for Mat4 {
    type Output = Self;

    #[inline]
    fn mul(self, other: f32) -> Self {
        Self {
            x_axis: self.x_axis * other,
            y_axis: self.y_axis * other,
            z_axis: self.z_axis * other,
            w_axis: self.w_axis * other,
        }
    }
}

impl MulAssign for Mat4 {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x_axis: self.x_axis * other.x_axis,
            y_axis: self.y_axis * other.y_axis,
            z_axis: self.z_axis * other.z_axis,
            w_axis: self.w_axis * other.w_axis,
        };
    }
}

impl MulAssign<f32> for Mat4 {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        *self = Self {
            x_axis: self.x_axis * other,
            y_axis: self.y_axis * other,
            z_axis: self.z_axis * other,
            w_axis: self.w_axis * other,
        };
    }
}

impl Rem for Mat4 {
    type Output = Self;

    #[inline]
    fn rem(self, other: Self) -> Self {
        Self {
            x_axis: self.x_axis % other.x_axis,
            y_axis: self.y_axis % other.y_axis,
            z_axis: self.z_axis % other.z_axis,
            w_axis: self.w_axis % other.w_axis,
        }
    }
}

impl Rem<f32> for Mat4 {
    type Output = Self;

    #[inline]
    fn rem(self, other: f32) -> Self {
        Self {
            x_axis: self.x_axis % other,
            y_axis: self.y_axis % other,
            z_axis: self.z_axis % other,
            w_axis: self.w_axis % other,
        }
    }
}

impl RemAssign for Mat4 {
    #[inline]
    fn rem_assign(&mut self, other: Self) {
        *self = Self {
            x_axis: self.x_axis % other.x_axis,
            y_axis: self.y_axis % other.y_axis,
            z_axis: self.z_axis % other.z_axis,
            w_axis: self.w_axis % other.w_axis,
        };
    }
}

impl RemAssign<f32> for Mat4 {
    #[inline]
    fn rem_assign(&mut self, other: f32) {
        *self = Self {
            x_axis: self.x_axis % other,
            y_axis: self.y_axis % other,
            z_axis: self.z_axis % other,
            w_axis: self.w_axis % other,
        };
    }
}

impl Sub for Mat4 {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x_axis: self.x_axis - other.x_axis,
            y_axis: self.y_axis - other.y_axis,
            z_axis: self.z_axis - other.z_axis,
            w_axis: self.w_axis - other.w_axis,
        }
    }
}

impl Sub<f32> for Mat4 {
    type Output = Self;

    #[inline]
    fn sub(self, other: f32) -> Self {
        Self {
            x_axis: self.x_axis - other,
            y_axis: self.y_axis - other,
            z_axis: self.z_axis - other,
            w_axis: self.w_axis - other,
        }
    }
}

impl SubAssign for Mat4 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x_axis: self.x_axis - other.x_axis,
            y_axis: self.y_axis - other.y_axis,
            z_axis: self.z_axis - other.z_axis,
            w_axis: self.w_axis - other.w_axis,
        };
    }
}

impl SubAssign<f32> for Mat4 {
    #[inline]
    fn sub_assign(&mut self, other: f32) {
        *self = Self {
            x_axis: self.x_axis - other,
            y_axis: self.y_axis - other,
            z_axis: self.z_axis - other,
            w_axis: self.w_axis - other,
        };
    }
}

impl Neg for Mat4 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self {
            x_axis: -self.x_axis,
            y_axis: -self.y_axis,
            z_axis: -self.z_axis,
            w_axis: -self.w_axis,
        }
    }
}

impl Index<usize> for Mat4 {
    type Output = f32;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x_axis.x,
            1 => &self.x_axis.y,
            2 => &self.x_axis.z,
            3 => &self.x_axis.w,
            4 => &self.y_axis.x,
            5 => &self.y_axis.y,
            6 => &self.y_axis.z,
            7 => &self.y_axis.w,
            8 => &self.z_axis.x,
            9 => &self.z_axis.y,
            10 => &self.z_axis.z,
            11 => &self.z_axis.w,
            12 => &self.w_axis.x,
            13 => &self.w_axis.y,
            14 => &self.w_axis.z,
            15 => &self.w_axis.w,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Mat4 {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x_axis.x,
            1 => &mut self.x_axis.y,
            2 => &mut self.x_axis.z,
            3 => &mut self.x_axis.w,
            4 => &mut self.y_axis.x,
            5 => &mut self.y_axis.y,
            6 => &mut self.y_axis.z,
            7 => &mut self.y_axis.w,
            8 => &mut self.z_axis.x,
            9 => &mut self.z_axis.y,
            10 => &mut self.z_axis.z,
            11 => &mut self.z_axis.w,
            12 => &mut self.w_axis.x,
            13 => &mut self.w_axis.y,
            14 => &mut self.w_axis.z,
            15 => &mut self.w_axis.w,
            _ => panic!("index out of bounds"),
        }
    }
}

impl fmt::Display for Mat4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}, {})",
            self.x_axis, self.y_axis, self.z_axis, self.w_axis
        )
    }
}

impl fmt::Debug for Mat4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(Mat4))
            .field("x_axis", &self.x_axis)
            .field("y_axis", &self.y_axis)
            .field("z_axis", &self.z_axis)
            .field("w_axis", &self.w_axis)
            .finish()
    }
}

impl From<[f32; 16]> for Mat4 {
    #[inline]
    fn from(a: [f32; 16]) -> Self {
        Self::new(
            a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11], a[12], a[13],
            a[14], a[15],
        )
    }
}

impl From<Mat4> for [f32; 16] {
    #[inline]
    fn from(m: Mat4) -> Self {
        [
            m.x_axis.x, m.x_axis.y, m.x_axis.z, m.x_axis.w, m.y_axis.x, m.y_axis.y, m.y_axis.z,
            m.y_axis.w, m.z_axis.x, m.z_axis.y, m.z_axis.z, m.z_axis.w, m.w_axis.x, m.w_axis.y,
            m.w_axis.z, m.w_axis.w,
        ]
    }
}

impl
    From<(
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
    )> for Mat4
{
    #[inline]
    fn from(
        t: (
            f32,
            f32,
            f32,
            f32,
            f32,
            f32,
            f32,
            f32,
            f32,
            f32,
            f32,
            f32,
            f32,
            f32,
            f32,
            f32,
        ),
    ) -> Self {
        Self::new(
            t.0, t.1, t.2, t.3, t.4, t.5, t.6, t.7, t.8, t.9, t.10, t.11, t.12, t.13, t.14, t.15,
        )
    }
}

impl From<Mat4>
    for (
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
    )
{
    #[inline]
    fn from(m: Mat4) -> Self {
        (
            m.x_axis.x, m.x_axis.y, m.x_axis.z, m.x_axis.w, m.y_axis.x, m.y_axis.y, m.y_axis.z,
            m.y_axis.w, m.z_axis.x, m.z_axis.y, m.z_axis.z, m.z_axis.w, m.w_axis.x, m.w_axis.y,
            m.w_axis.z, m.w_axis.w,
        )
    }
}

impl AsRef<[f32; 16]> for Mat4 {
    #[inline]
    fn as_ref(&self) -> &[f32; 16] {
        unsafe { &*(self as *const Mat4 as *const [f32; 16]) }
    }
}

impl AsMut<[f32; 16]> for Mat4 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 16] {
        unsafe { &mut *(self as *mut Mat4 as *mut [f32; 16]) }
    }
}

impl Sum for Mat4 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<'a> Sum<&'a Self> for Mat4 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl Product for Mat4 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ONE, Self::mul)
    }
}

impl<'a> Product<&'a Self> for Mat4 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, |a, &b| Self::mul(a, b))
    }
}
