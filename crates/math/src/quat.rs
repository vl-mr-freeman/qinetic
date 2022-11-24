use crate::{mat3::Mat3, mat4::Mat4, vec3::Vec3, vec4::Vec4};
use std::fmt;
use std::iter::{Product, Sum};
use std::ops::{Add, Div, Mul, MulAssign, Neg, Sub};

#[derive(PartialEq, Clone, Copy)]
/// A quaternion representing an orientation.
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quat {
    /// All values of [`Quat`] are zeroes.
    pub const ZERO: Self = Self::from_array([0.0; 4]);

    /// All values of [`Quat`] are positive ones.
    pub const ONE: Self = Self::from_array([1.0; 4]);

    /// All values of [`Quat`] are negative ones.
    pub const NEG_ONE: Self = Self::from_array([-1.0; 4]);

    /// All values of [`Quat`] are NaN.
    pub const NAN: Self = Self::from_array([f32::NAN; 4]);

    /// The `z` of [`Quat`] is `1`, and others are `0`.
    pub const IDENTITY: Self = Self::from_xyzw(0.0, 0.0, 0.0, 1.0);

    /// Returns a [`Quat`] with given values.
    #[inline(always)]
    pub const fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Returns a [`Quat`] converted from array.
    #[inline]
    pub const fn from_array(a: [f32; 4]) -> Self {
        Self {
            x: a[0],
            y: a[1],
            z: a[2],
            w: a[3],
        }
    }

    /// Returns array converted from [`Quat`].
    #[inline]
    pub const fn to_array(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }

    /// Returns a [`Quat`] converted from slice.
    #[inline]
    pub const fn from_slice(s: &[f32]) -> Self {
        Self {
            x: s[0],
            y: s[1],
            z: s[2],
            w: s[3],
        }
    }

    /// Converts [`Quat`] `self` to slice.
    #[inline]
    pub fn to_slice(self, s: &mut [f32]) {
        s[0] = self.x;
        s[1] = self.y;
        s[2] = self.z;
        s[3] = self.w;
    }

    /// Returns a [`Quat`] converted from [`Vec4`].
    #[inline]
    pub const fn from_vec4(v: &Vec4) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
            w: v.w,
        }
    }

    /// Returns a [`Vec4`] converted from [`Quat`].
    pub const fn to_vec4(&self) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, self.w)
    }

    /// Returns a [`Quat`] for a normalized rotation `axis` and `angle` (in radians).
    ///
    /// `axis` must be normalized.
    #[inline]
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        assert!(axis.is_normalized());
        let (s, c) = (angle * 0.5).sin_cos();
        let v = axis * s;
        Self::from_xyzw(v.x, v.y, v.z, c)
    }

    /// Returns a [`Quat`] that rotates `v.length()` radians around `v.normalize()`.
    #[inline]
    pub fn from_scaled_axis(v: Vec3) -> Self {
        let length = v.length();
        if length == 0.0 {
            Self::IDENTITY
        } else {
            Self::from_axis_angle(v / length, length)
        }
    }

    /// Returns a [`Quat`] from the columns of a 3x3 rotation matrix.
    #[inline]
    pub(crate) fn from_rotation_axes(x_axis: Vec3, y_axis: Vec3, z_axis: Vec3) -> Self {
        let (m00, m01, m02) = x_axis.into();
        let (m10, m11, m12) = y_axis.into();
        let (m20, m21, m22) = z_axis.into();
        if m22 <= 0.0 {
            // x^2 + y^2 >= z^2 + w^2
            let dif10 = m11 - m00;
            let omm22 = 1.0 - m22;
            if dif10 <= 0.0 {
                // x^2 >= y^2
                let four_xsq = omm22 - dif10;
                let inv4x = 0.5 / four_xsq.sqrt();
                Self::from_xyzw(
                    four_xsq * inv4x,
                    (m01 + m10) * inv4x,
                    (m02 + m20) * inv4x,
                    (m12 - m21) * inv4x,
                )
            } else {
                // y^2 >= x^2
                let four_ysq = omm22 + dif10;
                let inv4y = 0.5 / four_ysq.sqrt();
                Self::from_xyzw(
                    (m01 + m10) * inv4y,
                    four_ysq * inv4y,
                    (m12 + m21) * inv4y,
                    (m20 - m02) * inv4y,
                )
            }
        } else {
            // z^2 + w^2 >= x^2 + y^2
            let sum10 = m11 + m00;
            let opm22 = 1.0 + m22;
            if sum10 <= 0.0 {
                // z^2 >= w^2
                let four_zsq = opm22 - sum10;
                let inv4z = 0.5 / four_zsq.sqrt();
                Self::from_xyzw(
                    (m02 + m20) * inv4z,
                    (m12 + m21) * inv4z,
                    four_zsq * inv4z,
                    (m01 - m10) * inv4z,
                )
            } else {
                // w^2 >= z^2
                let four_wsq = opm22 + sum10;
                let inv4w = 0.5 / four_wsq.sqrt();
                Self::from_xyzw(
                    (m12 - m21) * inv4w,
                    (m20 - m02) * inv4w,
                    (m01 - m10) * inv4w,
                    four_wsq * inv4w,
                )
            }
        }
    }

    /// Returns a [`Quat`] from the `angle` (in radians) around the x axis.
    #[inline]
    pub fn from_rotation_x(angle: f32) -> Self {
        let (s, c) = (angle * 0.5).sin_cos();
        Self::from_xyzw(s, 0.0, 0.0, c)
    }

    /// Returns a [`Quat`] from the `angle` (in radians) around the y axis.
    #[inline]
    pub fn from_rotation_y(angle: f32) -> Self {
        let (s, c) = (angle * 0.5).sin_cos();
        Self::from_xyzw(0.0, s, 0.0, c)
    }

    /// Returns a [`Quat`] from the `angle` (in radians) around the z axis.
    #[inline]
    pub fn from_rotation_z(angle: f32) -> Self {
        let (s, c) = (angle * 0.5).sin_cos();
        Self::from_xyzw(0.0, 0.0, s, c)
    }

    /// Returns a [`Quat`] from a [`Mat3`].
    #[inline]
    pub fn from_mat3(mat: &Mat3) -> Self {
        Self::from_rotation_axes(mat.x_axis, mat.y_axis, mat.z_axis)
    }

    /// Returns a [`Quat`] from a [`Mat4`].
    #[inline]
    pub fn from_mat4(mat: &Mat4) -> Self {
        Self::from_rotation_axes(mat.x_axis.into(), mat.y_axis.into(), mat.z_axis.into())
    }

    /// Returns a [`Quat`] with linear interpolation between `self` and `other` based on the value `s`.
    #[inline]
    pub fn lerp(self, other: Self, s: f32) -> Self {
        assert!(self.is_normalized());
        assert!(other.is_normalized());

        let start = self;
        let dot = start.dot(other);
        let bias = if dot >= 0.0 { 1.0 } else { -1.0 };
        let interpolated = start.add(other.mul(bias).sub(start).mul(s));
        interpolated.normalize()
    }

    /// Returns a [`Quat`] inverse of `self`
    #[inline]
    pub fn inverse(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }

    /// Returns a [`Quat`] with dot product of `self` and `other`.
    #[inline]
    pub fn dot(self, other: Self) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w * other.w)
    }

    /// Returns length of `self`.
    #[inline]
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    /// Returns squared length of `self`.
    #[inline]
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    /// Returns reciprocal of `length`  of `self`.
    #[inline]
    pub fn length_recip(self) -> f32 {
        self.length().recip()
    }

    /// Returns reciprocal of `length_squared`  of `self`.
    #[inline]
    pub fn length_squared_recip(self) -> f32 {
        self.length_squared().recip()
    }

    /// Returns a [`Quat`] with normalized length of `self`.
    #[inline]
    pub fn normalize(self) -> Self {
        let n = self.mul(self.length_recip());
        assert!(n.is_finite());
        n
    }

    /// Returns `true` if length of `self` is `1.0`.
    #[inline]
    pub fn is_normalized(self) -> bool {
        (self.length_squared() - 1.0).abs() <= 1e-4
    }

    /// Returns `true` if all values of `self` are finite.
    #[inline]
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite() && self.w.is_finite()
    }

    /// Returns `true` if any values of `self` are `NaN`.
    #[inline]
    pub fn is_nan(self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan() || self.w.is_nan()
    }
}

impl Default for Quat {
    #[inline(always)]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Add for Quat {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Add<f32> for Quat {
    type Output = Self;

    fn add(self, other: f32) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
            w: self.w + other,
        }
    }
}

impl Div for Quat {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w,
        }
    }
}

impl Div<f32> for Quat {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

impl Mul for Quat {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

impl Mul<f32> for Quat {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl MulAssign for Quat {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        };
    }
}

impl MulAssign<f32> for Quat {
    fn mul_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        };
    }
}

impl Sub for Quat {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Sub<f32> for Quat {
    type Output = Self;

    fn sub(self, other: f32) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
            w: self.w - other,
        }
    }
}

impl Neg for Quat {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl fmt::Display for Quat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl fmt::Debug for Quat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Quat))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .field(&self.w)
            .finish()
    }
}

impl From<Vec4> for Quat {
    #[inline]
    fn from(v: Vec4) -> Self {
        Self::from_xyzw(v.x, v.y, v.z, v.w)
    }
}

impl From<[f32; 4]> for Quat {
    #[inline]
    fn from(a: [f32; 4]) -> Self {
        Self::from_xyzw(a[0], a[1], a[2], a[3])
    }
}

impl From<Quat> for [f32; 4] {
    #[inline]
    fn from(v: Quat) -> Self {
        [v.x, v.y, v.z, v.w]
    }
}

impl From<(f32, f32, f32, f32)> for Quat {
    #[inline]
    fn from(t: (f32, f32, f32, f32)) -> Self {
        Self::from_xyzw(t.0, t.1, t.2, t.3)
    }
}

impl From<Quat> for (f32, f32, f32, f32) {
    #[inline]
    fn from(v: Quat) -> Self {
        (v.x, v.y, v.z, v.w)
    }
}

impl AsRef<[f32; 4]> for Quat {
    #[inline]
    fn as_ref(&self) -> &[f32; 4] {
        unsafe { &*(self as *const Quat as *const [f32; 4]) }
    }
}

impl AsMut<[f32; 4]> for Quat {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 4] {
        unsafe { &mut *(self as *mut Quat as *mut [f32; 4]) }
    }
}

impl Sum for Quat {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<'a> Sum<&'a Self> for Quat {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl Product for Quat {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ONE, Self::mul)
    }
}

impl<'a> Product<&'a Self> for Quat {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, |a, &b| Self::mul(a, b))
    }
}
