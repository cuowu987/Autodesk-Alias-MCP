#![allow(non_camel_case_types)]
use crate::*;
use base_geometry_lib::Point_Trait;
#[repr(C)]
pub struct AlSpacePoint_ptr {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug)]
pub struct AlSpacePoint {
    pub ptr: *mut AlSpacePoint_ptr,
}

impl Drop for AlSpacePoint {
    fn drop(&mut self) {
        unsafe { alsacepoint_destroy(self.ptr) };
        self.ptr = std::ptr::null_mut();
    }
}

impl Point_Trait for AlSpacePoint {
    fn X(&self) -> f64 {
        let s = self.world_position().unwrap();
        s.x
    }
    fn Y(&self) -> f64 {
        let s = self.world_position().unwrap();
        s.y
    }
    fn Z(&self) -> f64 {
        let s = self.world_position().unwrap();
        s.z
    }
    fn W(&self) -> f64 {
        1.0
    }
    fn SetX(&mut self, x: f64) {
        let s = self.world_position().unwrap();
        self.set_world_position(x, s.y, s.z).unwrap();
    }
    fn SetY(&mut self, y: f64) {
        let s = self.world_position().unwrap();
        self.set_world_position(s.x, y, s.z).unwrap();
    }
    fn SetZ(&mut self, z: f64) {
        let s = self.world_position().unwrap();
        self.set_world_position(s.x, s.y, z).unwrap();
    }
    fn SetW(&mut self, _w: f64) {
        //do nothing
    }
}
//为所有 Point_Trait 实现 to_spacepoint()方法
pub trait Point_Trait_ToSpacePoint {
    fn to_spacepoint(&self) -> Result<AlSpacePoint, String>;
}
impl<T: Point_Trait> Point_Trait_ToSpacePoint for T {
    fn to_spacepoint(&self) -> Result<AlSpacePoint, String> {
        let x = self.X();
        let y = self.Y();
        let z = self.Z();
        AlSpacePoint::new_create(x, y, z)
    }
}

impl AlSpacePoint {
    pub fn new() -> AlSpacePoint {
        unsafe {
            let ptr = alsacepoint_new();
            AlSpacePoint { ptr }
        }
    }
    pub fn create(&mut self, x: f64, y: f64, z: f64) -> Result<(), String> {
        let status = unsafe { alsacepoint_create(self.ptr, x, y, z) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }
    pub fn new_create(x: f64, y: f64, z: f64) -> Result<AlSpacePoint, String> {
        let mut spacepoint = Self::new();
        spacepoint.create(x, y, z)?;
        Ok(spacepoint)
    }

    pub fn set_world_position(&self, x: f64, y: f64, z: f64) -> Result<(), String> {
        let status = unsafe { alsacepoint_set_world_position(self.ptr, x, y, z) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }
}

impl AlObjectMethods for AlSpacePoint {
    fn as_object_ptr(&self) -> *mut AlObject_ptr {
        self.ptr as *mut AlObject_ptr
    }
}
impl AlPickableMethods for AlSpacePoint {
    fn as_pickable_ptr(&self) -> *mut AlPickable_ptr {
        self.ptr as *mut AlPickable_ptr
    }
}
impl AlConstructionEntityMethods for AlSpacePoint {
    fn as_construction_entity_ptr(&self) -> *mut AlConstructionEntity_ptr {
        self.ptr as *mut AlConstructionEntity_ptr
    }
}

impl AlPointMethods for AlSpacePoint {
    fn as_point_ptr(&self) -> *mut AlPoint_ptr {
        self.ptr as *mut AlPoint_ptr
    }
}

unsafe extern "C" {
    fn alsacepoint_new() -> *mut AlSpacePoint_ptr;
    fn alsacepoint_destroy(spacePoint: *mut AlSpacePoint_ptr);

    fn alsacepoint_create(spacePoint: *mut AlSpacePoint_ptr, x: f64, y: f64, z: f64) -> statusCode;
    fn alsacepoint_set_world_position(
        spacePoint: *mut AlSpacePoint_ptr,
        x: f64,
        y: f64,
        z: f64,
    ) -> statusCode;
}
