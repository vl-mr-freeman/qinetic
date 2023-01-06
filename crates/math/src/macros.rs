macro_rules! fold_array {
    ($method:ident, $x:expr) => ($x);
    ($method:ident, $x:expr, $($y:expr),+) => ($x.$method(crate::fold_array!($method, $($y),+)))
}

pub(crate) use fold_array;

macro_rules! impl_operator {
    // Unary operator.
    (
        <
        $T:ident :
        $Constraint:ident >
        $Op:ident for
        $Lhs:ty { fn $op:ident($x:ident) -> $Output:ty { $body:expr } }
    ) => {
        impl<$T: $Constraint> $Op for $Lhs {
            type Output = $Output;

            #[inline]
            fn $op(self) -> $Output {
                let $x = self;
                $body
            }
        }

        impl<'a, $T: $Constraint> $Op for &'a $Lhs {
            type Output = $Output;

            #[inline]
            fn $op(self) -> $Output {
                let $x = self;
                $body
            }
        }
    };
    // Right operand is a scalar.
    (
        <
        $T:ident :
        $Constraint:ident >
        $Op:ident <
        $Rhs:ident > for
        $Lhs:ty { fn $op:ident($lhs:ident, $rhs:ident) -> $Output:ty { $body:expr } }
    ) => {
        impl<$T: $Constraint> $Op<$Rhs> for $Lhs {
            type Output = $Output;

            #[inline]
            fn $op(self, other: $Rhs) -> $Output {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }

        impl<'a, $T: $Constraint> $Op<$Rhs> for &'a $Lhs {
            type Output = $Output;

            #[inline]
            fn $op(self, other: $Rhs) -> $Output {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }
    };
    // Right operand is a compound type
    (
        <
        $T:ident :
        $Constraint:ident >
        $Op:ident <
        $Rhs:ty > for
        $Lhs:ty { fn $op:ident($lhs:ident, $rhs:ident) -> $Output:ty { $body:expr } }
    ) => {
        impl<$T: $Constraint> $Op<$Rhs> for $Lhs {
            type Output = $Output;

            #[inline]
            fn $op(self, other: $Rhs) -> $Output {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }

        impl<'a, $T: $Constraint> $Op<&'a $Rhs> for $Lhs {
            type Output = $Output;

            #[inline]
            fn $op(self, other: &'a $Rhs) -> $Output {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }

        impl<'a, $T: $Constraint> $Op<$Rhs> for &'a $Lhs {
            type Output = $Output;

            #[inline]
            fn $op(self, other: $Rhs) -> $Output {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }

        impl<'a, 'b, $T: $Constraint> $Op<&'a $Rhs> for &'b $Lhs {
            type Output = $Output;

            #[inline]
            fn $op(self, other: &'a $Rhs) -> $Output {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }
    };
    // Left operand is a scalar
    (
        $Op:ident <
        $Rhs:ident <
        $T:ident >> for
        $Lhs:ty { fn $op:ident($lhs:ident, $rhs:ident) -> $Output:ty { $body:expr } }
    ) => {
        impl $Op<$Rhs<$T>> for $Lhs {
            type Output = $Output;
            fn $op(self, other: $Rhs<$T>) -> $Output {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }

        impl<'a> $Op<&'a $Rhs<$T>> for $Lhs {
            type Output = $Output;
            fn $op(self, other: &'a $Rhs<$T>) -> $Output {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }
    };
}

pub(crate) use impl_operator;

macro_rules! impl_assign_operator {
    (<$T:ident: $Constraint:ident> $Op:ident<$Rhs:ty> for $Lhs:ty {
        fn $op:ident(&mut $lhs:ident, $rhs:ident) $body:block
    }) => {
        impl<$T: $Constraint + $Op<$T>> $Op<$Rhs> for $Lhs {
            fn $op(&mut $lhs, $rhs: $Rhs) $body
        }
    };
}

pub(crate) use impl_assign_operator;

macro_rules! impl_array_conversions {
    ($ArrayN:ident <$T:ident: $Constraint:ident> { $($field:ident : $index:expr),+ }, $n:expr) => {
        impl<$T: $Constraint> From<$ArrayN<$T>> for [$T; $n] {
            #[inline]
            fn from(v: $ArrayN<$T>) -> Self {
                match v { $ArrayN { $($field),+ } => [$($field),+] }
            }
        }

        impl<$T: $Constraint> AsRef<[$T; $n]> for $ArrayN<$T> {
            #[inline]
            fn as_ref(&self) -> &[$T; $n] {
                unsafe { std::mem::transmute(self) }
            }
        }

        impl<$T: $Constraint> AsMut<[$T; $n]> for $ArrayN<$T> {
            #[inline]
            fn as_mut(&mut self) -> &mut [$T; $n] {
                unsafe { std::mem::transmute(self) }
            }
        }

        impl<$T: $Constraint> From<[$T; $n]> for $ArrayN<$T> {
            #[inline]
            fn from(v: [$T; $n]) -> $ArrayN<$T> {
                // We need to use a clone here because we can't pattern match on arrays yet
                $ArrayN {
                    $(
                        $field: v[$index].clone()
                    ),+
                }
            }
        }

        impl<'a, $T: $Constraint> From<&'a [$T; $n]> for &'a $ArrayN<$T> {
            #[inline]
            fn from(v: &'a [$T; $n]) -> &'a $ArrayN<$T> {
                unsafe { std::mem::transmute(v) }
            }
        }

        impl<'a, $T: $Constraint> From<&'a mut [$T; $n]> for &'a mut $ArrayN<$T> {
            #[inline]
            fn from(v: &'a mut [$T; $n]) -> &'a mut $ArrayN<$T> {
                unsafe { std::mem::transmute(v) }
            }
        }
    };
}

pub(crate) use impl_array_conversions;

macro_rules! impl_matrix_array_conversions {
    ($ArrayN:ident <$T:ident: $Constraint:ident> { $($field:ident : $index:expr),+ }, $V:ident, $n:expr) => {
        impl<$T: $Constraint> From<$ArrayN<$T>> for [$V<$T>; $n] {
            #[inline]
            fn from(v: $ArrayN<$T>) -> Self {
                match v { $ArrayN { $($field),+ } => [$($field),+] }
            }
        }

        impl<$T: $Constraint> AsRef<[$V<$T>; $n]> for $ArrayN<$T> {
            #[inline]
            fn as_ref(&self) -> &[$V<$T>; $n] {
                unsafe { std::mem::transmute(self) }
            }
        }

        impl<$T: $Constraint> AsMut<[$V<$T>; $n]> for $ArrayN<$T> {
            #[inline]
            fn as_mut(&mut self) -> &mut [$V<$T>; $n] {
                unsafe { std::mem::transmute(self) }
            }
        }

        impl<$T: $Constraint> From<[$V<$T>; $n]> for $ArrayN<$T> {
            #[inline]
            fn from(v: [$V<$T>; $n]) -> $ArrayN<$T> {
                // We need to use a clone here because we can't pattern match on arrays yet
                $ArrayN {
                    $(
                        $field: v[$index].clone()
                    ),+
                }
            }
        }

        impl<'a, $T: $Constraint> From<&'a [$V<$T>; $n]> for &'a $ArrayN<$T> {
            #[inline]
            fn from(v: &'a [$V<$T>; $n]) -> &'a $ArrayN<$T> {
                unsafe { std::mem::transmute(v) }
            }
        }

        impl<'a, $T: $Constraint> From<&'a mut [$V<$T>; $n]> for &'a mut $ArrayN<$T> {
            #[inline]
            fn from(v: &'a mut [$V<$T>; $n]) -> &'a mut $ArrayN<$T> {
                unsafe { std::mem::transmute(v) }
            }
        }
    };
}

pub(crate) use impl_matrix_array_conversions;

macro_rules! impl_tuple_conversions {
    ($ArrayN:ident <$T:ident: $Constraint:ident> { $($field:ident),+ }, $Tuple:ty) => {
        impl<$T: $Constraint> From<$ArrayN<$T>> for $Tuple {
            #[inline]
            fn from(v: $ArrayN<$T>) -> Self {
                match v { $ArrayN { $($field),+ } => ($($field),+,) }
            }
        }

        impl<$T: $Constraint> AsRef<$Tuple> for $ArrayN<$T> {
            #[inline]
            fn as_ref(&self) -> &$Tuple {
                unsafe { std::mem::transmute(self) }
            }
        }

        impl<$T: $Constraint> AsMut<$Tuple> for $ArrayN<$T> {
            #[inline]
            fn as_mut(&mut self) -> &mut $Tuple {
                unsafe { std::mem::transmute(self) }
            }
        }

        impl<$T: $Constraint> From<$Tuple> for $ArrayN<$T> {
            #[inline]
            fn from(v: $Tuple) -> $ArrayN<$T> {
                match v { ($($field),+,) => $ArrayN { $($field),+ } }
            }
        }

        impl<'a, $T: $Constraint> From<&'a $Tuple> for &'a $ArrayN<$T> {
            #[inline]
            fn from(v: &'a $Tuple) -> &'a $ArrayN<$T> {
                unsafe { std::mem::transmute(v) }
            }
        }

        impl<'a, $T: $Constraint> From<&'a mut $Tuple> for &'a mut $ArrayN<$T> {
            #[inline]
            fn from(v: &'a mut $Tuple) -> &'a mut $ArrayN<$T> {
                unsafe { std::mem::transmute(v) }
            }
        }
    };
}

pub(crate) use impl_tuple_conversions;

macro_rules! impl_matrix_tuple_conversions {
    ($ArrayN:ident <$T:ident: $Constraint:ident> { $($field:ident),+ }, ( $($V:ident),+ )) => {
        impl<$T: $Constraint> From<$ArrayN<$T>> for ($($V<$T>),+) {
            #[inline]
            fn from(v: $ArrayN<$T>) -> Self {
                match v { $ArrayN { $($field),+ } => ($($field),+,) }
            }
        }

        impl<$T: $Constraint> AsRef<($($V<$T>),+)> for $ArrayN<$T> {
            #[inline]
            fn as_ref(&self) -> &($($V<$T>),+) {
                unsafe { std::mem::transmute(self) }
            }
        }

        impl<$T: $Constraint> AsMut<($($V<$T>),+)> for $ArrayN<$T> {
            #[inline]
            fn as_mut(&mut self) -> &mut ($($V<$T>),+) {
                unsafe { std::mem::transmute(self) }
            }
        }

        impl<$T: $Constraint> From<($($V<$T>),+)> for $ArrayN<$T> {
            #[inline]
            fn from(v: ($($V<$T>),+)) -> $ArrayN<$T> {
                match v { ($($field),+,) => $ArrayN { $($field),+ } }
            }
        }

        impl<'a, $T: $Constraint> From<&'a ($($V<$T>),+)> for &'a $ArrayN<$T> {
            #[inline]
            fn from(v: &'a ($($V<$T>),+)) -> &'a $ArrayN<$T> {
                unsafe { std::mem::transmute(v) }
            }
        }

        impl<'a, $T: $Constraint> From<&'a mut ($($V<$T>),+)> for &'a mut $ArrayN<$T> {
            #[inline]
            fn from(v: &'a mut ($($V<$T>),+)) -> &'a mut $ArrayN<$T> {
                unsafe { std::mem::transmute(v) }
            }
        }
    };
}

pub(crate) use impl_matrix_tuple_conversions;

macro_rules! impl_index_operators {
    ($VectorN:ident < $T:ident > , $n:expr, $Output:ty, $I:ty) => {
        impl<$T: Digit> Index<$I> for $VectorN<$T> {
            type Output = $Output;

            #[inline]
            fn index<'a>(&'a self, i: $I) -> &'a $Output {
                let v: &[$T; $n] = self.as_ref();
                &v[i]
            }
        }

        impl<$T: Digit> IndexMut<$I> for $VectorN<$T> {
            #[inline]
            fn index_mut<'a>(&'a mut self, i: $I) -> &'a mut $Output {
                let v: &mut [$T; $n] = self.as_mut();
                &mut v[i]
            }
        }
    };
}

pub(crate) use impl_index_operators;

macro_rules! impl_matrix_index_operators {
    ($ArrayN:ident < $T:ident : $Constraint:ident > , $n:expr, $Output:ty, $V:ident, $I:ty) => {
        impl<$T: $Constraint> Index<$I> for $ArrayN<$T> {
            type Output = $Output;

            #[inline]
            fn index<'a>(&'a self, i: $I) -> &'a $Output {
                let v: &[$V<$T>; $n] = self.as_ref();
                &v[i]
            }
        }

        impl<$T: $Constraint> IndexMut<$I> for $ArrayN<$T> {
            #[inline]
            fn index_mut<'a>(&'a mut self, i: $I) -> &'a mut $Output {
                let v: &mut [$V<$T>; $n] = self.as_mut();
                &mut v[i]
            }
        }
    };
}

pub(crate) use impl_matrix_index_operators;
