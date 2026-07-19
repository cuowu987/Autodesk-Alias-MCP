use crate::*;

#[repr(C)]
pub struct AlLocator_ptr {
    _private: [u8; 0],
}

#[derive(Debug)]
pub struct AlLocator {
    pub ptr: *mut AlLocator_ptr,
}

pub trait AlLocatorMethods: AlObjectMethods {
    fn as_locator_ptr(&self) -> *mut AlLocator_ptr;

    fn layer(&self) -> Option<AlLayer> {
        let ptr = unsafe { allocator_layer(self.as_locator_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(AlLayer { ptr })
        }
    }

    fn set_layer(&self, layer: &AlLayer) -> Result<(), String> {
        let status = unsafe { allocator_set_layer(self.as_locator_ptr(), layer.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn invisible(&self) -> bool {
        unsafe { allocator_invisible(self.as_locator_ptr()) }
    }

    fn set_invisible(&self, invisible: bool) -> Result<(), String> {
        let status = unsafe { allocator_set_invisible(self.as_locator_ptr(), invisible) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn pick(&self) -> Result<(), String> {
        let status = unsafe { allocator_pick(self.as_locator_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn unpick(&self) -> Result<(), String> {
        let status = unsafe { allocator_unpick(self.as_locator_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn is_picked(&self) -> Result<bool, String> {
        let mut out_is_picked: bool = false;
        let status = unsafe { allocator_is_picked(self.as_locator_ptr(), &mut out_is_picked) };
        if status == statusCode::Success {
            Ok(out_is_picked)
        } else {
            Err(status.to_string())
        }
    }

    fn templated(&self) -> Result<bool, String> {
        let mut out_templated: bool = false;
        let status = unsafe { allocator_templated(self.as_locator_ptr(), &mut out_templated) };
        if status == statusCode::Success {
            Ok(out_templated)
        } else {
            Err(status.to_string())
        }
    }
}

impl AlObjectMethods for AlLocator {
    fn as_object_ptr(&self) -> *mut AlObject_ptr {
        self.ptr as *mut AlObject_ptr
    }
}

impl AlLocatorMethods for AlLocator {
    fn as_locator_ptr(&self) -> *mut AlLocator_ptr {
        self.ptr
    }
}

unsafe extern "C" {
    fn allocator_layer(locator: *mut AlLocator_ptr) -> *mut AlLayer_ptr;
    fn allocator_set_layer(locator: *mut AlLocator_ptr, layer: *mut AlLayer_ptr) -> statusCode;
    fn allocator_invisible(locator: *mut AlLocator_ptr) -> bool;
    fn allocator_set_invisible(locator: *mut AlLocator_ptr, invisible: bool) -> statusCode;
    fn allocator_pick(locator: *mut AlLocator_ptr) -> statusCode;
    fn allocator_unpick(locator: *mut AlLocator_ptr) -> statusCode;
    fn allocator_is_picked(locator: *mut AlLocator_ptr, outIsPicked: *mut bool) -> statusCode;
    fn allocator_templated(locator: *mut AlLocator_ptr, outTemplated: *mut bool) -> statusCode;
}
