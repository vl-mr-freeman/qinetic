use crate::{bvec4::BVec4, quat::Quat};
use std::fmt;
use std::iter::{Product, Sum};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem, RemAssign, Sub,
    SubAssign,
};

/// A 4-dimensional vector.
#[derive(Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    /// All values of [`Vec4`] are zeroes.
    pub const ZERO: Self = Self::splat(0.0);

    /// All values of [`Vec4`] are positive ones.
    pub const ONE: Self = Self::splat(1.0);

    /// All values of [`Vec4`] are negative ones.
    pub const NEG_ONE: Self = Self::splat(-1.0);

    /// All values of [`Vec4`] are NaN.
    pub const NAN: Self = Self::splat(f32::NAN);

    /// A unit-length [`Vec4`] along the positive X axis.
    pub const X: Self = Self::new(1.0, 0.0, 0.0, 0.0);

    /// A unit-length [`Vec4`] along the positive Y axis.
    pub const Y: Self = Self::new(0.0, 1.0, 0.0, 0.0);

    /// A unit-length [`Vec4`] along the positive Z axis.
    pub const Z: Self = Self::new(0.0, 0.0, 1.0, 0.0);

    /// A unit-length [`Vec4`] along the positive W axis.
    pub const W: Self = Self::new(0.0, 0.0, 0.0, 1.0);

    /// A unit-length [`Vec4`] along the negative X axis.
    pub const NEG_X: Self = Self::new(-1.0, 0.0, 0.0, 0.0);

    /// A unit-length [`Vec4`] along the negative Y axis.
    pub const NEG_Y: Self = Self::new(0.0, -1.0, 0.0, 0.0);

    /// A unit-length [`Vec4`] along the negative Z axis.
    pub const NEG_Z: Self = Self::new(0.0, 0.0, -1.0, 0.0);

    /// A unit-length [`Vec4`] along the negative W axis.
    pub const NEG_W: Self = Self::new(0.0, 0.0, 0.0, -1.0);

    /// Returns a [`Vec4`] with all values set to `v`.
    #[inline(always)]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    /// Returns a [`Vec4`] with all values set to `v`.
    #[inline(always)]
    pub const fn splat(v: f32) -> Self {
        Self {
            x: v,
            y: v,
            z: v,
            w: v,
        }
    }

    /// Returns a [`Vec4`] converted from array.
    #[inline]
    pub const fn from_array(a: [f32; 4]) -> Self {
        Self::new(a[0], a[1], a[2], a[3])
    }

    /// Returns array converted from [`Vec4`].
    #[inline]
    pub const fn to_array(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }

    /// Returns a [`Vec4`] converted from slice.
    #[inline]
    pub const fn from_slice(s: &[f32]) -> Self {
        Self::new(s[0], s[1], s[2], s[3])
    }

    /// Converts [`Vec4`] `self` to slice.
    #[inline]
    pub fn to_slice(self, s: &mut [f32]) {
        s[0] = self.x;
        s[1] = self.y;
        s[2] = self.z;
        s[3] = self.w;
    }

    /// Returns a [`Vec4`] from elements `if_true` or `if_false`, by `mask`.
    #[inline]
    pub fn mask(mask: BVec4, if_true: Self, if_false: Self) -> Self {
        Self {
            x: if mask.x { if_true.x } else { if_false.x },
            y: if mask.y { if_true.y } else { if_false.y },
            z: if mask.z { if_true.z } else { if_false.z },
            w: if mask.w { if_true.w } else { if_false.w },
        }
    }

    /// Returns a [`BVec4`] of a `==` comparison elements of `self` and `other`.
    #[inline]
    pub fn cmpeq(self, other: Self) -> BVec4 {
        BVec4::new(
            self.x.eq(&other.x),
            self.y.eq(&other.y),
            self.z.eq(&other.z),
            self.w.eq(&other.w),
        )
    }

    /// Returns a [`BVec4`] of a `!=` comparison elements of `self` and `other`.
    #[inline]
    pub fn cmpne(self, other: Self) -> BVec4 {
        BVec4::new(
            self.x.ne(&other.x),
            self.y.ne(&other.y),
            self.z.ne(&other.z),
            self.w.ne(&other.w),
        )
    }

    /// Returns a [`BVec4`] of a `>=` comparison elements of `self` and `other`.
    #[inline]
    pub fn cmpge(self, other: Self) -> BVec4 {
        BVec4::new(
            self.x.ge(&other.x),
            self.y.ge(&other.y),
            self.z.ge(&other.z),
            self.w.ge(&other.w),
        )
    }

    /// Returns a [`BVec4`] of a `>` comparison elements of `self` and `other`.
    #[inline]
    pub fn cmpgt(self, other: Self) -> BVec4 {
        BVec4::new(
            self.x.gt(&other.x),
            self.y.gt(&other.y),
            self.z.gt(&other.z),
            self.w.gt(&other.w),
        )
    }

    /// Returns a [`BVec4`] of a `<=` comparison elements of `self` and `other`.
    #[inline]
    pub fn cmple(self, other: Self) -> BVec4 {
        BVec4::new(
            self.x.le(&other.x),
            self.y.le(&other.y),
            self.z.le(&other.z),
            self.w.le(&other.w),
        )
    }

    /// Returns a [`BVec4`] of a `<` comparison elements of `self` and `other`.
    #[inline]
    pub fn cmplt(self, other: Self) -> BVec4 {
        BVec4::new(
            self.x.lt(&other.x),
            self.y.lt(&other.y),
            self.z.lt(&other.z),
            self.w.lt(&other.w),
        )
    }

    /// Returns a [`Vec4`] with absolute values of `self`.
    #[inline]
    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
            w: self.w.abs(),
        }
    }

    /// Returns true if the absolute difference of all elements between `self` and `other` <= `max_abs_diff`.
    #[inline]
    pub fn abs_diff_eq(self, rhs: Self, max_abs_diff: f32) -> bool {
        self.sub(rhs).abs().cmple(Self::splat(max_abs_diff)).all()
    }

    /// Returns a [`Vec4`] with representing sign values of `self`.
    #[inline]
    pub fn signum(self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
            z: self.z.signum(),
            w: self.w.signum(),
        }
    }

    /// Returns a [`Vec4`] with minimum values of `self` and `other`.
    #[inline]
    pub fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
            w: self.w.min(other.w),
        }
    }

    /// Returns a [`Vec4`] with maximum values of `self` and `other`.
    #[inline]
    pub fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
            w: self.w.max(other.w),
        }
    }

    /// Returns the horizontal minimum of `self`.
    #[inline]
    pub fn min_element(self) -> f32 {
        self.x.min(self.y.min(self.z.min(self.w)))
    }

    /// Returns the horizontal maximum of `self`.
    #[inline]
    pub fn max_element(self) -> f32 {
        self.x.max(self.y.max(self.z.max(self.w)))
    }

    /// Returns a [`Vec4`] with clamp values of `self` between `min` and `max`.
    #[inline]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self {
            x: self.x.max(max.x).min(min.x),
            y: self.y.max(max.y).min(min.y),
            z: self.z.max(max.z).min(min.z),
            w: self.w.max(max.w).min(min.w),
        }
    }

    /// Returns a [`Vec4`] with a length no less than `min` and no more than `max`.
    ///
    /// `max` must be greater that `min`.
    #[inline]
    pub fn clamp_length(self, min: f32, max: f32) -> Self {
        assert!(min <= max);
        let length_sq = self.length_squared();
        if length_sq < min * min {
            self * (length_sq.sqrt().recip() * min)
        } else if length_sq > max * max {
            self * (length_sq.sqrt().recip() * max)
        } else {
            self
        }
    }

    /// Returns a [`Vec4`] with a length no more than `max`
    pub fn clamp_length_max(self, max: f32) -> Self {
        let length_sq = self.length_squared();
        if length_sq > max * max {
            self * (length_sq.sqrt().recip() * max)
        } else {
            self
        }
    }

    /// Returns a [`Vec4`] with a length no less than `min`
    pub fn clamp_length_min(self, min: f32) -> Self {
        let length_sq = self.length_squared();
        if length_sq < min * min {
            self * (length_sq.sqrt().recip() * min)
        } else {
            self
        }
    }

    /// Returns a [`Vec4`] with linear interpolation between `self` and `other` based on the value `s`.
    #[inline]
    pub fn lerp(self, other: Self, s: f32) -> Self {
        self + ((other - self) * s)
    }

    /// Returns a [`Vec4`] with dot product of `self` and `other`.
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

    /// Returns a [`Vec4`] with Euclidean distance of `self` and `other`.
    #[inline]
    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }

    /// Returns a [`Vec4`] with squared Euclidean distance of `self` and `other`.
    #[inline]
    pub fn distance_squared(self, other: Self) -> f32 {
        (self - other).length_squared()
    }

    /// Returns a [`Vec4`] with the nearest integer to a number for values of `self`.
    #[inline]
    pub fn round(self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
            z: self.z.round(),
            w: self.w.round(),
        }
    }

    /// Returns a [`Vec4`] with the largest integer less than or equal to a number for values of `self`.
    #[inline]
    pub fn floor(self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
            w: self.w.floor(),
        }
    }

    /// Returns a [`Vec4`] with containing the smallest integer greater than or equal to a number for values of `self`.
    #[inline]
    pub fn ceil(self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil(),
            w: self.w.ceil(),
        }
    }

    /// Returns a [`Vec4`] with fractional values of `self`.
    #[inline]
    pub fn fract(self) -> Self {
        self - self.floor()
    }

    /// Returns a [`Vec4`] with exponential function for values of `self`.
    #[inline]
    pub fn exp(self) -> Self {
        Self {
            x: self.x.exp(),
            y: self.y.exp(),
            z: self.z.exp(),
            w: self.w.exp(),
        }
    }

    /// Returns a [`Vec4`] with raised values of `self` to the power of `n`.
    #[inline]
    pub fn powf(self, n: f32) -> Self {
        Self {
            x: self.x.powf(n),
            y: self.y.powf(n),
            z: self.z.powf(n),
            w: self.w.powf(n),
        }
    }

    /// Returns a [`Vec4`] with reciprocaled values of `self`
    #[inline]
    pub fn recip(self) -> Self {
        Self {
            x: self.x.recip(),
            y: self.y.recip(),
            z: self.z.recip(),
            w: self.w.recip(),
        }
    }

    /// Returns a [`Vec4`] with normalized length of `self`.
    #[inline]
    pub fn normalize(self) -> Self {
        let n = self.mul(self.length_recip());
        assert!(n.is_finite());
        n
    }

    /// Returns a [`Vec4`] with projection of `self` onto `other`.
    ///
    /// `other` must be non-zero length.
    #[inline]
    pub fn project_onto(self, other: Self) -> Self {
        let other_length_squared_recip = other.length_squared_recip();
        assert!(other_length_squared_recip.is_finite());
        other * self.dot(other) * other_length_squared_recip
    }

    /// Returns a [`Vec4`] with projection of `self` onto `other`.
    ///
    /// `other` must be normalized.
    #[inline]
    pub fn project_onto_normalized(self, other: Self) -> Self {
        assert!(other.is_normalized());
        other * self.dot(other)
    }

    /// Returns a [`Vec4`] with rejection of `self` from `other`.
    ///
    /// `other` must be non-zero length.
    #[inline]
    pub fn reject_from(self, other: Self) -> Self {
        self - self.project_onto(other)
    }

    /// Returns a [`Vec4`] with rejection of `self` from `other`.
    ///
    /// `other` must be normalized.
    #[inline]
    pub fn reject_from_normalized(self, other: Self) -> Self {
        self - self.project_onto_normalized(other)
    }

    /// Returns `true` if length of `self` is `1.0`.
    #[inline]
    pub fn is_normalized(self) -> bool {
        (self.length_squared() - 1.0).abs() <= 1e-4
    }

    /// Returns a bitmask with the lowest bits set to the sign bits from the elements of `self`.
    #[inline]
    pub fn is_negative_bitmask(self) -> u32 {
        (self.x.is_sign_negative() as u32)
            | (self.y.is_sign_negative() as u32) << 1
            | (self.z.is_sign_negative() as u32) << 2
            | (self.w.is_sign_negative() as u32) << 2
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

impl Add for Vec4 {
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

impl Add<f32> for Vec4 {
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

impl AddAssign for Vec4 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        };
    }
}

impl AddAssign<f32> for Vec4 {
    fn add_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
            w: self.w + other,
        };
    }
}

impl Div for Vec4 {
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

impl Div<f32> for Vec4 {
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

impl DivAssign for Vec4 {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w,
        };
    }
}

impl DivAssign<f32> for Vec4 {
    fn div_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        };
    }
}

impl Mul for Vec4 {
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

impl Mul<f32> for Vec4 {
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

impl MulAssign for Vec4 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        };
    }
}

impl MulAssign<f32> for Vec4 {
    fn mul_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        };
    }
}

impl Rem for Vec4 {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Self {
            x: self.x % other.x,
            y: self.y % other.y,
            z: self.z % other.z,
            w: self.w % other.w,
        }
    }
}

impl Rem<f32> for Vec4 {
    type Output = Self;

    fn rem(self, other: f32) -> Self {
        Self {
            x: self.x % other,
            y: self.y % other,
            z: self.z % other,
            w: self.w % other,
        }
    }
}

impl RemAssign for Vec4 {
    fn rem_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x % other.x,
            y: self.y % other.y,
            z: self.z % other.z,
            w: self.w % other.w,
        };
    }
}

impl RemAssign<f32> for Vec4 {
    fn rem_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x % other,
            y: self.y % other,
            z: self.z % other,
            w: self.w % other,
        };
    }
}

impl Sub for Vec4 {
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

impl Sub<f32> for Vec4 {
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

impl SubAssign for Vec4 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        };
    }
}

impl SubAssign<f32> for Vec4 {
    fn sub_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
            w: self.w - other,
        };
    }
}

impl Neg for Vec4 {
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

impl Index<usize> for Vec4 {
    type Output = f32;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vec4 {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("index out of bounds"),
        }
    }
}

impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl fmt::Debug for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Vec4))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .field(&self.w)
            .finish()
    }
}

impl From<Quat> for Vec4 {
    #[inline]
    fn from(q: Quat) -> Self {
        Self::new(q.x, q.y, q.z, q.w)
    }
}

impl From<[f32; 4]> for Vec4 {
    #[inline]
    fn from(a: [f32; 4]) -> Self {
        Self::new(a[0], a[1], a[2], a[3])
    }
}

impl From<Vec4> for [f32; 4] {
    #[inline]
    fn from(v: Vec4) -> Self {
        [v.x, v.y, v.z, v.w]
    }
}

impl From<(f32, f32, f32, f32)> for Vec4 {
    #[inline]
    fn from(t: (f32, f32, f32, f32)) -> Self {
        Self::new(t.0, t.1, t.2, t.3)
    }
}

impl From<Vec4> for (f32, f32, f32, f32) {
    #[inline]
    fn from(v: Vec4) -> Self {
        (v.x, v.y, v.z, v.w)
    }
}

impl AsRef<[f32; 4]> for Vec4 {
    #[inline]
    fn as_ref(&self) -> &[f32; 4] {
        unsafe { &*(self as *const Vec4 as *const [f32; 4]) }
    }
}

impl AsMut<[f32; 4]> for Vec4 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 4] {
        unsafe { &mut *(self as *mut Vec4 as *mut [f32; 4]) }
    }
}

impl Sum for Vec4 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<'a> Sum<&'a Self> for Vec4 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl Product for Vec4 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ONE, Self::mul)
    }
}

impl<'a> Product<&'a Self> for Vec4 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, |a, &b| Self::mul(a, b))
    }
}
