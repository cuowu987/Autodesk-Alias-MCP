#![allow(non_camel_case_types)]
use base_geometry_lib::*;

use crate::*;
#[repr(C)]
pub struct AlConstructionPlane_ptr {
    _private: [u8; 0],
}

#[derive(Debug)]
pub struct AlConstructionPlane {
    pub ptr: *mut AlConstructionPlane_ptr,
}

impl Plane_Trait for AlConstructionPlane {
    fn origin(&self) -> RU_3dPoint {
        self.origin()
    }
    fn axis(&self) -> (RU_3dVector, RU_3dVector, RU_3dVector) {
        self.axes()
    }
}

pub trait Plane_Trait_ToConstructionPlane {
    fn to_construction_plane(&self) -> Result<AlConstructionPlane, String>;
}
impl<T: Plane_Trait> Plane_Trait_ToConstructionPlane for T {
    fn to_construction_plane(&self) -> Result<AlConstructionPlane, String> {
        let pos = self.origin();
        let axis = self.axis();
        AlConstructionPlane::new_create_xy_axis(pos, axis.0, axis.1)
    }
}

impl Drop for AlConstructionPlane {
    fn drop(&mut self) {
        unsafe {
            alconstructionplane_destroy(self.ptr);
            self.ptr = std::ptr::null_mut();
        }
    }
}

impl AlConstructionPlane {
    pub fn new() -> Self {
        let ptr = unsafe { alconstructionplane_new() };
        AlConstructionPlane { ptr }
    }

    pub fn create_3points(
        &mut self,
        p1: &AlPoint,
        p2: &AlPoint,
        p3: &AlPoint,
    ) -> Result<(), String> {
        let status =
            unsafe { alconstructionplane_create_3points(self.ptr, p1.ptr, p2.ptr, p3.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_2points_normal(
        &mut self,
        first: &AlPoint,
        second: &AlPoint,
        normal: &RU_3dVector,
    ) -> Result<(), String> {
        let status = unsafe {
            alconstructionplane_create_2points_normal(self.ptr, first.ptr, second.ptr, &normal.x)
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_2points_normal_1(
        &mut self,
        first: RU_3dPoint,
        second: RU_3dPoint,
        normal: RU_3dVector,
    ) -> Result<(), String> {
        let status = unsafe {
            alconstructionplane_create_2points_normal_1(
                self.ptr,
                &first.x,
                &second.x,
                &normal.x,
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }
    pub fn new_create_xy_axis(
        origin: RU_3dPoint,
        x_axis: RU_3dVector,
        y_axis: RU_3dVector,
    ) -> Result<Self, String> {
        let two_point = origin + x_axis;
        let normal = x_axis ^ y_axis;
        let mut plane = Self::new();
        match plane.create_2points_normal_1(
            origin,
            two_point,
            normal,
        ) {
            Ok(_) => Ok(plane),
            Err(e) => Err(e),
        }
    }
    pub fn new_create_yz_axis(origin: RU_3dPoint,y_axis: RU_3dVector, normal: RU_3dVector) -> Result<Self, String> {
        let two_point = origin + y_axis;
        let mut plane = Self::new();
        match plane.create_2points_normal_1(
            origin,
            two_point,
            normal,
        ) {
            Ok(_) => Ok(plane),
            Err(e) => Err(e),
        }
    }

    pub fn first(&self) -> Option<AlPoint> {
        let ptr = unsafe { alconstructionplane_first(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlPoint { ptr })
        }
    }

    pub fn second(&self) -> Option<AlPoint> {
        let ptr = unsafe { alconstructionplane_second(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlPoint { ptr })
        }
    }

    pub fn third(&self) -> Option<AlPoint> {
        let ptr = unsafe { alconstructionplane_third(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlPoint { ptr })
        }
    }

    pub fn scale(&self) -> Result<(f64, f64), String> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let status = unsafe { alconstructionplane_scale(self.ptr, &mut x, &mut y) };
        if status == statusCode::Success {
            Ok((x, y))
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_scale(&self, x: f64, y: f64) -> Result<(), String> {
        let status = unsafe { alconstructionplane_set_scale(self.ptr, x, y) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn rotate(&self) -> Result<(f64, f64, f64), String> {
        let mut rx: f64 = 0.0;
        let mut ry: f64 = 0.0;
        let mut rz: f64 = 0.0;
        let status = unsafe { alconstructionplane_rotate(self.ptr, &mut rx, &mut ry, &mut rz) };
        if status == statusCode::Success {
            Ok((rx, ry, rz))
        } else {
            Err(status.to_string())
        }
    }

    pub fn translate(&self) -> Result<(f64, f64, f64), String> {
        let mut tx: f64 = 0.0;
        let mut ty: f64 = 0.0;
        let mut tz: f64 = 0.0;
        let status = unsafe { alconstructionplane_translate(self.ptr, &mut tx, &mut ty, &mut tz) };
        if status == statusCode::Success {
            Ok((tx, ty, tz))
        } else {
            Err(status.to_string())
        }
    }

    pub fn transformation_matrix(&self) -> Result<[[f64; 4]; 4], String> {
        let mut matrix: [[f64; 4]; 4] = [[0.0; 4]; 4];
        let status = unsafe {
            alconstructionplane_transformation_matrix(self.ptr, matrix.as_mut_ptr() as *mut _)
        };
        if status == statusCode::Success {
            Ok(matrix)
        } else {
            Err(status.to_string())
        }
    }

    pub fn origin(&self) -> RU_3dPoint {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;
        let status = unsafe { alconstructionplane_origin(self.ptr, &mut x, &mut y, &mut z) };
        if status == statusCode::Success {
            RU_3dPoint::new(x, y, z)
        } else {
            panic!("get origin failed");
        }
    }

    pub fn set_origin(&self, origin: RU_3dPoint) -> Result<(), String> {
        let status = unsafe { alconstructionplane_set_origin(self.ptr, origin.x, origin.y, origin.z) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn axes(&self) -> (RU_3dVector, RU_3dVector, RU_3dVector) {
        let mut x_axis: RU_3dVector = RU_3dVector::default();
        let mut y_axis: RU_3dVector = RU_3dVector::default();
        let mut z_axis: RU_3dVector = RU_3dVector::default();
        let status = unsafe {
            alconstructionplane_axes(
                self.ptr,
                &mut x_axis.x,
                &mut y_axis.x,
                &mut z_axis.x,
            )
        };
        if status == statusCode::Success {
            (x_axis, y_axis, z_axis)
        } else {
            panic!("get axes failed");
        }
    }

    pub fn normal(&self) -> RU_3dVector {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;
        let status = unsafe { alconstructionplane_normal(self.ptr, &mut x, &mut y, &mut z) };
        if status == statusCode::Success {
            RU_3dVector::new(x, y, z)
        } else {
            panic!("get normal failed");
        }
    }

    pub fn equation(&self) -> Result<(f64, f64, f64, f64), String> {
        let mut a: f64 = 0.0;
        let mut b: f64 = 0.0;
        let mut c: f64 = 0.0;
        let mut d: f64 = 0.0;
        let status =
            unsafe { alconstructionplane_equation(self.ptr, &mut a, &mut b, &mut c, &mut d) };
        if status == statusCode::Success {
            Ok((a, b, c, d))
        } else {
            Err(status.to_string())
        }
    }

    pub fn canvas(&self) -> Option<AlCanvas> {
        let ptr = unsafe { alconstructionplane_canvas(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlCanvas { ptr })
        }
    }
}

impl AlObjectMethods for AlConstructionPlane {
    fn as_object_ptr(&self) -> *mut AlObject_ptr {
        self.ptr as *mut AlObject_ptr
    }
}

unsafe extern "C" {
    fn alconstructionplane_new() -> *mut AlConstructionPlane_ptr;
    fn alconstructionplane_destroy(plane: *mut AlConstructionPlane_ptr);

    fn alconstructionplane_create_3points(
        plane: *mut AlConstructionPlane_ptr,
        p1: *mut AlPoint_ptr,
        p2: *mut AlPoint_ptr,
        p3: *mut AlPoint_ptr,
    ) -> statusCode;
    fn alconstructionplane_create_2points_normal(
        plane: *mut AlConstructionPlane_ptr,
        p1: *mut AlPoint_ptr,
        p2: *mut AlPoint_ptr,
        normal: *const f64,
    ) -> statusCode;
    fn alconstructionplane_create_2points_normal_1(
        plane: *mut AlConstructionPlane_ptr,
        p1: *const f64,
        p2: *const f64,
        p3: *const f64,
    ) -> statusCode;

    fn alconstructionplane_first(plane: *mut AlConstructionPlane_ptr) -> *mut AlPoint_ptr;
    fn alconstructionplane_second(plane: *mut AlConstructionPlane_ptr) -> *mut AlPoint_ptr;
    fn alconstructionplane_third(plane: *mut AlConstructionPlane_ptr) -> *mut AlPoint_ptr;

    fn alconstructionplane_scale(
        plane: *mut AlConstructionPlane_ptr,
        x: *mut f64,
        y: *mut f64,
    ) -> statusCode;
    fn alconstructionplane_set_scale(
        plane: *mut AlConstructionPlane_ptr,
        x: f64,
        y: f64,
    ) -> statusCode;

    fn alconstructionplane_rotate(
        plane: *mut AlConstructionPlane_ptr,
        rx: *mut f64,
        ry: *mut f64,
        rz: *mut f64,
    ) -> statusCode;
    fn alconstructionplane_translate(
        plane: *mut AlConstructionPlane_ptr,
        tx: *mut f64,
        ty: *mut f64,
        tz: *mut f64,
    ) -> statusCode;

    fn alconstructionplane_transformation_matrix(
        plane: *mut AlConstructionPlane_ptr,
        matrix: *mut f64,
    ) -> statusCode;
    fn alconstructionplane_origin(
        plane: *mut AlConstructionPlane_ptr,
        x: *mut f64,
        y: *mut f64,
        z: *mut f64,
    ) -> statusCode;
    fn alconstructionplane_set_origin(
        plane: *mut AlConstructionPlane_ptr,
        x: f64,
        y: f64,
        z: f64,
    ) -> statusCode;
    fn alconstructionplane_axes(
        plane: *mut AlConstructionPlane_ptr,
        xAxis: *mut f64,
        yAxis: *mut f64,
        zAxis: *mut f64,
    ) -> statusCode;

    fn alconstructionplane_normal(
        plane: *mut AlConstructionPlane_ptr,
        x: *mut f64,
        y: *mut f64,
        z: *mut f64,
    ) -> statusCode;
    fn alconstructionplane_equation(
        plane: *mut AlConstructionPlane_ptr,
        a: *mut f64,
        b: *mut f64,
        c: *mut f64,
        d: *mut f64,
    ) -> statusCode;

    fn alconstructionplane_canvas(plane: *mut AlConstructionPlane_ptr) -> *mut AlCanvas_ptr;
}
