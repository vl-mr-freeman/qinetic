//! Vector functionality.

use crate::{Digit, DigitFloat, DigitNum};
use num_traits::{clamp, clamp_max as clamp_min, clamp_min as clamp_max, Signed};
use std::fmt;
use std::iter::{Product, Sum};
use std::ops::*;

macro_rules! impl_vector {
    ($(#[$attr:meta])* => $VectorN:ident { $($field:ident),+ }, $n:expr) => {
        $(#[$attr])*
        #[derive(Clone, Copy, PartialEq, PartialOrd, Default)]
        pub struct $VectorN<T: Digit> {
            $(
                pub $field: T
            ),+
        }

        impl<T: Digit> $VectorN<T> {
            /// Returns a `Vector` with given values.
            #[inline(always)]
            pub const fn new($($field: T),+) -> Self {
                Self { $($field),+ }
            }

            /// Returns a `Vector` with all elements set to `v`.
            #[inline(always)]
            pub const fn splat(v: T) -> Self {
                Self { $($field: v),+ }
            }

            /// Returns a `Vector` from elements `if_true` or `if_false`, by `mask`.
            #[inline(always)]
            pub const fn mask(mask: $VectorN<bool>, if_true: Self, if_false: Self) -> Self {
                Self {
                    $(
                        $field: if mask.$field { if_true.$field } else { if_false. $field },
                    )*
                }
            }
        }

        impl<T: DigitNum> $VectorN<T> {
            /// Returns a `Vector` with all elements set to `0`.
            #[inline]
            pub fn zero() -> Self {
                Self {
                    $(
                        $field: T::zero(),
                    )*
                }
            }

            /// Returns a `Vector` with all elements set to positive `1`.
            #[inline]
            pub fn one() -> Self {
                Self {
                    $(
                        $field: T::one(),
                    )*
                }
            }

            #[inline]
            fn sum(self) -> T {
                crate::fold_array!(add, $(self.$field),+ )
            }

            /// Returns a `Vector` with dot product of `self` and `rhs`.
            #[inline]
            pub fn dot(self, other: Self) -> T {
                Self {
                    $(
                        $field: self.$field * other.$field,
                    )*
                }
                .sum()
            }
        }

        impl<T: DigitNum + Signed> $VectorN<T> {
            /// Returns a `Vector` with all elements set to negative `1`.
            #[inline]
            pub fn neg_one() -> Self {
                Self {
                    $(
                        $field: -T::one(),
                    )*
                }
            }
        }

        impl<T: DigitFloat> $VectorN<T> {
            /// Returns a `Vector` with all elements set to `NaN`.
            #[inline]
            pub fn nan() -> Self {
                Self {
                    $(
                        $field: T::nan(),
                    )*
                }
            }
        }

        impl<T: Digit + PartialEq> $VectorN<T> {
            /// Returns a `boolean` `Vector``==` comparison elements of `self` and `rhs`.
            #[inline]
            pub fn cmpeq(&self, rhs: &Self) -> $VectorN<bool> {
                $VectorN::<bool>::new(
                    $(
                        self.$field.eq(&rhs.$field),
                    )*
                )
            }

            /// Returns a `boolean` `Vector``!=` comparison elements of `self` and `rhs`.
            #[inline]
            pub fn cmpne(&self, rhs: &Self) -> $VectorN<bool> {
                $VectorN::<bool>::new(
                    $(
                        self.$field.ne(&rhs.$field),
                    )*
                )
            }
        }

        impl<T: Digit + PartialOrd> $VectorN<T> {
            /// Returns a `boolean` `Vector``>=` comparison elements of `self` and `rhs`.
            #[inline]
            pub fn cmpge(&self, rhs: &Self) -> $VectorN<bool> {
                $VectorN::<bool>::new(
                    $(
                        self.$field.ge(&rhs.$field),
                    )*
                )
            }

            /// Returns a `boolean` `Vector``>` comparison elements of `self` and `rhs`.
            #[inline]
            pub fn cmpgt(&self, rhs: &Self) -> $VectorN<bool> {
                $VectorN::<bool>::new(
                    $(
                        self.$field.gt(&rhs.$field),
                    )*
                )
            }

            /// Returns a `boolean` `Vector``<=` comparison elements of `self` and `rhs`.
            #[inline]
            pub fn cmple(&self, rhs: &Self) -> $VectorN<bool> {
                $VectorN::<bool>::new(
                    $(
                        self.$field.le(&rhs.$field),
                    )*
                )
            }

            /// Returns a `boolean` `Vector``<` comparison elements of `self` and `rhs`.
            #[inline]
            pub fn cmplt(&self, rhs: &Self) -> $VectorN<bool> {
                $VectorN::<bool>::new(
                    $(
                        self.$field.lt(&rhs.$field),
                    )*
                )
            }
        }

        impl<T: DigitNum + Signed> $VectorN<T> {
            /// Returns a `Vector` with absolute values of `self`.
            #[inline]
            pub fn abs(self) -> Self {
                Self {
                    $(
                        $field: self.$field.abs(),
                    )*
                }
            }

            /// Returns a `Vector` with representing sign values of `self`.
            #[inline]
            pub fn signum(self) -> Self {
                Self {
                    $(
                        $field: self.$field.signum(),
                    )*
                }
            }
        }

        impl<T: DigitNum + Signed + PartialOrd> $VectorN<T> {
            /// Returns `true` if the absolute difference of all elements between `self` and `rhs` <= `max_abs_diff`.
            #[inline]
            pub fn abs_diff_eq(&self, rhs: &Self, max_abs_diff: T) -> bool {
                self.sub(rhs).abs().cmple(&Self::splat(max_abs_diff)).all()
            }
        }

        impl<T: DigitNum + PartialOrd> $VectorN<T> {
            /// Returns a `Vector` with minimum values of `self` and `min`.
            #[inline]
            pub fn min(self, min: Self) -> Self {
                Self {
                    $(
                        $field: clamp_min(self.$field, min.$field),
                    )*
                }
            }

            /// Returns a `Vector` with maximum values of `self` and `max`.
            #[inline]
            pub fn max(self, max: Self) -> Self {
                Self {
                    $(
                        $field: clamp_max(self.$field, max.$field),
                    )*
                }
            }

            /// Returns a `Vector` with clamp values of `self` between `min` and `max`.
            #[inline]
            pub fn clamp(self, min: Self, max: Self) -> Self {
                Self {
                    $(
                        $field: clamp(self.$field, min.$field, max.$field),
                    )*
                }
            }
        }

        impl<T: DigitFloat> $VectorN<T> {
            /// Returns a `Vector` with a length clamp between `min` and `max`.
            #[inline]
            pub fn clamp_length(self, min: T, max: T) -> Self {
                let length_sq = self.length_squared();

                if length_sq < min * min {
                    self.mul(length_sq.sqrt().recip() * min)
                } else if length_sq > max * max {
                    self.mul(length_sq.sqrt().recip() * max)
                } else {
                    self
                }
            }

            /// Returns a `Vector` with a length no more than `max`
            pub fn clamp_length_max(self, max: T) -> Self {
                let length_sq = self.length_squared();

                if length_sq > max * max {
                    self.mul(length_sq.sqrt().recip() * max)
                } else {
                    self
                }
            }

            /// Returns a `Vector` with a length no less than `min`
            pub fn clamp_length_min(self, min: T) -> Self {
                let length_sq = self.length_squared();

                if length_sq < min * min {
                    self.mul(length_sq.sqrt().recip() * min)
                } else {
                    self
                }
            }

            /// Returns a `Vector` with linear interpolation between `self` and `rhs` based on the value `s`.
            #[inline]
            pub fn lerp(self, rhs: Self, s: T) -> Self {
                self + (rhs.sub(self) * s)
            }

            /// Returns squared length of `self`.
            #[inline]
            pub fn length_squared(self) -> T {
                self.dot(self)
            }

            /// Returns length of `self`.
            #[inline]
            pub fn length(self) -> T {
                self.length_squared().sqrt()
            }

            /// Returns squared Euclidean distance of `self` and `rhs`.
            #[inline]
            pub fn distance_squared(self, rhs: Self) -> T {
                self.sub(rhs).length_squared()
            }

            /// Returns Euclidean distance of `self` and `rhs`.
            #[inline]
            pub fn distance(self, rhs: Self) -> T {
                self.sub(rhs).length()
            }

            /// Returns a `Vector` with the nearest integer to a number for values of `self`.
            #[inline]
            pub fn round(self) -> Self {
                Self {
                    $(
                        $field: self.$field.round(),
                    )*
                }
            }

            /// Returns a `Vector` with the largest integer less than or equal to a number for values of `self`.
            #[inline]
            pub fn floor(self) -> Self {
                Self {
                    $(
                        $field: self.$field.floor(),
                    )*
                }
            }

            /// Returns a `Vector` with containing the smallest integer greater than or equal to a number for values of `self`.
            #[inline]
            pub fn ceil(self) -> Self {
                Self {
                    $(
                        $field: self.$field.ceil(),
                    )*
                }
            }

            /// Returns a `Vector` with fractional values of `self`.
            #[inline]
            pub fn fract(self) -> Self {
                Self {
                    $(
                        $field: self.$field.fract(),
                    )*
                }
            }

            /// Returns a `Vector` with exponential function for values of `self`.
            #[inline]
            pub fn exp(self) -> Self {
                Self {
                    $(
                        $field: self.$field.exp(),
                    )*
                }
            }

            /// Returns a `Vector` with raised values of `self` to the power of `n`.
            #[inline]
            pub fn powf(self, n: T) -> Self {
                Self {
                    $(
                        $field: self.$field.powf(n),
                    )*
                }
            }

            /// Returns a `Vector` with reciprocaled values of `self`
            #[inline]
            pub fn recip(self) -> Self {
                Self {
                    $(
                        $field: self.$field.recip(),
                    )*
                }
            }

            /// Returns a `Vector` with normalized length of `self`.
            #[inline]
            pub fn normalize(self) -> Self {
                self.mul(self.length().recip())
            }

            /// Returns a `Vector` with projection of `self` onto `rhs`.
            #[inline]
            pub fn project_onto(self, rhs: Self) -> Self {
                rhs.mul(self.dot(rhs).mul(rhs.length_squared().recip()))
            }

            /// Returns a `Vector` with normalized projection of `self` onto `rhs`.
            #[inline]
            pub fn project_onto_normalized(self, rhs: Self) -> Self {
                rhs.mul(self.dot(rhs))
            }

            /// Returns a `Vector` with rejection of `self` from `rhs`.
            #[inline]
            pub fn reject_from(self, rhs: Self) -> Self {
                self.sub(self.project_onto(rhs))
            }

            /// Returns a `Vector` with normalized rejection of `self` from `rhs`.
            #[inline]
            pub fn reject_from_normalized(self, rhs: Self) -> Self {
                self.sub(self.project_onto_normalized(rhs))
            }

            /// Returns `true` if all values of `self` are finite.
            #[inline]
            pub fn is_finite(self) -> bool {
                $(
                    self.$field.is_finite()
                )&&+
            }

            /// Returns `true` if any values of `self` are `NaN`.
            #[inline]
            pub fn is_nan(self) -> bool {
                $(
                    self.$field.is_nan()
                )||+
            }
        }

        impl $VectorN<bool> {
            #[inline]
            pub const fn any(self) -> bool {
                $(
                    self.$field
                )||+
            }

            #[inline]
            pub const fn all(self) -> bool {
                $(
                    self.$field
                )&&+
            }
        }

        impl<T: DigitNum> Sum for $VectorN<T> {
            #[inline]
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::zero(), Self::add)
            }
        }

        impl<'a, T: DigitNum> Sum<&'a Self> for $VectorN<T> {
            #[inline]
            fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                iter.fold(Self::zero(), |a, &b| Self::add(a, b))
            }
        }

        impl<T: DigitNum> Product for $VectorN<T> {
            #[inline]
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::one(), Self::mul)
            }
        }

        impl<'a, T: DigitNum> Product<&'a Self> for $VectorN<T> {
            #[inline]
            fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                iter.fold(Self::one(), |a, &b| Self::mul(a, b))
            }
        }

        impl<T: DigitNum + Signed + Neg> Neg for $VectorN<T> {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self {
                Self {
                    $(
                        $field: self.$field.neg(),
                    )*
                }
            }
        }

        crate::impl_operator!(<T: DigitNum> Add<$VectorN<T>> for $VectorN<T> {
            fn add(lhs, rhs) -> $VectorN<T> { $VectorN::new($(lhs.$field + rhs.$field),+) }
        });

        crate::impl_operator!(<T: DigitNum> Add<T> for $VectorN<T> {
            fn add(lhs, rhs) -> $VectorN<T> { $VectorN::new($(lhs.$field + rhs),+) }
        });

        crate::impl_assign_operator!(<T: DigitNum> AddAssign<$VectorN<T>> for $VectorN<T> {
            fn add_assign(&mut self, rhs) { $(self.$field += rhs.$field);+ }
        });

        crate::impl_assign_operator!(<T: DigitNum> AddAssign<T> for $VectorN<T> {
            fn add_assign(&mut self, rhs) { $(self.$field += rhs);+ }
        });

        crate::impl_operator!(<T: DigitNum> Sub<$VectorN<T>> for $VectorN<T> {
            fn sub(lhs, rhs) -> $VectorN<T> { $VectorN::new($(lhs.$field - rhs.$field),+) }
        });

        crate::impl_operator!(<T: DigitNum> Sub<T> for $VectorN<T> {
            fn sub(lhs, rhs) -> $VectorN<T> { $VectorN::new($(lhs.$field - rhs),+) }
        });

        crate::impl_assign_operator!(<T: DigitNum> SubAssign<$VectorN<T>> for $VectorN<T> {
            fn sub_assign(&mut self, rhs) { $(self.$field -= rhs.$field);+ }
        });

        crate::impl_assign_operator!(<T: DigitNum> SubAssign<T> for $VectorN<T> {
            fn sub_assign(&mut self, rhs) { $(self.$field -= rhs);+ }
        });

        crate::impl_operator!(<T: DigitNum> Mul<$VectorN<T>> for $VectorN<T> {
            fn mul(lhs, rhs) -> $VectorN<T> { $VectorN::new($(lhs.$field * rhs.$field),+) }
        });

        crate::impl_operator!(<T: DigitNum> Mul<T> for $VectorN<T> {
            fn mul(lhs, rhs) -> $VectorN<T> { $VectorN::new($(lhs.$field * rhs),+) }
        });

        crate::impl_assign_operator!(<T: DigitNum> MulAssign<$VectorN<T>> for $VectorN<T> {
            fn mul_assign(&mut self, rhs) { $(self.$field *= rhs.$field);+ }
        });

        crate::impl_assign_operator!(<T: DigitNum> MulAssign<T> for $VectorN<T> {
            fn mul_assign(&mut self, rhs) { $(self.$field *= rhs);+ }
        });

        crate::impl_operator!(<T: DigitNum> Div<$VectorN<T>> for $VectorN<T> {
            fn div(lhs, rhs) -> $VectorN<T> { $VectorN::new($(lhs.$field / rhs.$field),+) }
        });

        crate::impl_operator!(<T: DigitNum> Div<T> for $VectorN<T> {
            fn div(lhs, rhs) -> $VectorN<T> { $VectorN::new($(lhs.$field / rhs),+) }
        });

        crate::impl_assign_operator!(<T: DigitNum> DivAssign<$VectorN<T>> for $VectorN<T> {
            fn div_assign(&mut self, rhs) { $(self.$field /= rhs.$field);+ }
        });

        crate::impl_assign_operator!(<T: DigitNum> DivAssign<T> for $VectorN<T> {
            fn div_assign(&mut self, rhs) { $(self.$field /= rhs);+ }
        });

        crate::impl_operator!(<T: DigitNum> Rem<$VectorN<T>> for $VectorN<T> {
            fn rem(lhs, rhs) -> $VectorN<T> { $VectorN::new($(lhs.$field % rhs.$field),+) }
        });

        crate::impl_operator!(<T: DigitNum> Rem<T> for $VectorN<T> {
            fn rem(lhs, rhs) -> $VectorN<T> { $VectorN::new($(lhs.$field % rhs),+) }
        });

        crate::impl_assign_operator!(<T: DigitNum> RemAssign<$VectorN<T>> for $VectorN<T> {
            fn rem_assign(&mut self, rhs) { $(self.$field %= rhs.$field);+ }
        });

        crate::impl_assign_operator!(<T: DigitNum> RemAssign<T> for $VectorN<T> {
            fn rem_assign(&mut self, rhs) { $(self.$field %= rhs);+ }
        });

        crate::impl_index_operators!($VectorN<T>, $n, T, usize);
        crate::impl_index_operators!($VectorN<T>, $n, [T], Range<usize>);
        crate::impl_index_operators!($VectorN<T>, $n, [T], RangeTo<usize>);
        crate::impl_index_operators!($VectorN<T>, $n, [T], RangeFrom<usize>);
        crate::impl_index_operators!($VectorN<T>, $n, [T], RangeFull);
    };
}

impl_vector!(
    /// A 2-dimensional vector
    => Vector2 { x, y }, 2
);
crate::impl_array_conversions!(Vector2<T: Digit> { x: 0, y: 1 }, 2);
crate::impl_tuple_conversions!(Vector2<T: Digit> { x, y }, (T, T));

impl<T: Digit> Vector2<T> {
    /// Returns a `Vector`, provides the `z` value.
    #[inline]
    pub fn extend(self, z: T) -> Vector3<T> {
        Vector3::new(self.x, self.y, z)
    }
}

impl<T: DigitNum + PartialOrd> Vector2<T> {
    /// Returns the horizontal minimum of `self`.
    #[inline]
    pub fn min_element(self) -> T {
        clamp_min(self.x, self.y)
    }

    /// Returns the horizontal maximum of `self`.
    #[inline]
    pub fn max_element(self) -> T {
        clamp_max(self.x, self.y)
    }
}

impl<T: DigitNum> Vector2<T> {
    /// Returns a `Vector` with pointing along the positive [`x`](Vector2::x).
    #[inline]
    pub fn unit_x() -> Self {
        Self::new(T::one(), T::zero())
    }

    /// Returns a `Vector` with pointing along the positive [`y`](Vector2::y).
    #[inline]
    pub fn unit_y() -> Self {
        Self::new(T::zero(), T::one())
    }
}

impl<T: DigitNum + Signed> Vector2<T> {
    /// Returns a `Vector` with pointing along the negative [`x`](Vector2::x).
    #[inline]
    pub fn unit_neg_x() -> Self {
        Self::new(-T::one(), T::zero())
    }

    /// Returns a `Vector` with pointing along the negative [`y`](Vector2::y).
    #[inline]
    pub fn unit_neg_y() -> Self {
        Self::new(T::zero(), -T::one())
    }
}

impl<T: Digit + fmt::Display> fmt::Display for Vector2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: Digit + fmt::Debug> fmt::Debug for Vector2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Vector2))
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

impl<T: Digit> From<Vector3<T>> for Vector2<T> {
    #[inline]
    fn from(v: Vector3<T>) -> Self {
        Self::new(v.x, v.y)
    }
}

impl<T: Digit> From<Vector4<T>> for Vector2<T> {
    #[inline]
    fn from(v: Vector4<T>) -> Self {
        Self::new(v.x, v.y)
    }
}

impl_vector!(
    /// A 3-dimensional vector
    => Vector3 { x, y, z }, 3);
crate::impl_array_conversions!(Vector3<T: Digit> { x: 0, y: 1 , z: 2 }, 3);
crate::impl_tuple_conversions!(Vector3<T: Digit> { x, y, z }, (T, T, T));

impl<T: Digit> Vector3<T> {
    /// Returns a `Vector`, dropping the `z` value.
    #[inline]
    pub fn truncate(self) -> Vector2<T> {
        Vector2::new(self.x, self.y)
    }

    /// Returns a `Vector`, provides the `w` value.
    #[inline]
    pub fn extend(self, w: T) -> Vector4<T> {
        Vector4::new(self.x, self.y, self.z, w)
    }
}

impl<T: DigitNum + PartialOrd> Vector3<T> {
    /// Returns the horizontal minimum of `self`.
    #[inline]
    pub fn min_element(self) -> T {
        clamp_min(self.x, clamp_min(self.y, self.z))
    }

    /// Returns the horizontal maximum of `self`.
    #[inline]
    pub fn max_element(self) -> T {
        clamp_max(self.x, clamp_max(self.y, self.z))
    }
}

impl<T: DigitNum> Vector3<T> {
    /// Returns a `Vector` with cross product of `self` and `rhs`.
    #[inline]
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - rhs.y * self.z,
            y: self.z * rhs.x - rhs.z * self.x,
            z: self.x * rhs.y - rhs.x * self.y,
        }
    }

    /// Returns a `Vector` with pointing along the positive [`x`](Vector3::x).
    #[inline]
    pub fn unit_x() -> Self {
        Self::new(T::one(), T::zero(), T::zero())
    }

    /// Returns a `Vector` with pointing along the positive [`y`](Vector3::y).
    #[inline]
    pub fn unit_y() -> Self {
        Self::new(T::zero(), T::one(), T::zero())
    }

    /// Returns a `Vector` with pointing along the positive [`z`](Vector3::z).
    #[inline]
    pub fn unit_z() -> Self {
        Self::new(T::zero(), T::zero(), T::one())
    }
}

impl<T: DigitNum + Signed> Vector3<T> {
    /// Returns a `Vector` with pointing along the negative [`x`](Vector3::x).
    #[inline]
    pub fn unit_neg_x() -> Self {
        Self::new(-T::one(), T::zero(), T::zero())
    }

    /// Returns a `Vector` with pointing along the negative [`y`](Vector3::y).
    #[inline]
    pub fn unit_neg_y() -> Self {
        Self::new(T::zero(), -T::one(), T::zero())
    }

    /// Returns a `Vector` with pointing along the negative [`z`](Vector3::z).
    #[inline]
    pub fn unit_neg_z() -> Self {
        Self::new(T::zero(), T::zero(), -T::one())
    }
}

impl<T: Digit + fmt::Display> fmt::Display for Vector3<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<T: Digit + fmt::Debug> fmt::Debug for Vector3<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Vector3))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

impl<T: Digit + Default> From<Vector2<T>> for Vector3<T> {
    #[inline]
    fn from(v: Vector2<T>) -> Self {
        Self::new(v.x, v.y, T::default())
    }
}

impl<T: Digit> From<Vector4<T>> for Vector3<T> {
    #[inline]
    fn from(v: Vector4<T>) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl_vector!(
    /// A 4-dimensional vector
    => Vector4 { x, y, z, w },
    4
);
crate::impl_array_conversions!(Vector4<T: Digit> { x: 0, y: 1 , z: 2, w: 3 }, 4);
crate::impl_tuple_conversions!(Vector4<T: Digit> { x, y, z, w }, (T, T, T, T));

impl<T: Digit> Vector4<T> {
    /// Returns a `Vector`, dropping the `w` value.
    #[inline]
    pub fn truncate(self) -> Vector3<T> {
        Vector3::new(self.x, self.y, self.w)
    }
}

impl<T: DigitNum + PartialOrd> Vector4<T> {
    /// Returns the horizontal minimum of `self`.
    #[inline]
    pub fn min_element(self) -> T {
        clamp_min(self.x, clamp_min(self.y, clamp_min(self.z, self.w)))
    }

    /// Returns the horizontal maximum of `self`.
    #[inline]
    pub fn max_element(self) -> T {
        clamp_max(self.x, clamp_max(self.y, clamp_max(self.z, self.w)))
    }
}

impl<T: DigitNum> Vector4<T> {
    /// Returns a `Vector` with pointing along the positive [`x`](Vector4::x).
    #[inline]
    pub fn unit_x() -> Self {
        Self::new(T::one(), T::zero(), T::zero(), T::zero())
    }

    /// Returns a `Vector` with pointing along the positive [`y`](Vector4::y).
    #[inline]
    pub fn unit_y() -> Self {
        Self::new(T::zero(), T::one(), T::zero(), T::zero())
    }

    /// Returns a `Vector` with pointing along the positive [`z`](Vector4::z).
    #[inline]
    pub fn unit_z() -> Self {
        Self::new(T::zero(), T::zero(), T::one(), T::zero())
    }

    /// Returns a `Vector` with pointing along the positive [`w`](Vector4::w).
    #[inline]
    pub fn unit_w() -> Self {
        Self::new(T::zero(), T::zero(), T::zero(), T::one())
    }
}

impl<T: DigitNum + Signed> Vector4<T> {
    /// Returns a `Vector` with pointing along the negative [`x`](Vector4::x).
    #[inline]
    pub fn unit_neg_x() -> Self {
        Self::new(-T::one(), T::zero(), T::zero(), T::zero())
    }

    /// Returns a `Vector` with pointing along the negative [`y`](Vector4::y).
    #[inline]
    pub fn unit_neg_y() -> Self {
        Self::new(T::zero(), -T::one(), T::zero(), T::zero())
    }

    /// Returns a `Vector` with pointing along the negative [`z`](Vector4::z).
    #[inline]
    pub fn unit_neg_z() -> Self {
        Self::new(T::zero(), T::zero(), -T::one(), T::zero())
    }

    /// Returns a `Vector` with pointing along the negative [`w`](Vector4::w).
    #[inline]
    pub fn unit_neg_w() -> Self {
        Self::new(T::zero(), T::zero(), T::zero(), -T::one())
    }
}

impl<T: Digit + fmt::Display> fmt::Display for Vector4<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl<T: Digit + fmt::Debug> fmt::Debug for Vector4<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!(Vector4))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .field(&self.w)
            .finish()
    }
}

impl<T: Digit + Default> From<Vector2<T>> for Vector4<T> {
    #[inline]
    fn from(v: Vector2<T>) -> Self {
        Self::new(v.x, v.y, T::default(), T::default())
    }
}

impl<T: Digit + Default> From<Vector3<T>> for Vector4<T> {
    #[inline]
    fn from(v: Vector3<T>) -> Self {
        Self::new(v.x, v.y, v.z, T::default())
    }
}
