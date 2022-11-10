use crate::{bvec2::BVec2, vec3::Vec3, vec4::Vec4};
use std::fmt;
use std::iter::{Product, Sum};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem, RemAssign, Sub,
    SubAssign,
};

/// A 2-dimensional vector.
#[derive(Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self::splat(0.0);

    /// All values of [`Vec2`] are positive ones.
    pub const ONE: Self = Self::splat(1.0);

    /// All values of [`Vec2`] are negative ones.
    pub const NEG_ONE: Self = Self::splat(-1.0);

    /// All values of [`Vec2`] are NaN.
    pub const NAN: Self = Self::splat(f32::NAN);

    /// A unit-length [`Vec2`] pointing along the positive X axis.
    pub const X: Self = Self::new(1.0, 0.0);

    /// A unit-length [`Vec2`] pointing along the positive Y axis.
    pub const Y: Self = Self::new(0.0, 1.0);

    /// A unit-length [`Vec2`] pointing along the negative X axis.
    pub const NEG_X: Self = Self::new(-1.0, 0.0);

    /// A unit-length [`Vec2`] pointing along the negative Y axis.
    pub const NEG_Y: Self = Self::new(0.0, -1.0);

    /// Returns a [`Vec2`] with given values.
    #[inline(always)]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x: x, y: y }
    }

    /// Returns a [`Vec2`] with all values set to `v`.
    #[inline(always)]
    pub const fn splat(v: f32) -> Self {
        Self { x: v, y: v }
    }

    /// Returns a [`Vec2`] converted from array.
    #[inline]
    pub const fn from_array(a: [f32; 2]) -> Self {
        Self::new(a[0], a[1])
    }

    /// Returns array converted from [`Vec2`].
    #[inline]
    pub const fn to_array(&self) -> [f32; 2] {
        [self.x, self.y]
    }

    /// Returns a [`Vec2`] converted from slice.
    #[inline]
    pub const fn from_slice(s: &[f32]) -> Self {
        Self::new(s[0], s[1])
    }

    /// Converts [`Vec2`] `self` to slice.
    #[inline]
    pub fn to_slice(self, s: &mut [f32]) {
        s[0] = self.x;
        s[1] = self.y;
    }

    /// Returns a [`Vec2`] from elements `if_true` or `if_false`, by `mask`.
    #[inline]
    pub fn mask(mask: BVec2, if_true: Self, if_false: Self) -> Self {
        Self {
            x: if mask.x { if_true.x } else { if_false.x },
            y: if mask.y { if_true.y } else { if_false.y },
        }
    }

    /// Returns a [`BVec2`] of a `==` comparison elements of `self` and `other`.
    #[inline]
    pub fn cmpeq(self, other: Self) -> BVec2 {
        BVec2::new(self.x.eq(&other.x), self.y.eq(&other.y))
    }

    /// Returns a [`BVec2`] of a `!=` comparison elements of `self` and `other`.
    #[inline]
    pub fn cmpne(self, other: Self) -> BVec2 {
        BVec2::new(self.x.ne(&other.x), self.y.ne(&other.y))
    }

    /// Returns a [`BVec2`] of a `>=` comparison elements of `self` and `other`.
    #[inline]
    pub fn cmpge(self, other: Self) -> BVec2 {
        BVec2::new(self.x.ge(&other.x), self.y.ge(&other.y))
    }

    /// Returns a [`BVec2`] of a `>` comparison elements of `self` and `other`.
    #[inline]
    pub fn cmpgt(self, other: Self) -> BVec2 {
        BVec2::new(self.x.gt(&other.x), self.y.gt(&other.y))
    }

    /// Returns a [`BVec2`] of a `<=` comparison elements of `self` and `other`.
    #[inline]
    pub fn cmple(self, other: Self) -> BVec2 {
        BVec2::new(self.x.le(&other.x), self.y.le(&other.y))
    }

    /// Returns a [`BVec2`] of a `<` comparison elements of `self` and `other`.
    #[inline]
    pub fn cmplt(self, other: Self) -> BVec2 {
        BVec2::new(self.x.lt(&other.x), self.y.lt(&other.y))
    }

    /// Returns a [`Vec2`] with absolute values of `self`.
    #[inline]
    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    /// Returns true if the absolute difference of all elements between `self` and `other` <= `max_abs_diff`.
    #[inline]
    pub fn abs_diff_eq(self, rhs: Self, max_abs_diff: f32) -> bool {
        self.sub(rhs).abs().cmple(Self::splat(max_abs_diff)).all()
    }

    /// Returns a [`Vec2`] with representing sign values of `self`.
    #[inline]
    pub fn signum(self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }

    /// Returns a [`Vec2`] with minimum values of `self` and `other`.
    #[inline]
    pub fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    /// Returns a [`Vec2`] with maximum values of `self` and `other`.
    #[inline]
    pub fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    /// Returns the horizontal minimum of `self`.
    #[inline]
    pub fn min_element(self) -> f32 {
        self.x.min(self.y)
    }

    /// Returns the horizontal maximum of `self`.
    #[inline]
    pub fn max_element(self) -> f32 {
        self.x.max(self.y)
    }

    /// Returns a [`Vec2`] with clamp values of `self` between `min` and `max`.
    #[inline]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self {
            x: self.x.max(max.x).min(min.x),
            y: self.y.max(max.y).min(min.y),
        }
    }

    /// Returns a [`Vec2`] with a length no less than `min` and no more than `max`.
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

    /// Returns a [`Vec2`] with a length no more than `max`
    pub fn clamp_length_max(self, max: f32) -> Self {
        let length_sq = self.length_squared();
        if length_sq > max * max {
            self * (length_sq.sqrt().recip() * max)
        } else {
            self
        }
    }

    /// Returns a [`Vec2`] with a length no less than `min`
    pub fn clamp_length_min(self, min: f32) -> Self {
        let length_sq = self.length_squared();
        if length_sq < min * min {
            self * (length_sq.sqrt().recip() * min)
        } else {
            self
        }
    }

    /// Returns a [`Vec2`] with linear interpolation between `self` and `other` based on the value `s`.
    #[inline]
    pub fn lerp(self, other: Self, s: f32) -> Self {
        self + ((other - self) * s)
    }

    /// Returns a [`Vec2`] with dot product of `self` and `other`.
    #[inline]
    pub fn dot(self, other: Self) -> f32 {
        (self.x * other.x) + (self.y * other.y)
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

    /// Returns a [`Vec2`] with Euclidean distance of `self` and `other`.
    #[inline]
    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }

    /// Returns a [`Vec2`] with squared Euclidean distance of `self` and `other`.
    #[inline]
    pub fn distance_squared(self, other: Self) -> f32 {
        (self - other).length_squared()
    }

    /// Returns a [`Vec2`] with the nearest integer to a number for values of `self`.
    #[inline]
    pub fn round(self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
        }
    }

    /// Returns a [`Vec2`] with the largest integer less than or equal to a number for values of `self`.
    #[inline]
    pub fn floor(self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }

    /// Returns a [`Vec2`] with containing the smallest integer greater than or equal to a number for values of `self`.
    #[inline]
    pub fn ceil(self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
        }
    }

    /// Returns a [`Vec2`] with fractional values of `self`.
    #[inline]
    pub fn fract(self) -> Self {
        self - self.floor()
    }

    /// Returns a [`Vec2`] with exponential function for values of `self`.
    #[inline]
    pub fn exp(self) -> Self {
        Self {
            x: self.x.exp(),
            y: self.y.exp(),
        }
    }

    /// Returns a [`Vec2`] with raised values of `self` to the power of `n`.
    #[inline]
    pub fn powf(self, n: f32) -> Self {
        Self {
            x: self.x.powf(n),
            y: self.y.powf(n),
        }
    }

    /// Returns a [`Vec2`] with reciprocaled values of `self`
    #[inline]
    pub fn recip(self) -> Self {
        Self {
            x: self.x.recip(),
            y: self.y.recip(),
        }
    }

    /// Returns a [`Vec2`] with normalized length of `self`.
    #[inline]
    pub fn normalize(self) -> Self {
        let n = self.mul(self.length_recip());
        assert!(n.is_finite());
        n
    }

    /// Returns a [`Vec2`] with projection of `self` onto `other`.
    ///
    /// `other` must be non-zero length.
    #[inline]
    pub fn project_onto(self, other: Self) -> Self {
        let other_length_squared_recip = other.length_squared_recip();
        assert!(other_length_squared_recip.is_finite());
        other * self.dot(other) * other_length_squared_recip
    }

    /// Returns a [`Vec2`] with projection of `self` onto `other`.
    ///
    /// `other` must be normalized.
    #[inline]
    pub fn project_onto_normalized(self, other: Self) -> Self {
        assert!(other.is_normalized());
        other * self.dot(other)
    }

    /// Returns a [`Vec2`] with rejection of `self` from `other`.
    ///
    /// `other` must be non-zero length.
    #[inline]
    pub fn reject_from(self, other: Self) -> Self {
        self - self.project_onto(other)
    }

    /// Returns a [`Vec2`] with rejection of `self` from `other`.
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
        (self.x.is_sign_negative() as u32) | (self.y.is_sign_negative() as u32) << 1
    }

    /// Returns `true` if all values of `self` are finite.
    #[inline]
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    /// Returns `true` if any values of `self` are `NaN`.
    #[inline]
    pub fn is_nan(self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }
}

impl Add for Vec2 {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn add(self, other: f32) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl AddAssign<f32> for Vec2 {
    #[inline]
    fn add_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x + other,
            y: self.y + other,
        };
    }
}

impl Div for Vec2 {
    type Output = Self;

    #[inline]
    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn div(self, other: f32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl DivAssign for Vec2 {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
        };
    }
}

impl DivAssign<f32> for Vec2 {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x / other,
            y: self.y / other,
        };
    }
}

impl Mul for Vec2 {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl MulAssign for Vec2 {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
        };
    }
}

impl MulAssign<f32> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
        };
    }
}

impl Rem for Vec2 {
    type Output = Self;

    #[inline]
    fn rem(self, other: Self) -> Self {
        Self {
            x: self.x % other.x,
            y: self.y % other.y,
        }
    }
}

impl Rem<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn rem(self, other: f32) -> Self {
        Self {
            x: self.x % other,
            y: self.y % other,
        }
    }
}

impl RemAssign for Vec2 {
    #[inline]
    fn rem_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x % other.x,
            y: self.y % other.y,
        };
    }
}

impl RemAssign<f32> for Vec2 {
    #[inline]
    fn rem_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x % other,
            y: self.y % other,
        };
    }
}

impl Sub for Vec2 {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn sub(self, other: f32) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
        }
    }
}

impl SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl SubAssign<f32> for Vec2 {
    #[inline]
    fn sub_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x - other,
            y: self.y - other,
        };
    }
}

impl Neg for Vec2 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Index<usize> for Vec2 {
    type Output = f32;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vec2 {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("index out of bounds"),
        }
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Debug for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Vec2))
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

impl From<Vec3> for Vec2 {
    #[inline]
    fn from(v: Vec3) -> Self {
        Self::new(v.x, v.y)
    }
}

impl From<Vec4> for Vec2 {
    #[inline]
    fn from(v: Vec4) -> Self {
        Self::new(v.x, v.y)
    }
}

impl From<[f32; 2]> for Vec2 {
    #[inline]
    fn from(a: [f32; 2]) -> Self {
        Self::new(a[0], a[1])
    }
}

impl From<Vec2> for [f32; 2] {
    #[inline]
    fn from(v: Vec2) -> Self {
        [v.x, v.y]
    }
}

impl From<(f32, f32)> for Vec2 {
    #[inline]
    fn from(t: (f32, f32)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl From<Vec2> for (f32, f32) {
    #[inline]
    fn from(v: Vec2) -> Self {
        (v.x, v.y)
    }
}

impl AsRef<[f32; 2]> for Vec2 {
    #[inline]
    fn as_ref(&self) -> &[f32; 2] {
        unsafe { &*(self as *const Vec2 as *const [f32; 2]) }
    }
}

impl AsMut<[f32; 2]> for Vec2 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 2] {
        unsafe { &mut *(self as *mut Vec2 as *mut [f32; 2]) }
    }
}

impl Sum for Vec2 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<'a> Sum<&'a Self> for Vec2 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl Product for Vec2 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ONE, Self::mul)
    }
}

impl<'a> Product<&'a Self> for Vec2 {
    #[inline]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, |a, &b| Self::mul(a, b))
    }
}
