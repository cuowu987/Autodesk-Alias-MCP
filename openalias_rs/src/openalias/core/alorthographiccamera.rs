use crate::*;

#[repr(C)]
pub struct AlOrthographicCamera_ptr {
    _private: [u8; 0],
}

#[derive(Debug)]
pub struct AlOrthographicCamera {
    pub ptr: *mut AlOrthographicCamera_ptr,
}

impl AlOrthographicCamera {
    pub fn new() -> Self {
        Self {
            ptr: unsafe { alorthographiccamera_new() },
        }
    }

    pub fn create(&self, view_type: AlWindowViewType) -> Result<(), String> {
        let status = unsafe { alorthographiccamera_create(self.ptr, view_type) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }
    pub fn new_create(view_type: AlWindowViewType) -> Result<Self, String> {
        let cam = Self::new();
        cam.create(view_type)?;
        Ok(cam)
    }

}

impl Drop for AlOrthographicCamera {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                alorthographiccamera_delete(self.ptr);
            }
            self.ptr = std::ptr::null_mut();
        }
    }
}
impl AlObjectMethods for AlOrthographicCamera {
    fn as_object_ptr(&self) -> *mut AlObject_ptr {
        self.ptr as *mut AlObject_ptr
    }
}
impl AlCameraMethods for AlOrthographicCamera {
    fn as_camera_ptr(&self) -> *mut AlCamera_ptr {
        self.ptr as *mut AlCamera_ptr
    }
}

unsafe extern "C" {
    fn alorthographiccamera_new() -> *mut AlOrthographicCamera_ptr;
    fn alorthographiccamera_delete(camera: *mut AlOrthographicCamera_ptr);
    fn alorthographiccamera_create(
        camera: *mut AlOrthographicCamera_ptr,
        viewType: AlWindowViewType,
    ) -> statusCode;
}
