use crate::*;

#[repr(C)]
pub struct AlConstructionEntity_ptr {
    _private: [u8; 0],
}

#[derive(Debug)]
pub struct AlConstructionEntity {
    pub ptr: *mut AlConstructionEntity_ptr,
}

pub trait AlConstructionEntityMethods: AlObjectMethods + AlPickableMethods {
    fn as_construction_entity_ptr(&self) -> *mut AlConstructionEntity_ptr;
    fn layer(&self) -> Option<AlLayer> {
        let ptr = unsafe { alconstructionentity_layer(self.as_construction_entity_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(AlLayer { ptr })
        }
    }

    fn set_layer(&self, layer: &AlLayer) -> Result<(), String> {
        let status =
            unsafe { alconstructionentity_set_layer(self.as_construction_entity_ptr(), layer.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn invisible(&self) -> bool {
        unsafe { alconstructionentity_invisible(self.as_construction_entity_ptr()) }
    }

    fn set_invisible(&self, invisible: bool) -> Result<(), String> {
        let status = unsafe {
            alconstructionentity_set_invisible(self.as_construction_entity_ptr(), invisible)
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn templated(&self) -> Result<bool, String> {
        let mut out_templated: bool = false;
        let status = unsafe {
            alconstructionentity_templated(self.as_construction_entity_ptr(), &mut out_templated)
        };
        if status == statusCode::Success {
            Ok(out_templated)
        } else {
            Err(status.to_string())
        }
    }
}

impl AlObjectMethods for AlConstructionEntity {
    fn as_object_ptr(&self) -> *mut AlObject_ptr {
        self.ptr as *mut AlObject_ptr
    }
}

impl AlPickableMethods for AlConstructionEntity {
    fn as_pickable_ptr(&self) -> *mut AlPickable_ptr {
        self.ptr as *mut AlPickable_ptr
    }
}

impl AlConstructionEntityMethods for AlConstructionEntity {
    fn as_construction_entity_ptr(&self) -> *mut AlConstructionEntity_ptr {
        self.ptr
    }
}

unsafe extern "C" {
    fn alconstructionentity_layer(entity: *mut AlConstructionEntity_ptr) -> *mut AlLayer_ptr;
    fn alconstructionentity_set_layer(
        entity: *mut AlConstructionEntity_ptr,
        layer: *mut AlLayer_ptr,
    ) -> statusCode;
    fn alconstructionentity_invisible(entity: *mut AlConstructionEntity_ptr) -> bool;
    fn alconstructionentity_set_invisible(
        entity: *mut AlConstructionEntity_ptr,
        invisible: bool,
    ) -> statusCode;
    fn alconstructionentity_templated(
        entity: *mut AlConstructionEntity_ptr,
        outTemplated: *mut bool,
    ) -> statusCode;
}
