#![allow(dead_code)]
use crate::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AlTM {
    data: [[f64; 4]; 4],
}

impl AlTM {
    pub fn new() -> AlTM {
        AlTM {
            data: [[0.0; 4]; 4],
        }
    }

    pub fn identity() -> AlTM {
        let mut tm = AlTM::new();
        tm.data[0][0] = 1.0;
        tm.data[1][1] = 1.0;
        tm.data[2][2] = 1.0;
        tm.data[3][3] = 1.0;
        tm
    }

    pub fn zero() -> AlTM {
        AlTM {
            data: [[0.0; 4]; 4],
        }
    }

    pub fn diagonal(d0: f64, d1: f64, d2: f64, d3: f64) -> AlTM {
        let mut tm = AlTM::zero();
        tm.data[0][0] = d0;
        tm.data[1][1] = d1;
        tm.data[2][2] = d2;
        tm.data[3][3] = d3;
        tm
    }

    pub fn scale(&mut self, scale_factor: f64) {
        unsafe { altm_scale(self as *mut AlTM, scale_factor) };
    }

    pub fn scale_nonp(&mut self, sx: f64, sy: f64, sz: f64) {
        unsafe { altm_scale_nonp(self as *mut AlTM, sx, sy, sz) };
    }

    pub fn translate(&mut self, tx: f64, ty: f64, tz: f64) {
        unsafe { altm_translate(self as *mut AlTM, tx, ty, tz) };
    }

    pub fn rotate_x(&mut self, angle: f64) {
        unsafe { altm_rotate_x(self as *mut AlTM, angle) };
    }

    pub fn rotate_y(&mut self, angle: f64) {
        unsafe { altm_rotate_y(self as *mut AlTM, angle) };
    }

    pub fn rotate_z(&mut self, angle: f64) {
        unsafe { altm_rotate_z(self as *mut AlTM, angle) };
    }

    pub fn rotate(&mut self, x: f64, y: f64, z: f64, angle: f64) {
        unsafe { altm_rotate(self as *mut AlTM, x, y, z, angle) };
    }

    pub fn get_tm(&self) -> [[f64; 4]; 4] {
        self.data
    }

    pub fn set_tm(&mut self, matrix: [[f64; 4]; 4]) -> Result<(), String> {
        let status = unsafe { altm_set_tm(self as *mut AlTM, matrix.as_ptr() as *const _) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn copy_from(&mut self, src: &AlTM) {
        unsafe { altm_copy(self as *mut AlTM, src as *const AlTM) };
    }

    pub fn set_value(&mut self, value: f64) {
        unsafe { altm_set_value(self as *mut AlTM, value) };
    }

    pub fn equal(&self, other: &AlTM) -> bool {
        unsafe { altm_equal(self as *const AlTM, other as *const AlTM) }
    }

    pub fn not_equal(&self, other: &AlTM) -> bool {
        unsafe { altm_not_equal(self as *const AlTM, other as *const AlTM) }
    }

    pub fn add(&mut self, tm1: &AlTM, tm2: &AlTM) {
        unsafe { altm_add(self as *mut AlTM, tm1 as *const AlTM, tm2 as *const AlTM) };
    }

    pub fn subtract(&mut self, tm1: &AlTM, tm2: &AlTM) {
        unsafe { altm_subtract(self as *mut AlTM, tm1 as *const AlTM, tm2 as *const AlTM) };
    }

    pub fn multiply_matrix(&mut self, tm1: &AlTM, tm2: &AlTM) {
        unsafe { altm_multiply_matrix(self as *mut AlTM, tm1 as *const AlTM, tm2 as *const AlTM) };
    }

    pub fn multiply_scalar(&mut self, tm: &AlTM, scalar: f64) {
        unsafe { altm_multiply_scalar(self as *mut AlTM, tm as *const AlTM, scalar) };
    }

    pub fn add_assign(&mut self, other: &AlTM) {
        unsafe { altm_add_assign(self as *mut AlTM, other as *const AlTM) };
    }

    pub fn subtract_assign(&mut self, other: &AlTM) {
        unsafe { altm_subtract_assign(self as *mut AlTM, other as *const AlTM) };
    }

    pub fn multiply_assign_matrix(&mut self, other: &AlTM) {
        unsafe { altm_multiply_assign_matrix(self as *mut AlTM, other as *const AlTM) };
    }

    pub fn multiply_assign_scalar(&mut self, scalar: f64) {
        unsafe { altm_multiply_assign_scalar(self as *mut AlTM, scalar) };
    }

    pub fn trans_point4(&self, point: &mut [f64; 4]) -> Result<(), String> {
        let status = unsafe { altm_trans_point4(self as *const AlTM, point.as_mut_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn trans_point4_ex(&self, in_point: [f64; 4]) -> Result<[f64; 4], String> {
        let mut out_point: [f64; 4] = [0.0; 4];
        let status = unsafe { altm_trans_point4_ex(self as *const AlTM, in_point.as_ptr(), out_point.as_mut_ptr()) };
        if status == statusCode::Success {
            Ok(out_point)
        } else {
            Err(status.to_string())
        }
    }

    pub fn trans_point3(&self, x: &mut f64, y: &mut f64, z: &mut f64, w: &mut f64) -> Result<(), String> {
        let status = unsafe { altm_trans_point3(self as *const AlTM, x, y, z, w) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn trans_point3_no_w(&self, x: &mut f64, y: &mut f64, z: &mut f64) -> Result<(), String> {
        let status = unsafe { altm_trans_point3_no_w(self as *const AlTM, x, y, z) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn trans_vector(&self, x: &mut f64, y: &mut f64, z: &mut f64) -> Result<(), String> {
        let status = unsafe { altm_trans_vector(self as *const AlTM, x, y, z) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn trans_normal(&self, x: &mut f64, y: &mut f64, z: &mut f64) -> Result<(), String> {
        let status = unsafe { altm_trans_normal(self as *const AlTM, x, y, z) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn decompose(&self) -> Result<([f64; 3], [f64; 3], [f64; 3], [f64; 3]), String> {
        let mut translation: [f64; 3] = [0.0; 3];
        let mut rotation: [f64; 3] = [0.0; 3];
        let mut scale: [f64; 3] = [0.0; 3];
        let mut shear: [f64; 3] = [0.0; 3];
        let status = unsafe {
            altm_decompose(self as *const AlTM, translation.as_mut_ptr(), rotation.as_mut_ptr(), scale.as_mut_ptr(), shear.as_mut_ptr())
        };
        if status == statusCode::Success {
            Ok((translation, rotation, scale, shear))
        } else {
            Err(status.to_string())
        }
    }

    pub fn inverse(&mut self, tm: &AlTM) {
        unsafe { altm_inverse(self as *mut AlTM, tm as *const AlTM) };
    }

    pub fn transpose(&mut self, tm: &AlTM) {
        unsafe { altm_transpose(self as *mut AlTM, tm as *const AlTM) };
    }

    pub fn get_element(&self, row: i32, col: i32) -> f64 {
        unsafe { altm_get_element(self as *const AlTM, row, col) }
    }

    pub fn set_element(&mut self, row: i32, col: i32, value: f64) {
        unsafe { altm_set_element(self as *mut AlTM, row, col, value) };
    }

    pub fn as_ptr(&self) -> *const AlTM {
        self as *const AlTM
    }

    pub fn as_mut_ptr(&mut self) -> *mut AlTM {
        self as *mut AlTM
    }
}

unsafe extern "C" {
    fn altm_identity(outTM: *mut AlTM);
    fn altm_zero(outTM: *mut AlTM);
    fn altm_diagonal(outTM: *mut AlTM, d0: f64, d1: f64, d2: f64, d3: f64);
    fn altm_scale(outTM: *mut AlTM, scaleFactor: f64);
    fn altm_scale_nonp(outTM: *mut AlTM, sx: f64, sy: f64, sz: f64);
    fn altm_translate(outTM: *mut AlTM, tx: f64, ty: f64, tz: f64);
    fn altm_rotate_x(outTM: *mut AlTM, angle: f64);
    fn altm_rotate_y(outTM: *mut AlTM, angle: f64);
    fn altm_rotate_z(outTM: *mut AlTM, angle: f64);
    fn altm_rotate(outTM: *mut AlTM, x: f64, y: f64, z: f64, angle: f64);
    fn altm_get_tm(tm: *const AlTM, outMatrix: *mut f64) -> statusCode;
    fn altm_set_tm(tm: *mut AlTM, matrix: *const f64) -> statusCode;
    fn altm_copy(outTM: *mut AlTM, srcTM: *const AlTM);
    fn altm_set_value(tm: *mut AlTM, value: f64);
    fn altm_equal(tm1: *const AlTM, tm2: *const AlTM) -> bool;
    fn altm_not_equal(tm1: *const AlTM, tm2: *const AlTM) -> bool;
    fn altm_add(outTM: *mut AlTM, tm1: *const AlTM, tm2: *const AlTM);
    fn altm_subtract(outTM: *mut AlTM, tm1: *const AlTM, tm2: *const AlTM);
    fn altm_multiply_matrix(outTM: *mut AlTM, tm1: *const AlTM, tm2: *const AlTM);
    fn altm_multiply_scalar(outTM: *mut AlTM, tm: *const AlTM, scalar: f64);
    fn altm_add_assign(tm: *mut AlTM, other: *const AlTM);
    fn altm_subtract_assign(tm: *mut AlTM, other: *const AlTM);
    fn altm_multiply_assign_matrix(tm: *mut AlTM, other: *const AlTM);
    fn altm_multiply_assign_scalar(tm: *mut AlTM, scalar: f64);
    fn altm_trans_point4(tm: *const AlTM, point: *mut f64) -> statusCode;
    fn altm_trans_point4_ex(tm: *const AlTM, inPoint: *const f64, outPoint: *mut f64) -> statusCode;
    fn altm_trans_point3(tm: *const AlTM, x: *mut f64, y: *mut f64, z: *mut f64, w: *mut f64) -> statusCode;
    fn altm_trans_point3_no_w(tm: *const AlTM, x: *mut f64, y: *mut f64, z: *mut f64) -> statusCode;
    fn altm_trans_vector(tm: *const AlTM, x: *mut f64, y: *mut f64, z: *mut f64) -> statusCode;
    fn altm_trans_normal(tm: *const AlTM, x: *mut f64, y: *mut f64, z: *mut f64) -> statusCode;
    fn altm_decompose(tm: *const AlTM, translation: *mut f64, rotation: *mut f64, scale: *mut f64, shear: *mut f64) -> statusCode;
    fn altm_inverse(outTM: *mut AlTM, tm: *const AlTM);
    fn altm_transpose(outTM: *mut AlTM, tm: *const AlTM);
    fn altm_get_element(tm: *const AlTM, row: i32, col: i32) -> f64;
    fn altm_set_element(tm: *mut AlTM, row: i32, col: i32, value: f64);
}
