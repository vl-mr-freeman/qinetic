//! Matrix functionality.

use std::{
    iter::{Product, Sum},
    ops::*,
};

use num_traits::Signed;
use qinetic_utils::prelude::*;

use crate::{
    digit::{Digit, DigitFloat, DigitNum},
    point::{Point2, Point3},
    quaternion::Quaternion,
    vector::{Vector2, Vector3, Vector4},
};

macro_rules! impl_matrix {
    ($(#[$attr:meta])* => $MatrixN:ident { $($field:ident),+ }, $VectorN:ident, $n:expr) => {
        $(#[$attr])*
        #[derive(SmartDefault, Clone, Copy, Debug, PartialEq, PartialOrd, Neg)]
        pub struct $MatrixN<T: Digit> {
            $(
                pub $field: $VectorN<T>
            ),+
        }

        impl<T: Digit> $MatrixN<T> {
            /// Returns a `Matrix` with given values.
            #[inline(always)]
            pub const fn new($($field: $VectorN<T>),+) -> Self {
                Self { $($field),+ }
            }

            /// Returns a `Matrix` with all elements set to `m`.
            #[inline(always)]
            pub const fn splat(m: $VectorN<T>) -> Self {
                Self { $($field: m),+ }
            }

            /// Returns a `Matrix` from elements `if_true` or `if_false`, by `mask`.
            #[inline(always)]
            pub const fn mask(mask: $MatrixN<bool>, if_true: Self, if_false: Self) -> Self {
                Self { $($field: $VectorN::mask(mask.$field, if_true.$field, if_false.$field) ),+ }
            }
        }

        impl<T: DigitNum> $MatrixN<T> {
            /// Returns a `Matrix` with all elements set to `0`.
            #[inline]
            pub fn zero() -> Self {
                Self { $($field: $VectorN::<T>::zero() ),+ }
            }

            /// Returns a `Matrix` with all elements set to `1`.
            #[inline]
            pub fn one() -> Self {
                Self { $($field: $VectorN::<T>::one() ),+ }
            }
        }

        impl<T: DigitFloat> $MatrixN<T> {
            /// Returns a `Matrix` with all elements set to `Nan`.
            #[inline]
            pub fn nan() -> Self {
                Self { $($field: $VectorN::nan() ),+ }
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

        impl<T: Digit + PartialEq> $MatrixN<T> {
            /// Returns a `boolean` `Matrix``==` comparison elements of `self` and `rhs`.
            #[inline]
            pub fn cmpeq(&self, rhs: &Self) -> $MatrixN<bool> {
                $MatrixN::<bool>::new($(self.$field.cmpeq(&rhs.$field) ),+)
            }

            /// Returns a `boolean` `Matrix``!=` comparison elements of `self` and `rhs`.
            #[inline]
            pub fn cmpne(&self, rhs: &Self) -> $MatrixN<bool> {
                $MatrixN::<bool>::new($(self.$field.cmpne(&rhs.$field) ),+)
            }
        }

        impl<T: Digit + PartialOrd> $MatrixN<T> {
            /// Returns a `boolean` `Matrix``>=` comparison elements of `self` and `rhs`.
            #[inline]
            pub fn cmpge(&self, rhs: &Self) -> $MatrixN<bool> {
                $MatrixN::<bool>::new($(self.$field.cmpge(&rhs.$field) ),+)
            }

            /// Returns a `boolean` `Matrix``>` comparison elements of `self` and `rhs`.
            #[inline]
            pub fn cmpgt(&self, rhs: &Self) -> $MatrixN<bool> {
                $MatrixN::<bool>::new($(self.$field.cmpgt(&rhs.$field) ),+)
            }

            /// Returns a `boolean` `Matrix``<=` comparison elements of `self` and `rhs`.
            #[inline]
            pub fn cmple(&self, rhs: &Self) -> $MatrixN<bool> {
                $MatrixN::<bool>::new($(self.$field.cmple(&rhs.$field) ),+)
            }

            /// Returns a `boolean` `Matrix``<` comparison elements of `self` and `rhs`.
            #[inline]
            pub fn cmplt(&self, rhs: &Self) -> $MatrixN<bool> {
                $MatrixN::<bool>::new($(self.$field.cmplt(&rhs.$field) ),+)
            }
        }

        impl $MatrixN<bool> {
            #[inline]
            pub const fn any(self) -> bool {
                $(self.$field.any() )||+
            }

            #[inline]
            pub const fn all(self) -> bool {
                $(self.$field.all() )&&+
            }
        }


        impl<T: DigitNum> Sum for $MatrixN<T> {
            #[inline]
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::zero(), Self::add)
            }
        }

        impl<'a, T: DigitNum> Sum<&'a Self> for $MatrixN<T> {
            #[inline]
            fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                iter.fold(Self::zero(), |a, &b| Self::add(a, b))
            }
        }

        impl<T: DigitNum> Product for $MatrixN<T> {
            #[inline]
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::one(), Self::mul)
            }
        }

        impl<'a, T: DigitNum> Product<&'a Self> for $MatrixN<T> {
            #[inline]
            fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                iter.fold(Self::one(), |a, &b| Self::mul(a, b))
            }
        }

        crate::impl_operator!(<T: DigitNum> Add<$MatrixN<T>> for $MatrixN<T> {
            fn add(lhs, rhs) -> $MatrixN<T> { $MatrixN::new($(lhs.$field + rhs.$field),+) }
        });

        crate::impl_assign_operator!(<T: DigitNum> AddAssign<$MatrixN<T>> for $MatrixN<T> {
            fn add_assign(&mut self, rhs) { $(self.$field += rhs.$field);+ }
        });

        crate::impl_operator!(<T: DigitNum> Sub<$MatrixN<T>> for $MatrixN<T> {
            fn sub(lhs, rhs) -> $MatrixN<T> { $MatrixN::new($(lhs.$field - rhs.$field),+) }
        });

        crate::impl_assign_operator!(<T: DigitNum> SubAssign<$MatrixN<T>> for $MatrixN<T> {
            fn sub_assign(&mut self, rhs) { $(self.$field -= rhs.$field);+ }
        });

        crate::impl_operator!(<T: DigitNum> Mul<$MatrixN<T>> for $MatrixN<T> {
            fn mul(lhs, rhs) -> $MatrixN<T> { $MatrixN::new($(lhs.$field * rhs.$field),+) }
        });

        crate::impl_assign_operator!(<T: DigitNum> MulAssign<$MatrixN<T>> for $MatrixN<T> {
            fn mul_assign(&mut self, rhs) { $(self.$field *= rhs.$field);+ }
        });

        crate::impl_operator!(<T: DigitNum> Div<$MatrixN<T>> for $MatrixN<T> {
            fn div(lhs, rhs) -> $MatrixN<T> { $MatrixN::new($(lhs.$field / rhs.$field),+) }
        });

        crate::impl_assign_operator!(<T: DigitNum> DivAssign<$MatrixN<T>> for $MatrixN<T> {
            fn div_assign(&mut self, rhs) { $(self.$field /= rhs.$field);+ }
        });

        crate::impl_operator!(<T: DigitNum> Rem<$MatrixN<T>> for $MatrixN<T> {
            fn rem(lhs, rhs) -> $MatrixN<T> { $MatrixN::new($(lhs.$field % rhs.$field),+) }
        });

        crate::impl_assign_operator!(<T: DigitNum> RemAssign<$MatrixN<T>> for $MatrixN<T> {
            fn rem_assign(&mut self, rhs) { $(self.$field %= rhs.$field);+ }
        });

        crate::impl_matrix_index_operators!($MatrixN<T: Digit>, $n, $VectorN<T>, $VectorN, usize);
        crate::impl_matrix_index_operators!($MatrixN<T: Digit>, $n, [$VectorN<T>], $VectorN, Range<usize>);
        crate::impl_matrix_index_operators!($MatrixN<T: Digit>, $n, [$VectorN<T>], $VectorN, RangeTo<usize>);
        crate::impl_matrix_index_operators!($MatrixN<T: Digit>, $n, [$VectorN<T>], $VectorN, RangeFrom<usize>);
        crate::impl_matrix_index_operators!($MatrixN<T: Digit>, $n, [$VectorN<T>], $VectorN, RangeFull);
    };
}

macro_rules! impl_mv_operator {
    ($MatrixN:ident, $VectorN:ident { $($field:ident : $row_index:expr),+ }) => {
        crate::impl_operator!(<S: DigitFloat> Mul<$VectorN<S> > for $MatrixN<S> {
            fn mul(matrix, vector) -> $VectorN<S> {$VectorN::new($(matrix[$row_index].dot(vector.clone())),+)}
        });
    }
}

impl_matrix!(
    /// A 2x2 column major matrix.
    => Matrix2x2 { x_axis, y_axis },
    Vector2,
    2
);
crate::impl_matrix_array_conversions!(Matrix2x2<T: Digit> { x_axis: 0, y_axis: 1 }, Vector2, 2);
crate::impl_matrix_tuple_conversions!(Matrix2x2<T: Digit> { x_axis, y_axis }, (Vector2, Vector2));
impl_mv_operator!(Matrix2x2, Vector2 { x: 0, y: 1 });

impl<T: DigitNum> Matrix2x2<T> {
    /// Returns a `Matrix` with all diagonal elements are `1`, and all off-diagonal elements are `0`.
    #[inline]
    pub fn identity() -> Self { Self::new(Vector2::unit_x(), Vector2::unit_y()) }

    /// Returns a `Matrix` with its diagonal set to `diagonal` and all other entries set to `0`.
    #[inline]
    pub fn from_diagonal(diagonal: Vector2<T>) -> Self {
        Self::new(
            Vector2::new(diagonal.x, T::zero()),
            Vector2::new(T::zero(), diagonal.y),
        )
    }

    /// Returns a `Matrix` with transpose from `self`.
    #[inline]
    pub fn transpose(&self) -> Self {
        Self::new(
            Vector2::new(self.x_axis.x, self.y_axis.x),
            Vector2::new(self.x_axis.y, self.y_axis.y),
        )
    }

    /// Returns the determinant of `self`.
    #[inline]
    pub fn determinant(&self) -> T {
        let (m00, m01) = self.x_axis.into();
        let (m10, m11) = self.y_axis.into();

        m00 * m11 - m01 * m10
    }
}

impl<T: DigitFloat> Matrix2x2<T> {
    /// Returns a `Matrix` with the combining non-uniform `scale` and rotation of `angle` (in radians).
    #[inline]
    pub fn from_scale_angle(scale: Vector2<T>, angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();

        Self::new(
            Vector2::new(cos * scale.x, sin * scale.x),
            Vector2::new(-sin * scale.y, cos * scale.y),
        )
    }

    #[inline]
    pub fn from_angle(angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();

        Self::new(Vector2::new(cos, sin), Vector2::new(-sin, cos))
    }
}

impl_matrix!(
    /// A 2x3 column major matrix.
    => Matrix2x3 { x_axis, y_axis },
    Vector3,
    2
);
crate::impl_matrix_array_conversions!(Matrix2x3<T: Digit> { x_axis: 0, y_axis: 1 }, Vector3, 2);
crate::impl_matrix_tuple_conversions!(Matrix2x3<T: Digit> { x_axis, y_axis }, (Vector3, Vector3));
impl_mv_operator!(Matrix2x3, Vector3 { x: 0, y: 1, z: 2 });

impl<T: DigitNum> Matrix2x3<T> {
    /// Returns a `Matrix` with all diagonal elements are `1`, and all off-diagonal elements are `0`.
    #[inline]
    pub fn identity() -> Self { Self::new(Vector3::unit_x(), Vector3::unit_y()) }

    /// Returns a `Matrix` with its diagonal set to `diagonal` and all other entries set to `0`.
    #[inline]
    pub fn from_diagonal(diagonal: Vector3<T>) -> Self {
        Self::new(
            Vector3::new(diagonal.x, T::zero(), T::zero()),
            Vector3::new(T::zero(), diagonal.y, T::zero()),
        )
    }

    /// Returns a `Matrix` with transpose from `self`.
    #[inline]
    pub fn transpose(&self) -> Matrix3x2<T> {
        Matrix3x2::new(
            Vector2::new(self.x_axis.x, self.y_axis.x),
            Vector2::new(self.x_axis.y, self.y_axis.y),
            Vector2::new(self.x_axis.z, self.y_axis.z),
        )
    }
}

impl_matrix!(
    /// A 2x4 column major matrix.
    => Matrix2x4 { x_axis, y_axis },
    Vector4,
    2
);
crate::impl_matrix_array_conversions!(Matrix2x4<T: Digit> { x_axis: 0, y_axis: 1 }, Vector4, 2);
crate::impl_matrix_tuple_conversions!(Matrix2x4<T: Digit> { x_axis, y_axis }, (Vector4, Vector4));
impl_mv_operator!(
    Matrix2x4,
    Vector4 {
        x: 0,
        y: 1,
        z: 2,
        w: 3
    }
);

impl<T: DigitNum> Matrix2x4<T> {
    /// Returns a `Matrix` with all diagonal elements are `1`, and all off-diagonal elements are `0`.
    #[inline]
    pub fn identity() -> Self { Self::new(Vector4::unit_x(), Vector4::unit_y()) }

    /// Returns a `Matrix` with its diagonal set to `diagonal` and all other entries set to `0`.
    #[inline]
    pub fn from_diagonal(diagonal: Vector4<T>) -> Self {
        Self::new(
            Vector4::new(diagonal.x, T::zero(), T::zero(), T::zero()),
            Vector4::new(T::zero(), diagonal.y, T::zero(), T::zero()),
        )
    }

    /// Returns a `Matrix` with transpose from `self`.
    #[inline]
    pub fn transpose(&self) -> Matrix4x2<T> {
        Matrix4x2::new(
            Vector2::new(self.x_axis.x, self.y_axis.x),
            Vector2::new(self.x_axis.y, self.y_axis.y),
            Vector2::new(self.x_axis.z, self.y_axis.z),
            Vector2::new(self.x_axis.w, self.y_axis.w),
        )
    }
}

impl_matrix!(
    /// A 3x2 column major matrix.
    => Matrix3x2 {
        x_axis,
        y_axis,
        z_axis
    },
    Vector2,
    3
);
crate::impl_matrix_array_conversions!(Matrix3x2<T: Digit> { x_axis: 0, y_axis: 1, z_axis: 2 }, Vector2, 3);
crate::impl_matrix_tuple_conversions!(Matrix3x2<T: Digit> { x_axis, y_axis, z_axis }, (Vector2, Vector2, Vector2));
impl_mv_operator!(Matrix3x2, Vector2 { x: 0, y: 1 });

impl<T: DigitNum> Matrix3x2<T> {
    /// Returns a `Matrix` with all diagonal elements are `1`, and all off-diagonal elements are `0`.
    #[inline]
    pub fn identity() -> Self { Self::new(Vector2::unit_x(), Vector2::unit_y(), Vector2::zero()) }

    /// Returns a `Matrix` with its diagonal set to `diagonal` and all other entries set to `0`.
    #[inline]
    pub fn from_diagonal(diagonal: Vector2<T>) -> Self {
        Self::new(
            Vector2::new(diagonal.x, T::zero()),
            Vector2::new(T::zero(), diagonal.y),
            Vector2::new(T::zero(), T::zero()),
        )
    }

    /// Returns a `Matrix` with transpose from `self`.
    #[inline]
    pub fn transpose(&self) -> Matrix2x3<T> {
        Matrix2x3::new(
            Vector3::new(self.x_axis.x, self.y_axis.x, self.z_axis.x),
            Vector3::new(self.x_axis.y, self.y_axis.y, self.z_axis.y),
        )
    }
}

impl_matrix!(
    /// A 3x3 column major matrix.
    => Matrix3x3 {
        x_axis,
        y_axis,
        z_axis
    },
    Vector3,
    3
);
crate::impl_matrix_array_conversions!(Matrix3x3<T: Digit> { x_axis: 0, y_axis: 1, z_axis: 2 }, Vector3, 3);
crate::impl_matrix_tuple_conversions!(Matrix3x3<T: Digit> { x_axis, y_axis, z_axis }, (Vector3, Vector3, Vector3));
impl_mv_operator!(Matrix3x3, Vector3 { x: 0, y: 1, z: 2 });

impl<T: DigitNum> Matrix3x3<T> {
    /// Returns a `Matrix` with all diagonal elements are `1`, and all off-diagonal elements are `0`.
    #[inline]
    pub fn identity() -> Self { Self::new(Vector3::unit_x(), Vector3::unit_y(), Vector3::unit_z()) }

    /// Returns a `Matrix` with its diagonal set to `diagonal` and all other entries set to `0`.
    #[inline]
    pub fn from_diagonal(diagonal: Vector3<T>) -> Self {
        Self::new(
            Vector3::new(diagonal.x, T::zero(), T::zero()),
            Vector3::new(T::zero(), diagonal.y, T::zero()),
            Vector3::new(T::zero(), T::zero(), diagonal.z),
        )
    }

    /// Returns a `Matrix` with transpose from `self`.
    #[inline]
    pub fn transpose(&self) -> Self {
        Self::new(
            Vector3::new(self.x_axis.x, self.y_axis.x, self.z_axis.x),
            Vector3::new(self.x_axis.y, self.y_axis.y, self.z_axis.y),
            Vector3::new(self.x_axis.z, self.y_axis.z, self.z_axis.z),
        )
    }

    /// Returns the determinant of `self`.
    #[inline]
    pub fn determinant(&self) -> T {
        let (m00, m01, m02) = self.x_axis.into();
        let (m10, m11, m12) = self.y_axis.into();
        let (m20, m21, m22) = self.z_axis.into();

        m00 * (m11 * m22 - m12 * m21)
            - m01 * (m10 * m22 - m12 * m20)
            - m02 * (m10 * m21 - m11 * m20)
    }
}

impl<T: DigitFloat> Matrix3x3<T> {
    /// Returns a `Matrix` from `rotation` [`Quaternion`].
    #[inline]
    pub fn from_quat(rotation: Quaternion<T>) -> Self {
        let x2 = rotation.x + rotation.x;
        let y2 = rotation.y + rotation.y;
        let z2 = rotation.z + rotation.z;
        let xx = rotation.x * x2;
        let xy = rotation.x * y2;
        let xz = rotation.x * z2;
        let yy = rotation.y * y2;
        let yz = rotation.y * z2;
        let zz = rotation.z * z2;
        let wx = rotation.w * x2;
        let wy = rotation.w * y2;
        let wz = rotation.w * z2;

        Self::new(
            Vector3::new(T::one() - (yy + zz), xy + wz, xz - wy),
            Vector3::new(xy - wz, T::one() - (xx + zz), yz + wx),
            Vector3::new(xz + wy, yz - wx, T::one() - (xx + yy)),
        )
    }

    /// Returns a `Matrix` from `angle` (in radians) around the x axis.
    #[inline]
    pub fn from_rotation_x(angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();

        Self::new(
            Vector3::unit_x(),
            Vector3::new(T::zero(), cos, sin),
            Vector3::new(T::zero(), -sin, cos),
        )
    }

    /// Returns a `Matrix` from `angle` (in radians) around the y axis.
    #[inline]
    pub fn from_rotation_y(angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();

        Self::new(
            Vector3::new(cos, T::zero(), -sin),
            Vector3::unit_y(),
            Vector3::new(sin, T::zero(), cos),
        )
    }

    /// Returns a `Matrix` from `angle` (in radians) around the z axis.
    #[inline]
    pub fn from_rotation_z(angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();

        Self::new(
            Vector3::new(cos, sin, T::zero()),
            Vector3::new(-sin, cos, T::zero()),
            Vector3::unit_z(),
        )
    }

    /// Returns a `Matrix` an affine transformation from the `translation`.
    #[inline]
    pub fn from_translation(translation: Vector2<T>) -> Self {
        Self::new(
            Vector3::unit_x(),
            Vector3::unit_y(),
            Vector3::new(translation.x, translation.y, T::one()),
        )
    }

    /// Returns a `Matrix` an affine transformation from the rotation `angle` (in radians).
    #[inline]
    pub fn from_angle(angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();

        Self::new(
            Vector3::new(cos, sin, T::zero()),
            Vector3::new(-sin, cos, T::zero()),
            Vector3::unit_z(),
        )
    }

    /// Returns a `Matrix` an affine transformation from the `scale`, rotation `angle` (in radians) and `translation`.
    #[inline]
    pub fn from_scale_angle_translation(
        scale: Vector2<T>,
        angle: T,
        translation: Vector2<T>,
    ) -> Self {
        let (sin, cos) = angle.sin_cos();

        Self::new(
            Vector3::new(cos * scale.x, sin * scale.x, T::zero()),
            Vector3::new(-sin * scale.y, cos * scale.y, T::zero()),
            Vector3::new(translation.x, translation.y, T::one()),
        )
    }

    /// Returns a `Matrix` with non-uniform `scale`.
    #[inline]
    pub fn from_scale(scale: Vector2<T>) -> Self {
        Self::new(
            Vector3::new(scale.x, T::zero(), T::zero()),
            Vector3::new(T::zero(), scale.y, T::zero()),
            Vector3::unit_z(),
        )
    }

    /// Transforms the given `Point` as a point.
    ///
    /// This method assumes that `self` contains a valid affine transform.
    ///
    /// It does not perform a persective divide, if `self` contains a perspective transform, or if you are unsure,
    /// the [`Self::project_point()`] method should be used instead.
    #[inline]
    pub fn transform_point(&self, point: Point2<T>) -> Point2<T> {
        Point2::from_vector((self * Point3::new(point.x, point.y, T::one()).to_vector()).truncate())
    }

    /// Transforms the give `Vector` as a direction.
    ///
    /// This method assumes that `self` contains a valid affine transform.
    #[inline]
    pub fn transform_vector(&self, vector: Vector2<T>) -> Vector2<T> {
        (self * vector.extend(T::zero())).truncate()
    }
}

impl_matrix!(
    /// A 3x4 column major matrix.
    => Matrix3x4 {
        x_axis,
        y_axis,
        z_axis
    },
    Vector4,
    3
);
crate::impl_matrix_array_conversions!(Matrix3x4<T: Digit> { x_axis: 0, y_axis: 1, z_axis: 2 }, Vector4, 3);
crate::impl_matrix_tuple_conversions!(Matrix3x4<T: Digit> { x_axis, y_axis, z_axis }, (Vector4, Vector4, Vector4));
impl_mv_operator!(
    Matrix3x4,
    Vector4 {
        x: 0,
        y: 1,
        z: 2,
        w: 3
    }
);

impl<T: DigitNum> Matrix3x4<T> {
    /// Returns a `Matrix` with all diagonal elements are `1`, and all off-diagonal elements are `0`.
    #[inline]
    pub fn identity() -> Self { Self::new(Vector4::unit_x(), Vector4::unit_y(), Vector4::unit_z()) }

    /// Returns a `Matrix` with its diagonal set to `diagonal` and all other entries set to `0`.
    #[inline]
    pub fn from_diagonal(diagonal: Vector4<T>) -> Self {
        Self::new(
            Vector4::new(diagonal.x, T::zero(), T::zero(), T::zero()),
            Vector4::new(T::zero(), diagonal.y, T::zero(), T::zero()),
            Vector4::new(T::zero(), T::zero(), diagonal.z, T::zero()),
        )
    }

    /// Returns a `Matrix` with transpose from `self`.
    #[inline]
    pub fn transpose(&self) -> Matrix4x3<T> {
        Matrix4x3::new(
            Vector3::new(self.x_axis.x, self.y_axis.x, self.z_axis.x),
            Vector3::new(self.x_axis.y, self.y_axis.y, self.z_axis.y),
            Vector3::new(self.x_axis.z, self.y_axis.z, self.z_axis.z),
            Vector3::new(self.x_axis.w, self.y_axis.w, self.z_axis.w),
        )
    }
}

impl_matrix!(
    /// A 4x2 column major matrix.
    => Matrix4x2 {
        x_axis,
        y_axis,
        z_axis,
        w_axis
    },
    Vector2,
    4
);
crate::impl_matrix_array_conversions!(Matrix4x2<T: Digit> { x_axis: 0, y_axis: 1, z_axis: 2, w_axis: 3 }, Vector2, 4);
crate::impl_matrix_tuple_conversions!(Matrix4x2<T: Digit> { x_axis, y_axis, z_axis, w_axis }, (Vector2, Vector2, Vector2, Vector2));
impl_mv_operator!(Matrix4x2, Vector2 { x: 0, y: 1 });

impl<T: DigitNum> Matrix4x2<T> {
    /// Returns a `Matrix` with all diagonal elements are `1`, and all off-diagonal elements are `0`.
    #[inline]
    pub fn identity() -> Self {
        Self::new(
            Vector2::unit_x(),
            Vector2::unit_y(),
            Vector2::zero(),
            Vector2::zero(),
        )
    }

    /// Returns a `Matrix` with its diagonal set to `diagonal` and all other entries set to `0`.
    #[inline]
    pub fn from_diagonal(diagonal: Vector2<T>) -> Self {
        Self::new(
            Vector2::new(diagonal.x, T::zero()),
            Vector2::new(T::zero(), diagonal.y),
            Vector2::new(T::zero(), T::zero()),
            Vector2::new(T::zero(), T::zero()),
        )
    }

    /// Returns a `Matrix` with transpose from `self`.
    #[inline]
    pub fn transpose(&self) -> Matrix2x4<T> {
        Matrix2x4::new(
            Vector4::new(self.x_axis.x, self.y_axis.x, self.z_axis.x, self.w_axis.x),
            Vector4::new(self.x_axis.y, self.y_axis.y, self.z_axis.y, self.w_axis.y),
        )
    }
}

impl_matrix!(
    /// A 4x3 column major matrix.
    => Matrix4x3 {
        x_axis,
        y_axis,
        z_axis,
        w_axis
    },
    Vector3,
    4
);
crate::impl_matrix_array_conversions!(Matrix4x3<T: Digit> { x_axis: 0, y_axis: 1, z_axis: 2, w_axis: 3 }, Vector3, 4);
crate::impl_matrix_tuple_conversions!(Matrix4x3<T: Digit> { x_axis, y_axis, z_axis, w_axis }, (Vector3, Vector3, Vector3, Vector3));
impl_mv_operator!(Matrix4x3, Vector3 { x: 0, y: 1, z: 2 });

impl<T: DigitNum> Matrix4x3<T> {
    /// Returns a `Matrix` with all diagonal elements are `1`, and all off-diagonal elements are `0`.
    #[inline]
    pub fn identity() -> Self {
        Self::new(
            Vector3::unit_x(),
            Vector3::unit_y(),
            Vector3::unit_z(),
            Vector3::zero(),
        )
    }

    /// Returns a `Matrix` with its diagonal set to `diagonal` and all other entries set to `0`.
    #[inline]
    pub fn from_diagonal(diagonal: Vector3<T>) -> Self {
        Self::new(
            Vector3::new(diagonal.x, T::zero(), T::zero()),
            Vector3::new(T::zero(), diagonal.y, T::zero()),
            Vector3::new(T::zero(), T::zero(), diagonal.z),
            Vector3::new(T::zero(), T::zero(), T::zero()),
        )
    }

    /// Returns a `Matrix` with transpose from `self`.
    #[inline]
    pub fn transpose(&self) -> Matrix3x4<T> {
        Matrix3x4::new(
            Vector4::new(self.x_axis.x, self.y_axis.x, self.z_axis.x, self.w_axis.x),
            Vector4::new(self.x_axis.y, self.y_axis.y, self.z_axis.y, self.w_axis.y),
            Vector4::new(self.x_axis.z, self.y_axis.z, self.z_axis.z, self.w_axis.z),
        )
    }
}

impl_matrix!(
    /// A 4x4 column major matrix.
    => Matrix4x4 {
        x_axis,
        y_axis,
        z_axis,
        w_axis
    },
    Vector4,
    4
);
crate::impl_matrix_array_conversions!(Matrix4x4<T: Digit> { x_axis: 0, y_axis: 1, z_axis: 2, w_axis: 3 }, Vector4, 4);
crate::impl_matrix_tuple_conversions!(Matrix4x4<T: Digit> { x_axis, y_axis, z_axis, w_axis }, (Vector4, Vector4, Vector4, Vector4));
impl_mv_operator!(
    Matrix4x4,
    Vector4 {
        x: 0,
        y: 1,
        z: 2,
        w: 3
    }
);

impl<T: DigitNum> Matrix4x4<T> {
    /// Returns a `Matrix` with all diagonal elements are `1`, and all off-diagonal elements are `0`.
    #[inline]
    pub fn identity() -> Self {
        Self::new(
            Vector4::unit_x(),
            Vector4::unit_y(),
            Vector4::unit_z(),
            Vector4::unit_w(),
        )
    }

    /// Returns a `Matrix` with its diagonal set to `diagonal` and all other entries set to `0`.
    #[inline]
    pub fn from_diagonal(diagonal: Vector4<T>) -> Self {
        Self::new(
            Vector4::new(diagonal.x, T::zero(), T::zero(), T::zero()),
            Vector4::new(T::zero(), diagonal.y, T::zero(), T::zero()),
            Vector4::new(T::zero(), T::zero(), diagonal.z, T::zero()),
            Vector4::new(T::zero(), T::zero(), T::zero(), diagonal.w),
        )
    }

    /// Returns a `Matrix` with transpose from `self`.
    #[inline]
    pub fn transpose(&self) -> Self {
        Self::new(
            Vector4::new(self.x_axis.x, self.y_axis.x, self.z_axis.x, self.x_axis.w),
            Vector4::new(self.x_axis.y, self.y_axis.y, self.z_axis.y, self.y_axis.w),
            Vector4::new(self.x_axis.z, self.y_axis.z, self.z_axis.z, self.z_axis.w),
            Vector4::new(self.x_axis.w, self.y_axis.w, self.z_axis.w, self.w_axis.w),
        )
    }

    /// Returns the determinant of `self`.
    #[inline]
    pub fn determinant(&self) -> T {
        let (m00, m01, m02, m03) = self.x_axis.into();
        let (m10, m11, m12, m13) = self.y_axis.into();
        let (m20, m21, m22, m23) = self.z_axis.into();
        let (m30, m31, m32, m33) = self.w_axis.into();

        let a2323 = m22 * m33 - m23 * m32;
        let a1323 = m21 * m33 - m23 * m31;
        let a1223 = m21 * m32 - m22 * m31;
        let a0323 = m20 * m33 - m23 * m30;
        let a0223 = m20 * m32 - m22 * m30;
        let a0123 = m20 * m31 - m21 * m30;

        m00 * (m11 * a2323 - m12 * a1323 + m13 * a1223)
            - m01 * (m10 * a2323 - m12 * a0323 + m13 * a0223)
            + m02 * (m10 * a1323 - m11 * a0323 + m13 * a0123)
            - m03 * (m10 * a1223 - m11 * a0223 + m12 * a0123)
    }
}

impl<T: DigitFloat + Signed> Matrix4x4<T> {
    /// Returns a `Matrix` with the `rotation`.
    #[inline]
    pub fn from_quat(rotation: Quaternion<T>) -> Self {
        let (x_axis, y_axis, z_axis) = Self::quat_to_axes(rotation);

        Self::new(x_axis, y_axis, z_axis, Vector4::unit_w())
    }

    /// Returns a `Matrix` from `angle` (in radians) around the x axis.
    #[inline]
    pub fn from_rotation_x(angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();

        Self::new(
            Vector4::unit_x(),
            Vector4::new(T::zero(), cos, sin, T::zero()),
            Vector4::new(T::zero(), -sin, cos, T::zero()),
            Vector4::unit_w(),
        )
    }

    /// Returns a `Matrix` from `angle` (in radians) around the y axis.
    #[inline]
    pub fn from_rotation_y(angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();

        Self::new(
            Vector4::new(cos, T::zero(), -sin, T::zero()),
            Vector4::unit_y(),
            Vector4::new(sin, T::zero(), cos, T::zero()),
            Vector4::unit_w(),
        )
    }

    /// Returns a `Matrix` from `angle` (in radians) around the z axis.
    #[inline]
    pub fn from_rotation_z(angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();

        Self::new(
            Vector4::new(cos, sin, T::zero(), T::zero()),
            Vector4::new(-sin, cos, T::zero(), T::zero()),
            Vector4::unit_z(),
            Vector4::unit_w(),
        )
    }

    /// Returns a `Matrix` with the `translation`.
    #[inline]
    pub fn from_translation(translation: Vector3<T>) -> Self {
        Self::new(
            Vector4::unit_x(),
            Vector4::unit_y(),
            Vector4::unit_z(),
            Vector4::new(translation.x, translation.y, translation.z, T::one()),
        )
    }

    /// Returns a `Matrix` with the `scale`, `rotation` and `translation`.
    #[inline]
    pub fn from_scale_rotation_translation(
        scale: Vector3<T>,
        rotation: Quaternion<T>,
        translation: Vector3<T>,
    ) -> Self {
        let (x_axis, y_axis, z_axis) = Self::quat_to_axes(rotation);

        Self::new(
            x_axis.mul(scale.x),
            y_axis.mul(scale.y),
            z_axis.mul(scale.z),
            Vector4::new(translation.x, translation.y, translation.z, T::one()),
        )
    }

    /// Returns a `Matrix` with the `rotation` and `translation`.
    #[inline]
    pub fn from_rotation_translation(rotation: Quaternion<T>, translation: Vector3<T>) -> Self {
        let (x_axis, y_axis, z_axis) = Self::quat_to_axes(rotation);

        Self::new(
            x_axis,
            y_axis,
            z_axis,
            Vector4::new(translation.x, translation.y, translation.z, T::one()),
        )
    }

    /// Returns a `Matrix` with rotation around a normalized rotation `axis` of `angle` (in radians).
    #[inline]
    pub fn from_axis_angle(axis: Vector3<T>, angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();

        let axis_sin = axis.mul(sin);

        let axis_sq = axis.mul(axis);

        let omc = T::one() - cos;

        let xyomc = axis.x * axis.y * omc;
        let xzomc = axis.x * axis.z * omc;
        let yzomc = axis.y * axis.z * omc;

        Self::new(
            Vector4::new(
                axis_sq.x * omc + cos,
                xyomc + axis_sin.z,
                xzomc - axis_sin.y,
                T::zero(),
            ),
            Vector4::new(
                xyomc - axis_sin.z,
                axis_sq.y * omc + cos,
                yzomc + axis_sin.x,
                T::zero(),
            ),
            Vector4::new(
                xzomc + axis_sin.y,
                yzomc - axis_sin.x,
                axis_sq.z * omc + cos,
                T::zero(),
            ),
            Vector4::unit_w(),
        )
    }

    /// Returns a `Matrix` with non-uniform `scale`.
    #[inline]
    pub fn from_scale(scale: Vector3<T>) -> Self {
        Self::new(
            Vector4::new(scale.x, T::zero(), T::zero(), T::zero()),
            Vector4::new(T::zero(), scale.y, T::zero(), T::zero()),
            Vector4::new(T::zero(), T::zero(), scale.z, T::zero()),
            Vector4::unit_w(),
        )
    }

    /// Returns a `Matrix` with left-handed view using camera position, an up direction, and a facing direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    #[inline]
    pub fn look_to_lh(eye: Vector3<T>, dir: Vector3<T>, up: Vector3<T>) -> Self {
        Self::look_to_rh(eye, dir.neg(), up)
    }

    /// Returns a `Matrix` with right-handed view using camera position, an up direction, and a facing direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    #[inline]
    pub fn look_to_rh(eye: Vector3<T>, dir: Vector3<T>, up: Vector3<T>) -> Self {
        let f = dir.normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(f);

        Self::new(
            Vector4::new(s.x, u.x, -f.x, T::zero()),
            Vector4::new(s.y, u.y, -f.y, T::zero()),
            Vector4::new(s.z, u.z, -f.z, T::zero()),
            Vector4::new(-eye.dot(s), -eye.dot(u), eye.dot(f), T::one()),
        )
    }

    /// Returns a `Matrix` with left-handed view matrix using a camera position, an up direction, and a focal point.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    #[inline]
    pub fn look_at_lh(eye: Vector3<T>, center: Vector3<T>, up: Vector3<T>) -> Self {
        Self::look_to_lh(eye, center.sub(eye), up)
    }

    /// Returns a `Matrix` with right-handed view matrix using a camera position, an up direction, and a focal point.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    #[inline]
    pub fn look_at_rh(eye: Vector3<T>, center: Vector3<T>, up: Vector3<T>) -> Self {
        Self::look_to_rh(eye, center.sub(eye), up)
    }

    /// Returns a `Matrix` with left-handed perspective projection with `[0,1]` depth range.
    #[inline]
    pub fn perspective_lh(fov_y_radians: T, aspect_ratio: T, z_near: T, z_far: T) -> Self {
        let zpf = T::one() / (T::one() + T::one());

        let (sin_fov, cos_fov) = (zpf * fov_y_radians).sin_cos();

        let h = cos_fov / sin_fov;
        let w = h / aspect_ratio;
        let r = z_far / (z_far - z_near);

        Self::new(
            Vector4::new(w, T::zero(), T::zero(), T::zero()),
            Vector4::new(T::zero(), h, T::zero(), T::zero()),
            Vector4::new(T::zero(), T::zero(), r, T::one()),
            Vector4::new(T::zero(), T::zero(), -r * z_near, T::zero()),
        )
    }

    /// Returns a `Matrix` with right-handed perspective projection with `[0,1]` depth range.
    #[inline]
    pub fn perspective_rh(fov_y_radians: T, aspect_ratio: T, z_near: T, z_far: T) -> Self {
        let zpf = T::one() / (T::one() + T::one());

        let (sin_fov, cos_fov) = (zpf * fov_y_radians).sin_cos();

        let h = cos_fov / sin_fov;
        let w = h / aspect_ratio;
        let r = z_far / (z_near - z_far);

        Self::new(
            Vector4::new(w, T::zero(), T::zero(), T::zero()),
            Vector4::new(T::zero(), h, T::zero(), T::zero()),
            Vector4::new(T::zero(), T::zero(), r, -T::one()),
            Vector4::new(T::zero(), T::zero(), r * z_near, T::zero()),
        )
    }

    /// Returns a `Matrix` with an infinite left-handed perspective projection with `[0,1]` depth range.
    #[inline]
    pub fn perspective_infinite_lh(fov_y_radians: T, aspect_ratio: T, z_near: T) -> Self {
        let zpf = T::one() / (T::one() + T::one());

        let (sin_fov, cos_fov) = (zpf * fov_y_radians).sin_cos();

        let h = cos_fov / sin_fov;
        let w = h / aspect_ratio;

        Self::new(
            Vector4::new(w, T::zero(), T::zero(), T::zero()),
            Vector4::new(T::zero(), h, T::zero(), T::zero()),
            Vector4::new(T::zero(), T::zero(), T::one(), T::one()),
            Vector4::new(T::zero(), T::zero(), -z_near, T::zero()),
        )
    }

    /// Returns a `Matrix` with an infinite left-handed perspective projection with `[0,1]` depth range.
    #[inline]
    pub fn perspective_infinite_reverse_lh(fov_y_radians: T, aspect_ratio: T, z_near: T) -> Self {
        let zpf = T::one() / (T::one() + T::one());

        let (sin_fov, cos_fov) = (zpf * fov_y_radians).sin_cos();
        let h = cos_fov / sin_fov;
        let w = h / aspect_ratio;
        Self::new(
            Vector4::new(w, T::zero(), T::zero(), T::zero()),
            Vector4::new(T::zero(), h, T::zero(), T::zero()),
            Vector4::new(T::zero(), T::zero(), T::zero(), T::one()),
            Vector4::new(T::zero(), T::zero(), z_near, T::zero()),
        )
    }

    /// Returns a `Matrix` with an infinite right-handed perspective projection with `[0,1]` depth range.
    #[inline]
    pub fn perspective_infinite_rh(fov_y_radians: T, aspect_ratio: T, z_near: T) -> Self {
        let zpf = T::one() / (T::one() + T::one());

        let f = T::one() / (zpf * fov_y_radians).tan();
        Self::new(
            Vector4::new(f / aspect_ratio, T::zero(), T::zero(), T::zero()),
            Vector4::new(T::zero(), f, T::zero(), T::zero()),
            Vector4::new(T::zero(), T::zero(), -T::one(), -T::one()),
            Vector4::new(T::zero(), T::zero(), -z_near, T::zero()),
        )
    }

    /// Returns a `Matrix` with an infinite reverse right-handed perspective projection with `[0,1]` depth range.
    #[inline]
    pub fn perspective_infinite_reverse_rh(fov_y_radians: T, aspect_ratio: T, z_near: T) -> Self {
        let zpf = T::one() / (T::one() + T::one());

        let f = T::one() / (zpf * fov_y_radians).tan();
        Self::new(
            Vector4::new(f / aspect_ratio, T::zero(), T::zero(), T::zero()),
            Vector4::new(T::zero(), f, T::zero(), T::zero()),
            Vector4::new(T::zero(), T::zero(), T::zero(), -T::one()),
            Vector4::new(T::zero(), T::zero(), z_near, T::zero()),
        )
    }

    /// Returns a `Matrix` with left-handed orthographic projection with `[0,1]` depth range.
    #[inline]
    pub fn orthographic_lh(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Self {
        let rcp_width = T::one() / (right - left);
        let rcp_height = T::one() / (top - bottom);
        let r = T::one() / (far - near);

        Self::new(
            Vector4::new(rcp_width + rcp_width, T::zero(), T::zero(), T::zero()),
            Vector4::new(T::zero(), rcp_height + rcp_height, T::zero(), T::zero()),
            Vector4::new(T::zero(), T::zero(), r, T::zero()),
            Vector4::new(
                -(left + right) * rcp_width,
                -(top + bottom) * rcp_height,
                -r * near,
                T::one(),
            ),
        )
    }

    /// Returns a `Matrix` with right-handed orthographic projection with `[0,1]` depth range.
    #[inline]
    pub fn orthographic_rh(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Self {
        let rcp_width = T::one() / (right - left);
        let rcp_height = T::one() / (top - bottom);
        let r = T::one() / (near - far);

        Self::new(
            Vector4::new(rcp_width + rcp_width, T::zero(), T::zero(), T::zero()),
            Vector4::new(T::zero(), rcp_height + rcp_height, T::zero(), T::zero()),
            Vector4::new(T::zero(), T::zero(), r, T::zero()),
            Vector4::new(
                -(left + right) * rcp_width,
                -(top + bottom) * rcp_height,
                r * near,
                T::one(),
            ),
        )
    }

    /// Transforms the given `Point` as a point.
    ///
    /// This method assumes that `self` contains a valid affine transform.
    ///
    /// It does not perform a persective divide, if `self` contains a perspective transform, or if you are unsure,
    /// the [`Self::project_point()`] method should be used instead.
    #[inline]
    pub fn transform_point(&self, point: Point3<T>) -> Point3<T> {
        Point3::from_homogeneous(self * point.to_homogeneous())
    }

    /// Transforms the give `Vector` as a direction.
    ///
    /// This method assumes that `self` contains a valid affine transform.
    #[inline]
    pub fn transform_vector(&self, vector: Vector3<T>) -> Vector3<T> {
        (self * vector.extend(T::zero())).truncate()
    }

    #[inline]
    fn quat_to_axes(rotation: Quaternion<T>) -> (Vector4<T>, Vector4<T>, Vector4<T>) {
        let (x, y, z, w) = rotation.into();
        let x2 = x + x;
        let y2 = y + y;
        let z2 = z + z;
        let xx = x * x2;
        let xy = x * y2;
        let xz = x * z2;
        let yy = y * y2;
        let yz = y * z2;
        let zz = z * z2;
        let wx = w * x2;
        let wy = w * y2;
        let wz = w * z2;

        let x_axis = Vector4::new(T::one() - (yy + zz), xy + wz, xz - wy, T::zero());
        let y_axis = Vector4::new(xy - wz, T::one() - (xx + zz), yz + wx, T::zero());
        let z_axis = Vector4::new(xz + wy, yz - wx, T::one() - (xx + yy), T::zero());
        (x_axis, y_axis, z_axis)
    }
}
