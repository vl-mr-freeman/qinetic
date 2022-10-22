#[doc(hidden)]
pub use crate::vec::*;

use crate::bvec4::BVec4;

/// A 4-dimensional vector.
#[derive(Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec for Vec4 {
    #[inline]
    fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
            w: self.w.abs(),
        }
    }

    #[inline]
    fn signum(self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
            z: self.z.signum(),
            w: self.w.signum(),
        }
    }

    #[inline]
    fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
            w: self.w.min(other.w),
        }
    }

    #[inline]
    fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
            w: self.w.max(other.w),
        }
    }

    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self {
        Self {
            x: self.x.max(max.x).min(min.x),
            y: self.y.max(max.y).min(min.y),
            z: self.z.max(max.z).min(min.z),
            w: self.w.max(max.w).min(min.w),
        }
    }

    #[inline]
    fn lerp(self, other: Self, s: f32) -> Self {
        self + ((other - self) * s)
    }

    #[inline]
    fn dot(self, other: Self) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w * other.w)
    }

    #[inline]
    fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }

    #[inline]
    fn distance_squared(self, other: Self) -> f32 {
        (self - other).length_squared()
    }

    #[inline]
    fn round(self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
            z: self.z.round(),
            w: self.w.round(),
        }
    }

    #[inline]
    fn floor(self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
            w: self.w.floor(),
        }
    }

    #[inline]
    fn ceil(self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil(),
            w: self.w.ceil(),
        }
    }

    #[inline]
    fn fract(self) -> Self {
        self - self.floor()
    }

    #[inline]
    fn exp(self) -> Self {
        Self {
            x: self.x.exp(),
            y: self.y.exp(),
            z: self.z.exp(),
            w: self.w.exp(),
        }
    }

    #[inline]
    fn powf(self, n: f32) -> Self {
        Self {
            x: self.x.powf(n),
            y: self.y.powf(n),
            z: self.z.powf(n),
            w: self.w.powf(n),
        }
    }

    #[inline]
    fn recip(self) -> Self {
        Self {
            x: self.x.recip(),
            y: self.y.recip(),
            z: self.z.recip(),
            w: self.w.recip(),
        }
    }

    #[inline]
    fn normalize(self) -> Self {
        let n = self.mul(self.length_recip());
        assert!(n.is_finite());
        n
    }

    #[inline]
    fn is_normalized(self) -> bool {
        (self.length_squared() - 1.0).abs() <= 1e-4
    }

    #[inline]
    fn is_negative_bitmask(self) -> u32 {
        (self.x.is_sign_negative() as u32)
            | (self.y.is_sign_negative() as u32) << 1
            | (self.z.is_sign_negative() as u32) << 2
            | (self.w.is_sign_negative() as u32) << 2
    }

    #[inline]
    fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite() && self.w.is_finite()
    }

    #[inline]
    fn is_nan(self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan() || self.w.is_nan()
    }
}

impl Vec4 {
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0.0);

    /// All ones.
    pub const ONE: Self = Self::splat(1.0);

    /// All negative ones.
    pub const NEG_ONE: Self = Self::splat(-1.0);

    /// All NAN.
    pub const NAN: Self = Self::splat(f32::NAN);

    /// A unit-length vector pointing along the positive X axis.
    pub const X: Self = Self::new(1.0, 0.0, 0.0, 0.0);

    /// A unit-length vector pointing along the positive Y axis.
    pub const Y: Self = Self::new(0.0, 1.0, 0.0, 0.0);

    /// A unit-length vector pointing along the positive Z axis.
    pub const Z: Self = Self::new(0.0, 0.0, 1.0, 0.0);

    /// A unit-length vector pointing along the positive W axis.
    pub const W: Self = Self::new(0.0, 0.0, 0.0, 1.0);

    /// A unit-length vector pointing along the negative X axis.
    pub const NEG_X: Self = Self::new(-1.0, 0.0, 0.0, 0.0);

    /// A unit-length vector pointing along the negative Y axis.
    pub const NEG_Y: Self = Self::new(0.0, -1.0, 0.0, 0.0);

    /// A unit-length vector pointing along the negative Z axis.
    pub const NEG_Z: Self = Self::new(0.0, 0.0, -1.0, 0.0);

    /// A unit-length vector pointing along the negative W axis.
    pub const NEG_W: Self = Self::new(0.0, 0.0, 0.0, -1.0);

    /// Creates a [`Vec4`].
    #[inline(always)]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    /// Creates a [`Vec4`] with all elements set to v.
    #[inline(always)]
    pub const fn splat(v: f32) -> Self {
        Self {
            x: v,
            y: v,
            z: v,
            w: v,
        }
    }

    /// Converts a [`Vec4`] from array.
    #[inline]
    pub const fn from_array(a: [f32; 4]) -> Self {
        Self::new(a[0], a[1], a[2], a[3])
    }

    /// Converts [`Vec4`] to array.
    #[inline]
    pub const fn to_array(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }

    /// Converts a [`Vec4`] from slice.
    #[inline]
    pub const fn from_slice(s: &[f32]) -> Self {
        Self::new(s[0], s[1], s[2], s[3])
    }

    /// Converts a [`Vec4`] to slice.
    #[inline]
    pub fn to_slice(self, s: &mut [f32]) {
        s[0] = self.x;
        s[1] = self.y;
        s[2] = self.z;
        s[3] = self.w;
    }

    /// Creates a [`Vec4`] from elements `if_true` or `if_false`, by `mask`.
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
