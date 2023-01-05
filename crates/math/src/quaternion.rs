//! Quaternionernion functionality.

use crate::{
    digit::DigitFloat,
    matrix::{Matrix3x3, Matrix4x4},
    vector::Vector3,
};
use qinetic_utils::prelude::*;
use std::ops::*;

macro_rules! impl_quaternion {
    ($(#[$attr:meta])* => $QuaternionN:ident { $($field:ident),+ }, $n:expr) => {
        $(#[$attr])*
        #[derive(
            SmartDefault,
            Clone,
            Copy,
            Debug,
            PartialEq,
            PartialOrd,
            Add,
            AddAssign,
            Sub,
            SubAssign,
            Mul,
            MulAssign,
            Div,
            DivAssign,
            Sum,
            Product,
            Neg
        )]
        #[mul(forward)]
        #[mul_assign(forward)]
        #[div(forward)]
        #[div_assign(forward)]
        pub struct $QuaternionN<T: DigitFloat> {
            $(pub $field: T),+
        }

        impl<T: DigitFloat> $QuaternionN<T> {
            /// Returns a `Quaternion` with given values
            #[inline(always)]
            pub const fn new($($field: T),+) -> Self {
                Self { $($field),+ }
            }

            /// Returns a `Quaternion` with all elements set to `0`.
            #[inline]
            pub fn zero() -> Self {
                Self { $($field: T::zero() ),+ }
            }

            /// Returns a `Quaternion` with all elements set to `1`.
            #[inline]
            pub fn one() -> Self {
                Self { $($field: T::one() ),+ }
            }

            /// Returns a `Quaternion` with all elements set to negative `1`.
            #[inline]
            pub fn neg_one() -> Self {
                Self { $($field: -T::one() ),+ }
            }

            #[inline]
            fn sum(self) -> T {
                crate::fold_array!(add, $(self.$field),+ )
            }
        }

        impl<T: DigitFloat> $QuaternionN<T> {
            /// Returns a `Quaternion` with dot product of `self` and `other`.
            #[inline]
            pub fn dot(self, other: Self) -> T {
                Self { $($field: self.$field * other.$field ),+ }.sum()
            }

            /// Returns length of `self`.
            #[inline]
            pub fn length(self) -> T {
                self.dot(self).sqrt()
            }

            /// Returns squared length of `self`.
            #[inline]
            pub fn length_squared(self) -> T {
                self.dot(self)
            }

            /// Returns a `Quaternion` with normalized length of `self`.
            #[inline]
            pub fn normalize(self) -> Self {
                self.mul(self.length().recip())
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

        crate::impl_operator!(<T: DigitFloat> Add<T> for $QuaternionN<T> {
            fn add(lhs, rhs) -> $QuaternionN<T> {
                $QuaternionN::new($(lhs.$field + rhs),+)
            }
        });

        crate::impl_assign_operator!(<T: DigitFloat> AddAssign<T> for $QuaternionN<T> {
            fn add_assign(&mut self, rhs) {
                $(self.$field += rhs);+
            }
        });

        crate::impl_operator!(<T: DigitFloat> Sub<T> for $QuaternionN<T> {
            fn sub(lhs, rhs) -> $QuaternionN<T> {
                $QuaternionN::new($(lhs.$field - rhs),+)
            }
        });

        crate::impl_assign_operator!(<T: DigitFloat> SubAssign<T> for $QuaternionN<T> {
            fn sub_assign(&mut self, rhs) {
                $(self.$field -= rhs);+
            }
        });

        crate::impl_operator!(<T: DigitFloat> Mul<T> for $QuaternionN<T> {
            fn mul(lhs, rhs) -> $QuaternionN<T> {
                $QuaternionN::new($(lhs.$field * rhs),+)
            }
        });

        crate::impl_assign_operator!(<T: DigitFloat> MulAssign<T> for $QuaternionN<T> {
            fn mul_assign(&mut self, rhs) {
                $(self.$field *= rhs);+
            }
        });

        crate::impl_operator!(<T: DigitFloat> Div<T> for $QuaternionN<T> {
            fn div(lhs, rhs) -> $QuaternionN<T> {
                $QuaternionN::new($(lhs.$field / rhs),+)
            }
        });

        crate::impl_assign_operator!(<T: DigitFloat> DivAssign<T> for $QuaternionN<T> {
            fn div_assign(&mut self, rhs) {
                $(self.$field /= rhs);+
            }
        });
    };
}

impl_quaternion!(
    /// A quaternion representing an orientation.
    => Quaternion { x, y, z, w },
    4
);
crate::impl_array_conversions!(Quaternion<T: DigitFloat> { x: 0, y: 1, z: 2, w: 3 }, 4);
crate::impl_tuple_conversions!(Quaternion<T: DigitFloat> { x, y, z, w }, (T, T, T, T));

impl<T: DigitFloat> Quaternion<T> {
    /// Returns a `Quaternion` with all elements are `0`, expect `w` which is `1`.
    #[inline]
    pub fn identity() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
            w: T::one(),
        }
    }

    /// Returns a `Quaternion` for a normalized rotation `axis` and `angle` (in radians).
    #[inline]
    pub fn from_axis_angle(axis: Vector3<T>, angle: T) -> Self {
        let zpf = T::one() / (T::one() + T::one());

        let (s, c) = (angle * zpf).sin_cos();
        let v = axis * s;
        Self::new(v.x, v.y, v.z, c)
    }

    /// Returns a `Quaternion` that rotates `v.length()` radians around `v.normalize()`.
    #[inline]
    pub fn from_scaled_axis(v: Vector3<T>) -> Self {
        let length = v.length();
        if length == T::zero() {
            Self::identity()
        } else {
            Self::from_axis_angle(v / length, length)
        }
    }

    /// Returns a `Quaternion` from the columns of a 3x[`Vector3`].
    #[inline]
    pub(crate) fn from_rotation_axes(
        x_axis: Vector3<T>,
        y_axis: Vector3<T>,
        z_axis: Vector3<T>,
    ) -> Self {
        let zpf = T::one() / (T::one() + T::one());

        let (m00, m01, m02) = x_axis.into();
        let (m10, m11, m12) = y_axis.into();
        let (m20, m21, m22) = z_axis.into();
        if m22 <= T::zero() {
            // x^2 + y^2 >= z^2 + w^2
            let dif10 = m11 - m00;
            let omm22 = T::one() - m22;

            if dif10 <= T::zero() {
                // x^2 >= y^2
                let four_xsq = omm22 - dif10;
                let inv4x = zpf / four_xsq.sqrt();
                Self::new(
                    four_xsq * inv4x,
                    (m01 + m10) * inv4x,
                    (m02 + m20) * inv4x,
                    (m12 - m21) * inv4x,
                )
            } else {
                // y^2 >= x^2
                let four_ysq = omm22 + dif10;
                let inv4y = zpf / four_ysq.sqrt();
                Self::new(
                    (m01 + m10) * inv4y,
                    four_ysq * inv4y,
                    (m12 + m21) * inv4y,
                    (m20 - m02) * inv4y,
                )
            }
        } else {
            // z^2 + w^2 >= x^2 + y^2
            let sum10 = m11 + m00;
            let opm22 = T::one() + m22;

            if sum10 <= T::zero() {
                // z^2 >= w^2
                let four_zsq = opm22 - sum10;
                let inv4z = zpf / four_zsq.sqrt();
                Self::new(
                    (m02 + m20) * inv4z,
                    (m12 + m21) * inv4z,
                    four_zsq * inv4z,
                    (m01 - m10) * inv4z,
                )
            } else {
                //  w^2 >= z^2
                let four_wsq = opm22 + sum10;
                let inv4w = zpf / four_wsq.sqrt();
                Self::new(
                    (m12 - m21) * inv4w,
                    (m20 - m02) * inv4w,
                    (m01 - m10) * inv4w,
                    four_wsq * inv4w,
                )
            }
        }
    }

    /// Returns a `Quaternion` from the `angle` (in radians) around the x axis.
    #[inline]
    pub fn from_rotation_x(angle: T) -> Self {
        let zpf = T::one() / (T::one() + T::one());

        let (s, c) = (angle * zpf).sin_cos();
        Self::new(s, T::zero(), T::zero(), c)
    }

    /// Returns a `Quaternion` from the `angle` (in radians) around the y axis.
    #[inline]
    pub fn from_rotation_y(angle: T) -> Self {
        let zpf = T::one() / (T::one() + T::one());

        let (s, c) = (angle * zpf).sin_cos();
        Self::new(T::zero(), s, T::zero(), c)
    }

    /// Returns a `Quaternion` from the `angle` (in radians) around the z axis.
    #[inline]
    pub fn from_rotation_z(angle: T) -> Self {
        let zpf = T::one() / (T::one() + T::one());

        let (s, c) = (angle * zpf).sin_cos();
        Self::new(T::zero(), T::zero(), s, c)
    }

    /// Returns a `Quaternion` from a [`Matrix3x3`].
    #[inline]
    pub fn from_mat3(mat: &Matrix3x3<T>) -> Self {
        Self::from_rotation_axes(mat.x_axis, mat.y_axis, mat.z_axis)
    }

    /// Returns a `Quaternion` from a [`Matrix4x4`].
    #[inline]
    pub fn from_mat4(mat: &Matrix4x4<T>) -> Self {
        Self::from_rotation_axes(mat.x_axis.into(), mat.y_axis.into(), mat.z_axis.into())
    }

    /// Returns a `Quaternion` with linear interpolation between `self` and `other` based on the value `s`.
    #[inline]
    pub fn lerp(self, other: Self, s: T) -> Self {
        let start = self;
        let dot = start.dot(other);
        let bias = if dot >= T::zero() {
            T::one()
        } else {
            -T::one()
        };
        let interpolated = start.add(other.mul(bias).sub(start).mul(s));
        interpolated.normalize()
    }

    /// Returns a `Quaternion` inverse of `self`
    #[inline]
    pub fn inverse(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }
}
