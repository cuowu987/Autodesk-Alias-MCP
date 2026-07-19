use crate::*;
use base_geometry_lib::RU_3dPoint;
#[repr(C)]
pub struct AlPoint_ptr {
    _private: [u8; 0],
}

#[derive(Debug)]
pub struct AlPoint {
    pub ptr: *mut AlPoint_ptr,
}

pub trait AlPointMethods: AlConstructionEntityMethods {
    fn as_point_ptr(&self) -> *mut AlPoint_ptr;

    fn world_position(&self) -> Result<RU_3dPoint, String> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;
        let status = unsafe { alpoint_world_position(self.as_point_ptr(), &mut x, &mut y, &mut z) };
        if status == statusCode::Success {
            Ok(RU_3dPoint { x, y, z })
        } else {
            Err(status.to_string())
        }
    }
}

impl AlObjectMethods for AlPoint {
    fn as_object_ptr(&self) -> *mut AlObject_ptr {
        self.ptr as *mut AlObject_ptr
    }
}
impl AlPickableMethods for AlPoint {
    fn as_pickable_ptr(&self) -> *mut AlPickable_ptr {
        self.ptr as *mut AlPickable_ptr
    }
}
impl AlConstructionEntityMethods for AlPoint {
    fn as_construction_entity_ptr(&self) -> *mut AlConstructionEntity_ptr {
        self.ptr as *mut AlConstructionEntity_ptr
    }
}

impl AlPointMethods for AlPoint {
    fn as_point_ptr(&self) -> *mut AlPoint_ptr {
        self.ptr
    }
}



unsafe extern "C" {
    fn alpoint_world_position(
        point: *mut AlPoint_ptr,
        x: *mut f64,
        y: *mut f64,
        z: *mut f64,
    ) -> statusCode;
}
