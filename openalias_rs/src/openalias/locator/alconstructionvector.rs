use base_geometry_lib::*;

use crate::*;

#[repr(C)]
pub struct AlConstructionVector_ptr {
    _private: [u8; 0],
}

#[derive(Debug)]
pub struct AlConstructionVector {
    pub ptr: *mut AlConstructionVector_ptr,
}

impl Drop for AlConstructionVector {
    fn drop(&mut self) {
        unsafe {
            alconstructionvector_destroy(self.ptr);
            self.ptr = std::ptr::null_mut();
        }
    }
}

impl AlConstructionVector {
    pub fn new() -> Self {
        let ptr = unsafe { alconstructionvector_new() };
        AlConstructionVector { ptr }
    }

    pub fn create_point_vector(
        &self,
        origin: &AlPoint,
        x: f64,
        y: f64,
        z: f64,
    ) -> Result<(), String> {
        let status =
            unsafe { alconstructionvector_create_point_vector(self.ptr, origin.ptr, x, y, z) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_two_points<T: AlPointMethods>(&self, origin: T, end: T) -> Result<(), String> {
        let status = unsafe {
            alconstructionvector_create_two_points(
                self.ptr,
                origin.as_point_ptr(),
                end.as_point_ptr(),
            )
        };
        if status == statusCode::Success {
            std::mem::forget(origin);
            std::mem::forget(end);
            Ok(())
        } else {
            Err(status.to_string())
        }
    }
    pub fn new_create(origin: RU_3dPoint, vec: RU_3dVector) -> Result<Self, String> {
        let spacePoint = AlSpacePoint::new_create(origin.x, origin.y, origin.z)?;
        let vector = Self::new();
        let status = unsafe {
            alconstructionvector_create_point_vector(
                vector.ptr,
                spacePoint.as_point_ptr(),
                vec.x,
                vec.y,
                vec.z,
            )
        };
        if status == statusCode::Success {
            Ok(vector)
        } else {
            Err(status.to_string())
        }
    }

    pub fn vector(&self) -> Result<[f64; 3], String> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;
        let status = unsafe { alconstructionvector_vector(self.ptr, &mut x, &mut y, &mut z) };
        if status == statusCode::Success {
            Ok([x, y, z])
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_vector(&self, x: f64, y: f64, z: f64) -> Result<(), String> {
        let status = unsafe { alconstructionvector_set_vector(self.ptr, x, y, z) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn origin(&self) -> Option<AlPoint> {
        let ptr = unsafe { alconstructionvector_origin(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlPoint { ptr })
        }
    }

    pub fn end(&self) -> Option<AlPoint> {
        let ptr = unsafe { alconstructionvector_end(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlPoint { ptr })
        }
    }
}

impl AlObjectMethods for AlConstructionVector {
    fn as_object_ptr(&self) -> *mut AlObject_ptr {
        self.ptr as *mut AlObject_ptr
    }
}
impl AlPickableMethods for AlConstructionVector {
    fn as_pickable_ptr(&self) -> *mut AlPickable_ptr {
        self.ptr as *mut AlPickable_ptr
    }
}
impl AlConstructionEntityMethods for AlConstructionVector {
    fn as_construction_entity_ptr(&self) -> *mut AlConstructionEntity_ptr {
        self.ptr as *mut AlConstructionEntity_ptr
    }
}
impl Point_Trait for AlConstructionVector {
    fn X(&self) -> f64 {
        self.vector().unwrap()[0]
    }
    fn Y(&self) -> f64 {
        self.vector().unwrap()[1]
    }
    fn Z(&self) -> f64 {
        self.vector().unwrap()[2]
    }
    fn W(&self) -> f64 {
        1.0
    }
    fn SetX(&mut self, x: f64) {
        let s = self.vector().unwrap();
        self.set_vector(x, s[1], s[2]).unwrap();
    }
    fn SetY(&mut self, y: f64) {
        let s = self.vector().unwrap();
        self.set_vector(s[0], y, s[2]).unwrap();
    }
    fn SetZ(&mut self, z: f64) {
        let s = self.vector().unwrap();
        self.set_vector(s[0], s[1], z).unwrap();
    }
    fn SetW(&mut self, _w: f64) {
        //do nothing
    }
}

impl Vector_Trait for AlConstructionVector {}

//为所有 Vector_Trait 实现 to_construction_vector()方法
#[allow(non_camel_case_types)]
pub trait Vector_Trait_ToConstructionVector {
    fn to_construction_vector(&self, origin: RU_3dPoint) -> Result<AlConstructionVector, String>;
}
impl<T: Vector_Trait> Vector_Trait_ToConstructionVector for T {
    fn to_construction_vector(&self, origin: RU_3dPoint) -> Result<AlConstructionVector, String> {
        let vec = self.to_3dvector();
        AlConstructionVector::new_create(origin, vec)
    }
}

unsafe extern "C" {
    fn alconstructionvector_new() -> *mut AlConstructionVector_ptr;
    fn alconstructionvector_destroy(vec: *mut AlConstructionVector_ptr);

    fn alconstructionvector_create_point_vector(
        vec: *mut AlConstructionVector_ptr,
        origin: *mut AlPoint_ptr,
        x: f64,
        y: f64,
        z: f64,
    ) -> statusCode;
    fn alconstructionvector_create_two_points(
        vec: *mut AlConstructionVector_ptr,
        origin: *mut AlPoint_ptr,
        end: *mut AlPoint_ptr,
    ) -> statusCode;

    fn alconstructionvector_vector(
        vec: *mut AlConstructionVector_ptr,
        x: *mut f64,
        y: *mut f64,
        z: *mut f64,
    ) -> statusCode;
    fn alconstructionvector_set_vector(
        vec: *mut AlConstructionVector_ptr,
        x: f64,
        y: f64,
        z: f64,
    ) -> statusCode;

    fn alconstructionvector_origin(vec: *mut AlConstructionVector_ptr) -> *mut AlPoint_ptr;
    fn alconstructionvector_end(vec: *mut AlConstructionVector_ptr) -> *mut AlPoint_ptr;
}
