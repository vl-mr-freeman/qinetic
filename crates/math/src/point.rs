//! Point functionality.

use std::ops::*;

use num_traits::{clamp, clamp_max as clamp_min, clamp_min as clamp_max, Signed};
use qinetic_utils::prelude::*;

use crate::{
    digit::{Digit, DigitFloat, DigitNum},
    vector::{Vector2, Vector3, Vector4},
};

macro_rules! impl_point {
    ($(#[$attr:meta])* => $PointN:ident { $($field:ident),+ }, $VectorN:ident, $n:expr) => {
        $(#[$attr])*
        #[derive(SmartDefault, Clone, Copy, Debug, PartialEq, PartialOrd, Neg)]
        pub struct $PointN<T: Digit> {
            $(pub $field: T),+
        }

        impl<T: Digit> $PointN<T> {
            /// Returns a `Point` with given values.
            #[inline(always)]
            pub const fn new($($field: T),+) -> Self {
                Self { $($field),+ }
            }

            /// Returns a `Point` with all elements set to `p`.
            #[inline(always)]
            pub const fn splat(p: T) -> Self {
                Self { $($field: p),+ }
            }

            /// Returns a `Point` from elements `if_true` or `if_false`, by `mask`.
            #[inline(always)]
            pub const fn mask(mask: $PointN<bool>, if_true: Self, if_false: Self) -> Self {
                Self { $($field: if mask.$field { if_true.$field } else { if_false. $field }),+ }
            }
        }

        impl<T: DigitNum> $PointN<T> {
            /// Returns a `Point` with all elements set to `0`.
            #[inline]
            pub fn origin() -> Self {
                Self { $($field: T::zero() ),+ }
            }

            /// Returns a `Point` with dot product of `self` and `rhs`.k
            #[inline]
            pub fn dot(self, other: Self) -> T {
                Self { $($field: self.$field * other.$field ),+ }.sum()
            }

            #[inline]
            #[allow(dead_code)]
            pub(crate) fn from_vector(v: $VectorN<T>) -> $PointN<T> {
                $PointN::new($(v.$field),+)
            }

            #[inline]
            pub(crate) fn to_vector(self) -> $VectorN<T> {
                $VectorN::new($(self.$field),+)
            }

            #[inline]
            fn sum(self) -> T {
                crate::fold_array!(add, $(self.$field),+ )
            }
        }

        impl<T: DigitNum + Signed> $PointN<T> {
            /// Returns a `Point` with all elements set to negative `1`.
            #[inline]
            pub fn neg_one() -> Self {
                Self { $($field: -T::one() ),+ }
            }
        }

        impl<T: DigitFloat> $PointN<T> {
            /// Returns a `Point` with all elements set to `NaN`.
            #[inline]
            pub fn nan() -> Self {
                Self { $($field: T::nan() ),+ }
            }
        }

        impl<T: Digit + PartialEq> $PointN<T> {
            /// Returns a `boolean` `Point` of a `==` comparison elements of `self` and `rhs`.
            #[inline]
            pub fn cmpeq(&self, rhs: &Self) -> $PointN<bool> {
                $PointN::<bool>::new($(self.$field.eq(&rhs.$field) ),+)
            }

            /// Returns a `boolean` `Point` of a `!=` comparison elements of `self` and `rhs`.
            #[inline]
            pub fn cmpne(&self, rhs: &Self) -> $PointN<bool> {
                $PointN::<bool>::new($(self.$field.ne(&rhs.$field) ),+)
            }
        }

        impl<T: Digit + PartialOrd> $PointN<T> {
            /// Returns a `boolean` `Point` of a `>=` comparison elements of `self` and `rhs`.
            #[inline]
            pub fn cmpge(&self, rhs: &Self) -> $PointN<bool> {
                $PointN::<bool>::new($(self.$field.ge(&rhs.$field) ),+)
            }

            /// Returns a `boolean` `Point` of a `>` comparison elements of `self` and `rhs`.
            #[inline]
            pub fn cmpgt(&self, rhs: &Self) -> $PointN<bool> {
                $PointN::<bool>::new($(self.$field.gt(&rhs.$field) ),+)
            }

            /// Returns a `boolean` `Point` of a `<=` comparison elements of `self` and `rhs`.
            #[inline]
            pub fn cmple(&self, rhs: &Self) -> $PointN<bool> {
                $PointN::<bool>::new($(self.$field.le(&rhs.$field) ),+)
            }

            /// Returns a `boolean` `Point` of a `<` comparison elements of `self` and `rhs`.
            #[inline]
            pub fn cmplt(&self, rhs: &Self) -> $PointN<bool> {
                $PointN::<bool>::new($(self.$field.lt(&rhs.$field) ),+)
            }
        }

        impl<T: DigitNum + Signed> $PointN<T> {
            /// Returns a `Point` with absolute values of `self`.
            #[inline]
            pub fn abs(self) -> Self {
                Self { $($field: self.$field.abs() ),+ }
            }

            /// Returns a `Point` with representing sign values of `self`.
            #[inline]
            pub fn signum(self) -> Self {
                Self { $($field: self.$field.signum() ),+ }
            }
        }

        impl<T: DigitNum + Signed + PartialOrd> $PointN<T> {
            /// Returns `true` if the absolute difference of all elements between `self` and `rhs` <= `max_abs_diff`.
            #[inline]
            pub fn abs_diff_eq(self, rhs: Self, max_abs_diff: T) -> bool {
                (self - rhs.to_vector()).abs().cmple(&Self::splat(max_abs_diff)).all()
            }
        }

        impl<T: DigitNum + PartialOrd> $PointN<T> {
            /// Returns a `Point` with minimum values of `self` and `min`.
            #[inline]
            pub fn min(self, min: Self) -> Self {
                Self { $($field: clamp_min(self.$field, min.$field) ),+ }
            }

            /// Returns a `Point` with maximum values of `self` and `max`.
            #[inline]
            pub fn max(self, max: Self) -> Self {
                Self { $($field: clamp_max(self.$field, max.$field) ),+ }
            }

            /// Returns a `Point` with clamp values of `self` between `min` and `max`.
            #[inline]
            pub fn clamp(self, min: Self, max: Self) -> Self {
                Self { $($field: clamp(self.$field, min.$field, max.$field) ),+ }
            }
        }

        impl<T: DigitFloat> $PointN<T> {
            /// Returns a `Point` with the nearest integer to a number for values of `self`.
            #[inline]
            pub fn round(self) -> Self {
                Self { $($field: self.$field.round() ),+ }
            }

            /// Returns a `Point` with the largest integer less than or equal to a number for values of `self`.
            #[inline]
            pub fn floor(self) -> Self {
                Self { $($field: self.$field.floor() ),+ }
            }

            /// Returns a `Point` with containing the smallest integer greater than or equal to a number for values of `self`.
            #[inline]
            pub fn ceil(self) -> Self {
                Self { $($field: self.$field.ceil() ),+ }
            }

            /// Returns a `Point` with fractional values of `self`.
            #[inline]
            pub fn fract(self) -> Self {
                Self { $($field: self.$field.fract() ),+ }
            }

            /// Returns a `Point` with exponential function for values of `self`.
            #[inline]
            pub fn exp(self) -> Self {
                Self { $($field: self.$field.exp() ),+ }
            }

            /// Returns a `Point` with raised values of `self` to the power of `n`.
            #[inline]
            pub fn powf(self, n: T) -> Self {
                Self { $($field: self.$field.powf(n) ),+ }
            }

            /// Returns a `Point` with reciprocaled values of `self`
            #[inline]
            pub fn recip(self) -> Self {
                Self { $($field: self.$field.recip() ),+ }
            }

            /// Returns `true` if all values of `self` are finite.
            #[inline]
            pub fn is_finite(self) -> bool {
                $(self.$field.is_finite() )&&+
            }

            /// Returns `true` if any values of `self` are `NaN`.
            #[inline]
            pub fn is_nan(self) -> bool {
                $(self.$field.is_nan() )||+
            }
        }

        impl $PointN<bool> {
            #[inline]
            pub const fn any(self) -> bool {
                $(self.$field )||+
            }

            #[inline]
            pub const fn all(self) -> bool {
                $(self.$field )&&+
            }
        }

        crate::impl_operator!(<T: DigitNum> Add<$VectorN<T>> for $PointN<T> {
            fn add(lhs, rhs) -> $PointN<T> {
                $PointN::new($(lhs.$field + rhs.$field),+)
            }
        });

        crate::impl_assign_operator!(<T: DigitNum> AddAssign<$VectorN<T>> for $PointN<T> {
            fn add_assign(&mut self, rhs) {
                $(self.$field += rhs.$field);+
            }
        });

        crate::impl_operator!(<T: DigitNum> Sub<$VectorN<T>> for $PointN<T> {
            fn sub(lhs, rhs) -> $PointN<T> {
                $PointN::new($(lhs.$field - rhs.$field),+)
            }
        });

        crate::impl_assign_operator!(<T: DigitNum> SubAssign<$VectorN<T>> for $PointN<T> {
            fn sub_assign(&mut self, rhs) {
                $(self.$field -= rhs.$field);+
            }
        });

        crate::impl_operator!(<T: DigitNum> Mul<T> for $PointN<T> {
            fn mul(lhs, rhs) -> $PointN<T> {
                $PointN::new($(lhs.$field * rhs),+)
            }
        });

        crate::impl_assign_operator!(<T: DigitNum> MulAssign<T> for $PointN<T> {
            fn mul_assign(&mut self, rhs) {
                $(self.$field *= rhs);+
            }
        });

        crate::impl_operator!(<T: DigitNum> Div<T> for $PointN<T> {
            fn div(lhs, rhs) -> $PointN<T> {
                $PointN::new($(lhs.$field / rhs),+)
            }
        });

        crate::impl_assign_operator!(<T: DigitNum> DivAssign<T> for $PointN<T> {
            fn div_assign(&mut self, rhs) {
                $(self.$field /= rhs);+
            }
        });

        crate::impl_operator!(<T: DigitNum> Rem<T> for $PointN<T> {
            fn rem(lhs, rhs) -> $PointN<T> {
                $PointN::new($(lhs.$field % rhs),+)
            }
        });

        crate::impl_assign_operator!(<T: DigitNum> RemAssign<T> for $PointN<T> {
            fn rem_assign(&mut self, rhs) {
                $(self.$field %= rhs);+
            }
        });

        crate::impl_index_operators!($PointN<T>, $n, T, usize);
        crate::impl_index_operators!($PointN<T>, $n, [T], Range<usize>);
        crate::impl_index_operators!($PointN<T>, $n, [T], RangeTo<usize>);
        crate::impl_index_operators!($PointN<T>, $n, [T], RangeFrom<usize>);
        crate::impl_index_operators!($PointN<T>, $n, [T], RangeFull);
    };
}

impl_point!(
    /// A 2-dimensional point in space.
    => Point2 { x, y }, Vector2, 2
);
crate::impl_array_conversions!(Point2<T: Digit> { x: 0, y: 1 }, 2);
crate::impl_tuple_conversions!(Point2<T: Digit> { x, y }, (T, T));

impl<T: DigitNum> Point2<T> {
    #[inline]
    pub fn from_homogeneous(v: Vector3<T>) -> Self {
        let e = v.truncate() * (T::one() / v.z);
        Point2::new(e.x, e.y)
    }

    #[inline]
    pub fn to_homogeneous(self) -> Vector3<T> { Vector3::new(self.x, self.y, T::one()) }
}

impl_point!(
    /// A 3-dimensional point in space.
    => Point3 { x, y, z }, Vector3, 3
);
crate::impl_array_conversions!(Point3<T: Digit> { x: 0, y: 1, z: 2 }, 3);
crate::impl_tuple_conversions!(Point3<T: Digit> { x, y, z }, (T, T, T));

impl<T: DigitNum> Point3<T> {
    #[inline]
    pub fn from_homogeneous(v: Vector4<T>) -> Self {
        let e = v.truncate() * (T::one() / v.w);

        Point3::new(e.x, e.y, e.z)
    }

    #[inline]
    pub fn to_homogeneous(self) -> Vector4<T> { Vector4::new(self.x, self.y, self.z, T::one()) }
}
