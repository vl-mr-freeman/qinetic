use crate::{quat::*, vec3::*, vec4::*};
use std::fmt;
use std::iter::{Product, Sum};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem, RemAssign, Sub,
    SubAssign,
};

/// A 4x4 column major matrix.
#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Mat4 {
    pub x_axis: Vec4,
    pub y_axis: Vec4,
    pub z_axis: Vec4,
    pub w_axis: Vec4,
}

impl Mat4 {
    /// All values of [`Mat4`] are zeroes.
    pub const ZERO: Self = Self::splat(0.0);

    /// All values of [`Mat4`] are positive ones.
    pub const ONE: Self = Self::splat(1.0);

    /// All values of [`Mat4`] are negative ones.
    pub const NEG_ONE: Self = Self::splat(-1.0);

    /// All values of [`Mat4`] are NaN.
    pub const NAN: Self = Self::splat(f32::NAN);

    /// All diagonal elements of [`Mat4`] are `1`, and all off-diagonal elements are `0`.
    pub const IDENTITY: Self = Self::from_cols(Vec4::X, Vec4::Y, Vec4::Z, Vec4::W);

    /// Returns a [`Mat4`] with given values.
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

    /// Returns a [`Mat4`] converted from 4x[`Vec4`].
    pub const fn from_cols(x_axis: Vec4, y_axis: Vec4, z_axis: Vec4, w_axis: Vec4) -> Self {
        Self {
            x_axis: x_axis,
            y_axis: y_axis,
            z_axis: z_axis,
            w_axis: w_axis,
        }
    }

    /// Returns a [`Mat4`] with all values set to `m`.
    #[inline]
    pub const fn splat(m: f32) -> Self {
        Self::from_cols(
            Vec4::splat(m),
            Vec4::splat(m),
            Vec4::splat(m),
            Vec4::splat(m),
        )
    }

    /// Returns a [`Mat4`] converted from array.
    #[inline]
    pub const fn from_array(a: [f32; 16]) -> Self {
        Self::new(
            a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11], a[12], a[13],
            a[14], a[15],
        )
    }

    /// Returns array converted from [`Mat4`].
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

    /// Returns a [`Mat4`] converted from slice.
    #[inline]
    pub const fn from_slice(s: &[f32]) -> Self {
        Self::new(
            s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7], s[8], s[9], s[10], s[11], s[12], s[13],
            s[14], s[15],
        )
    }

    /// Converts [`Mat4`] `self` to slice.
    #[inline]
    pub fn to_slice(self, s: &mut [f32]) {
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

    /// Returns a [`Mat4`] with its diagonal set to `diagonal` and all other entries set to 0.
    #[inline]
    pub fn from_diagonal(diagonal: Vec4) -> Self {
        Self::new(
            diagonal.x, 0.0, 0.0, 0.0, 0.0, diagonal.y, 0.0, 0.0, 0.0, 0.0, diagonal.z, 0.0, 0.0,
            0.0, 0.0, diagonal.w,
        )
    }

    /// Returns a [`Mat4`] with the `rotation`.
    #[inline]
    pub fn from_quat(rotation: Quat) -> Self {
        let (x_axis, y_axis, z_axis) = Self::quat_to_axes(rotation);
        Self::from_cols(x_axis, y_axis, z_axis, Vec4::W)
    }

    /// Returns a [`Mat4`] from `angle` (in radians) around the x axis.
    #[inline]
    pub fn from_rotation_x(angle: f32) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self::from_cols(
            Vec4::X,
            Vec4::new(0.0, cosa, sina, 0.0),
            Vec4::new(0.0, -sina, cosa, 0.0),
            Vec4::W,
        )
    }

    /// Returns a [`Mat4`] from `angle` (in radians) around the y axis.
    #[inline]
    pub fn from_rotation_y(angle: f32) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self::from_cols(
            Vec4::new(cosa, 0.0, -sina, 0.0),
            Vec4::Y,
            Vec4::new(sina, 0.0, cosa, 0.0),
            Vec4::W,
        )
    }

    /// Returns a [`Mat4`] from `angle` (in radians) around the z axis.
    #[inline]
    pub fn from_rotation_z(angle: f32) -> Self {
        let (sina, cosa) = angle.sin_cos();
        Self::from_cols(
            Vec4::new(cosa, sina, 0.0, 0.0),
            Vec4::new(-sina, cosa, 0.0, 0.0),
            Vec4::Z,
            Vec4::W,
        )
    }

    /// Returns a [`Mat4`] with the `translation`.
    #[inline]
    pub fn from_translation(translation: Vec3) -> Self {
        Self::from_cols(
            Vec4::X,
            Vec4::Y,
            Vec4::Z,
            Vec4::new(translation.x, translation.y, translation.z, 1.0),
        )
    }

    /// Returns a [`Mat4`] with the `scale`, `rotation` and `translation`.
    #[inline]
    pub fn from_scale_rotation_translation(scale: Vec3, rotation: Quat, translation: Vec3) -> Self {
        let (x_axis, y_axis, z_axis) = Self::quat_to_axes(rotation);
        Self::from_cols(
            x_axis.mul(scale.x),
            y_axis.mul(scale.y),
            z_axis.mul(scale.z),
            Vec4::new(translation.x, translation.y, translation.z, 1.0),
        )
    }

    /// Returns a [`Mat4`] with the `rotation` and `translation`.
    #[inline]
    pub fn from_rotation_translation(rotation: Quat, translation: Vec3) -> Self {
        let (x_axis, y_axis, z_axis) = Self::quat_to_axes(rotation);
        Self::from_cols(
            x_axis,
            y_axis,
            z_axis,
            Vec4::new(translation.x, translation.y, translation.z, 1.0),
        )
    }

    /// Returns a [`Mat4`] with rotation around a normalized rotation `axis` of `angle` (in radians).
    ///
    /// `axis` must be normalized
    #[inline]
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        assert!(axis.is_normalized());

        let (sin, cos) = angle.sin_cos();
        let axis_sin = axis.mul(sin);
        let axis_sq = axis.mul(axis);
        let omc = 1.0 - cos;
        let xyomc = axis.x * axis.y * omc;
        let xzomc = axis.x * axis.z * omc;
        let yzomc = axis.y * axis.z * omc;
        Self::from_cols(
            Vec4::new(
                axis_sq.x * omc + cos,
                xyomc + axis_sin.z,
                xzomc - axis_sin.y,
                0.0,
            ),
            Vec4::new(
                xyomc - axis_sin.z,
                axis_sq.y * omc + cos,
                yzomc + axis_sin.x,
                0.0,
            ),
            Vec4::new(
                xzomc + axis_sin.y,
                yzomc - axis_sin.x,
                axis_sq.z * omc + cos,
                0.0,
            ),
            Vec4::W,
        )
    }

    /// Returns a [`Mat4`] with non-uniform `scale`.
    #[inline]
    pub fn from_scale(scale: Vec3) -> Self {
        assert!(scale.cmpne(Vec3::ZERO).any());

        Self::from_cols(
            Vec4::new(scale.x, 0.0, 0.0, 0.0),
            Vec4::new(0.0, scale.y, 0.0, 0.0),
            Vec4::new(0.0, 0.0, scale.z, 0.0),
            Vec4::W,
        )
    }

    /// Returns a [`Vec4`] with `self` column for the `index`.
    #[inline]
    pub fn col(&self, index: usize) -> Vec4 {
        match index {
            0 => self.x_axis,
            1 => self.y_axis,
            2 => self.z_axis,
            3 => self.w_axis,
            _ => panic!("index out of bounds"),
        }
    }

    /// Returns a [`Vec4`] with `self` row for the `index`.
    #[inline]
    pub fn row(&self, index: usize) -> Vec4 {
        match index {
            0 => Vec4::new(self.x_axis.x, self.y_axis.x, self.z_axis.x, self.w_axis.x),
            1 => Vec4::new(self.x_axis.y, self.y_axis.y, self.z_axis.y, self.w_axis.y),
            2 => Vec4::new(self.x_axis.z, self.y_axis.z, self.z_axis.z, self.w_axis.z),
            3 => Vec4::new(self.x_axis.w, self.y_axis.w, self.z_axis.w, self.w_axis.w),
            _ => panic!("index out of bounds"),
        }
    }

    /// Returns a [`Mat4`] with left-handed view using camera position, an up direction, and a facing direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    #[inline]
    pub fn look_to_lh(eye: Vec3, dir: Vec3, up: Vec3) -> Self {
        Self::look_to_rh(eye, -dir, up)
    }

    /// Returns a [`Mat4`] with right-handed view using camera position, an up direction, and a facing direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    #[inline]
    pub fn look_to_rh(eye: Vec3, dir: Vec3, up: Vec3) -> Self {
        let f = dir.normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(f);

        Self::from_cols(
            Vec4::new(s.x, u.x, -f.x, 0.0),
            Vec4::new(s.y, u.y, -f.y, 0.0),
            Vec4::new(s.z, u.z, -f.z, 0.0),
            Vec4::new(-eye.dot(s), -eye.dot(u), eye.dot(f), 1.0),
        )
    }

    /// Returns a [`Mat4`] with left-handed view matrix using a camera position, an up direction, and a focal point.
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    ///
    /// `up` must be normalized.
    #[inline]
    pub fn look_at_lh(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        assert!(up.is_normalized());
        Self::look_to_lh(eye, center.sub(eye), up)
    }

    /// Returns a [`Mat4`] with right-handed view matrix using a camera position, an up direction, and a focal point.
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    ///
    /// `up` must be normalized.
    #[inline]
    pub fn look_at_rh(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        assert!(up.is_normalized());
        Self::look_to_rh(eye, center.sub(eye), up)
    }

    /// Returns a [`Mat4`] iwth left-handed perspective projection with `[0,1]` depth range.
    ///
    /// `z_near` must be greater that 0.
    /// `z_far` must be greater that 0.
    #[inline]
    pub fn perspective_lh(fov_y_radians: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
        assert!(z_near > 0.0 && z_far > 0.0);
        let (sin_fov, cos_fov) = (0.5 * fov_y_radians).sin_cos();
        let h = cos_fov / sin_fov;
        let w = h / aspect_ratio;
        let r = z_far / (z_far - z_near);
        Self::from_cols(
            Vec4::new(w, 0.0, 0.0, 0.0),
            Vec4::new(0.0, h, 0.0, 0.0),
            Vec4::new(0.0, 0.0, r, 1.0),
            Vec4::new(0.0, 0.0, -r * z_near, 0.0),
        )
    }

    /// Returns a [`Mat4`] with right-handed perspective projection with `[0,1]` depth range.
    ///
    /// `z_near` must be greater that 0.
    /// `z_far` must be greater that 0.
    #[inline]
    pub fn perspective_rh(fov_y_radians: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
        assert!(z_near > 0.0 && z_far > 0.0);
        let (sin_fov, cos_fov) = (0.5 * fov_y_radians).sin_cos();
        let h = cos_fov / sin_fov;
        let w = h / aspect_ratio;
        let r = z_far / (z_near - z_far);
        Self::from_cols(
            Vec4::new(w, 0.0, 0.0, 0.0),
            Vec4::new(0.0, h, 0.0, 0.0),
            Vec4::new(0.0, 0.0, r, -1.0),
            Vec4::new(0.0, 0.0, r * z_near, 0.0),
        )
    }

    /// Returns a [`Mat4`] with an infinite left-handed perspective projection with `[0,1]` depth range.
    ///
    /// `z_near` must be greater that 0.
    #[inline]
    pub fn perspective_infinite_lh(fov_y_radians: f32, aspect_ratio: f32, z_near: f32) -> Self {
        assert!(z_near > 0.0);
        let (sin_fov, cos_fov) = (0.5 * fov_y_radians).sin_cos();
        let h = cos_fov / sin_fov;
        let w = h / aspect_ratio;
        Self::from_cols(
            Vec4::new(w, 0.0, 0.0, 0.0),
            Vec4::new(0.0, h, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 1.0),
            Vec4::new(0.0, 0.0, -z_near, 0.0),
        )
    }

    /// Returns a [`Mat4`] with an infinite left-handed perspective projection with `[0,1]` depth range.
    ///
    /// `z_near` must be greater that 0.
    #[inline]
    pub fn perspective_infinite_reverse_lh(
        fov_y_radians: f32,
        aspect_ratio: f32,
        z_near: f32,
    ) -> Self {
        assert!(z_near > 0.0);
        let (sin_fov, cos_fov) = (0.5 * fov_y_radians).sin_cos();
        let h = cos_fov / sin_fov;
        let w = h / aspect_ratio;
        Self::from_cols(
            Vec4::new(w, 0.0, 0.0, 0.0),
            Vec4::new(0.0, h, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
            Vec4::new(0.0, 0.0, z_near, 0.0),
        )
    }

    /// Returns a [`Mat4`] with an infinite right-handed perspective projection with `[0,1]` depth range.
    ///
    /// `z_near` must be greater that 0.
    #[inline]
    pub fn perspective_infinite_rh(fov_y_radians: f32, aspect_ratio: f32, z_near: f32) -> Self {
        assert!(z_near > 0.0);
        let f = 1.0 / (0.5 * fov_y_radians).tan();
        Self::from_cols(
            Vec4::new(f / aspect_ratio, 0.0, 0.0, 0.0),
            Vec4::new(0.0, f, 0.0, 0.0),
            Vec4::new(0.0, 0.0, -1.0, -1.0),
            Vec4::new(0.0, 0.0, -z_near, 0.0),
        )
    }

    /// Returns a [`Mat4`] with an infinite reverse right-handed perspective projection with `[0,1]` depth range.
    ///
    /// `z_near` must be greater that 0.
    #[inline]
    pub fn perspective_infinite_reverse_rh(
        fov_y_radians: f32,
        aspect_ratio: f32,
        z_near: f32,
    ) -> Self {
        assert!(z_near > 0.0);
        let f = 1.0 / (0.5 * fov_y_radians).tan();
        Self::from_cols(
            Vec4::new(f / aspect_ratio, 0.0, 0.0, 0.0),
            Vec4::new(0.0, f, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, -1.0),
            Vec4::new(0.0, 0.0, z_near, 0.0),
        )
    }

    /// Returns a [`Mat4`] with left-handed orthographic projection with `[0,1]` depth range.
    #[inline]
    pub fn orthographic_lh(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Self {
        let rcp_width = 1.0 / (right - left);
        let rcp_height = 1.0 / (top - bottom);
        let r = 1.0 / (far - near);
        Self::from_cols(
            Vec4::new(rcp_width + rcp_width, 0.0, 0.0, 0.0),
            Vec4::new(0.0, rcp_height + rcp_height, 0.0, 0.0),
            Vec4::new(0.0, 0.0, r, 0.0),
            Vec4::new(
                -(left + right) * rcp_width,
                -(top + bottom) * rcp_height,
                -r * near,
                1.0,
            ),
        )
    }

    /// Returns a [`Mat4`] with right-handed orthographic projection with `[0,1]` depth range.
    #[inline]
    pub fn orthographic_rh(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Self {
        let rcp_width = 1.0 / (right - left);
        let rcp_height = 1.0 / (top - bottom);
        let r = 1.0 / (near - far);
        Self::from_cols(
            Vec4::new(rcp_width + rcp_width, 0.0, 0.0, 0.0),
            Vec4::new(0.0, rcp_height + rcp_height, 0.0, 0.0),
            Vec4::new(0.0, 0.0, r, 0.0),
            Vec4::new(
                -(left + right) * rcp_width,
                -(top + bottom) * rcp_height,
                r * near,
                1.0,
            ),
        )
    }

    /// Transforms the given 3D vector as a point, applying perspective correction.
    ///
    /// This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is `1.0`.
    /// The perspective divide is performed meaning the resulting 3D vector is divided by `w`.
    ///
    /// This method assumes that `self` contains a projective transform.
    #[inline]
    pub fn project_point3(&self, rhs: Vec3) -> Vec3 {
        let mut res = self.x_axis.mul(rhs.x);
        res = self.y_axis.mul(rhs.y).add(res);
        res = self.z_axis.mul(rhs.z).add(res);
        res = self.w_axis.add(res);
        res = res.mul(res.recip());
        res.into()
    }

    /// Transforms the given 3D vector as a point.
    ///
    /// This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is `1.0`.
    ///
    /// This method assumes that `self` contains a valid affine transform. It does not perform
    /// a persective divide, if `self` contains a perspective transform, or if you are unsure,
    /// the [`Self::project_point3()`] method should be used instead.
    ///
    /// 3d row of `self` must be (0, 0, 0, 1).
    #[inline]
    pub fn transform_point3(&self, rhs: Vec3) -> Vec3 {
        assert!(self.row(3).abs_diff_eq(Vec4::W, 1e-6));
        let mut res = self.x_axis.mul(rhs.x);
        res = self.y_axis.mul(rhs.y).add(res);
        res = self.z_axis.mul(rhs.z).add(res);
        res = self.w_axis.add(res);
        res.into()
    }

    /// Transforms the give 3D vector as a direction.
    ///
    /// This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is `0.0`.
    ///
    /// This method assumes that `self` contains a valid affine transform.
    ///
    /// 3d row of `self` must be (0, 0, 0, 1).
    #[inline]
    pub fn transform_vector3(&self, rhs: Vec3) -> Vec3 {
        assert!(self.row(3).abs_diff_eq(Vec4::W, 1e-6));
        let mut res = self.x_axis.mul(rhs.x);
        res = self.y_axis.mul(rhs.y).add(res);
        res = self.z_axis.mul(rhs.z).add(res);
        res.into()
    }

    /// Returns a [`Mat4`] with transpose from `self`.
    #[inline]
    pub fn transpose(&self) -> Self {
        Self::from_cols(
            Vec4::new(self.x_axis.x, self.y_axis.x, self.z_axis.x, self.w_axis.x),
            Vec4::new(self.x_axis.y, self.y_axis.y, self.z_axis.y, self.w_axis.y),
            Vec4::new(self.x_axis.z, self.y_axis.z, self.z_axis.z, self.w_axis.z),
            Vec4::new(self.x_axis.w, self.y_axis.w, self.z_axis.w, self.w_axis.w),
        )
    }

    /// Returns the determinant of `self`.
    #[inline]
    pub fn determinant(&self) -> f32 {
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

    /// Returns a [`Mat4`] with inverse of `self`.
    #[inline]
    pub fn inverse(&self) -> Self {
        let (m00, m01, m02, m03) = self.x_axis.into();
        let (m10, m11, m12, m13) = self.y_axis.into();
        let (m20, m21, m22, m23) = self.z_axis.into();
        let (m30, m31, m32, m33) = self.w_axis.into();

        let coef00 = m22 * m33 - m32 * m23;
        let coef02 = m12 * m33 - m32 * m13;
        let coef03 = m12 * m23 - m22 * m13;

        let coef04 = m21 * m33 - m31 * m23;
        let coef06 = m11 * m33 - m31 * m13;
        let coef07 = m11 * m23 - m21 * m13;

        let coef08 = m21 * m32 - m31 * m22;
        let coef10 = m11 * m32 - m31 * m12;
        let coef11 = m11 * m22 - m21 * m12;

        let coef12 = m20 * m33 - m30 * m23;
        let coef14 = m10 * m33 - m30 * m13;
        let coef15 = m10 * m23 - m20 * m13;

        let coef16 = m20 * m32 - m30 * m22;
        let coef18 = m10 * m32 - m30 * m12;
        let coef19 = m10 * m22 - m20 * m12;

        let coef20 = m20 * m31 - m30 * m21;
        let coef22 = m10 * m31 - m30 * m11;
        let coef23 = m10 * m21 - m20 * m11;

        let fac0 = Vec4::new(coef00, coef00, coef02, coef03);
        let fac1 = Vec4::new(coef04, coef04, coef06, coef07);
        let fac2 = Vec4::new(coef08, coef08, coef10, coef11);
        let fac3 = Vec4::new(coef12, coef12, coef14, coef15);
        let fac4 = Vec4::new(coef16, coef16, coef18, coef19);
        let fac5 = Vec4::new(coef20, coef20, coef22, coef23);

        let vec0 = Vec4::new(m10, m00, m00, m00);
        let vec1 = Vec4::new(m11, m01, m01, m01);
        let vec2 = Vec4::new(m12, m02, m02, m02);
        let vec3 = Vec4::new(m13, m03, m03, m03);

        let inv0 = vec1.mul(fac0).sub(vec2.mul(fac1)).add(vec3.mul(fac2));
        let inv1 = vec0.mul(fac0).sub(vec2.mul(fac3)).add(vec3.mul(fac4));
        let inv2 = vec0.mul(fac1).sub(vec1.mul(fac3)).add(vec3.mul(fac5));
        let inv3 = vec0.mul(fac2).sub(vec1.mul(fac4)).add(vec2.mul(fac5));

        let sign_a = Vec4::new(1.0, -1.0, 1.0, -1.0);
        let sign_b = Vec4::new(-1.0, 1.0, -1.0, 1.0);

        let inverse = Self::from_cols(
            inv0.mul(sign_a),
            inv1.mul(sign_b),
            inv2.mul(sign_a),
            inv3.mul(sign_b),
        );

        let col0 = Vec4::new(
            inverse.x_axis.x,
            inverse.y_axis.x,
            inverse.z_axis.x,
            inverse.w_axis.x,
        );

        let dot0 = self.x_axis.mul(col0);
        let dot1 = dot0.x + dot0.y + dot0.z + dot0.w;

        assert!(dot1 != 0.0);

        let rcp_det = dot1.recip();
        inverse.mul(rcp_det)
    }

    /// Returns `true` if all elements of `self` are finite.
    #[inline]
    pub fn is_finite(self) -> bool {
        self.x_axis.is_finite()
            && self.y_axis.is_finite()
            && self.z_axis.is_finite()
            && self.w_axis.is_finite()
    }

    /// Returns `true` if any elements of `self` are `NaN`.
    #[inline]
    pub fn is_nan(self) -> bool {
        self.x_axis.is_nan() || self.y_axis.is_nan() || self.z_axis.is_nan() || self.w_axis.is_nan()
    }

    #[inline]
    fn quat_to_axes(rotation: Quat) -> (Vec4, Vec4, Vec4) {
        assert!(rotation.is_normalized());

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

        let x_axis = Vec4::new(1.0 - (yy + zz), xy + wz, xz - wy, 0.0);
        let y_axis = Vec4::new(xy - wz, 1.0 - (xx + zz), yz + wx, 0.0);
        let z_axis = Vec4::new(xz + wy, yz - wx, 1.0 - (xx + yy), 0.0);
        (x_axis, y_axis, z_axis)
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
