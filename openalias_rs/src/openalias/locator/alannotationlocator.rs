use crate::*;

#[repr(C)]
pub struct AlAnnotationLocator_ptr {
    _private: [u8; 0],
}

#[derive(Debug)]
pub struct AlAnnotationLocator {
    pub ptr: *mut AlAnnotationLocator_ptr,
}
impl Drop for AlAnnotationLocator {
    fn drop(&mut self) {
        unsafe {
            alannotationlocator_destroy(self.ptr);
            self.ptr = std::ptr::null_mut();
        }
    }
}
impl AlAnnotationLocator {
    pub fn create(x: f64, y: f64, z: f64, annotation: &str) -> Result<Self, String> {
        let mut ann = AlAnnotationLocator::new();
        ann.create_locator(x, y, z, annotation, x, y, z)?;
        Ok(ann)
    }

    pub fn new() -> Self {
        let ptr = unsafe { alannotationlocator_create() };
        if ptr.is_null() {
            panic!("Failed to create AlAnnotationLocator");
        } else {
            AlAnnotationLocator { ptr }
        }
    }

    pub fn create_locator(
        &mut self,
        start_x: f64,
        start_y: f64,
        start_z: f64,
        annotation: &str,
        end_x: f64,
        end_y: f64,
        end_z: f64,
    ) -> Result<(), String> {
        let c_annotation =
            std::ffi::CString::new(annotation).map_err(|_| "Invalid string".to_string())?;
        let status = unsafe {
            alannotationlocator_create_locator(
                self.ptr,
                start_x,
                start_y,
                start_z,
                c_annotation.as_ptr(),
                end_x,
                end_y,
                end_z,
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_string(&self, str: &str) -> Result<(), String> {
        let c_str = std::ffi::CString::new(str).map_err(|_| "Invalid string".to_string())?;
        let status = unsafe { alannotationlocator_set_string(self.ptr, c_str.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn string(&self) -> Option<String> {
        let c_str = unsafe { alannotationlocator_string(self.ptr) };
        if c_str.is_null() {
            None
        } else {
            Some(
                unsafe { std::ffi::CStr::from_ptr(c_str) }
                    .to_string_lossy()
                    .to_string(),
            )
        }
    }

    pub fn set_world_leader_position(&self, x: f64, y: f64, z: f64) -> Result<(), String> {
        let status = unsafe { alannotationlocator_set_world_leader_position(self.ptr, x, y, z) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn world_leader_position(&self) -> Result<(f64, f64, f64), String> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;
        let status =
            unsafe { alannotationlocator_world_leader_position(self.ptr, &mut x, &mut y, &mut z) };
        if status == statusCode::Success {
            Ok((x, y, z))
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_local_leader_position(&self, x: f64, y: f64, z: f64) -> Result<(), String> {
        let status = unsafe { alannotationlocator_set_local_leader_position(self.ptr, x, y, z) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn local_leader_position(&self) -> Result<(f64, f64, f64), String> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;
        let status =
            unsafe { alannotationlocator_local_leader_position(self.ptr, &mut x, &mut y, &mut z) };
        if status == statusCode::Success {
            Ok((x, y, z))
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_offset(&self, x: f64, y: f64, z: f64) -> Result<(), String> {
        let status = unsafe { alannotationlocator_set_offset(self.ptr, x, y, z) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn offset(&self) -> Result<(f64, f64, f64), String> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;
        let status = unsafe { alannotationlocator_offset(self.ptr, &mut x, &mut y, &mut z) };
        if status == statusCode::Success {
            Ok((x, y, z))
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_length(&self, length: f64) -> Result<(), String> {
        let status = unsafe { alannotationlocator_set_length(self.ptr, length) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn length(&self) -> Result<f64, String> {
        let mut out_length: f64 = 0.0;
        let status = unsafe { alannotationlocator_length(self.ptr, &mut out_length) };
        if status == statusCode::Success {
            Ok(out_length)
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_show_arrow(&self, show_arrow: bool) -> Result<(), String> {
        let status = unsafe { alannotationlocator_set_show_arrow(self.ptr, show_arrow) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn show_arrow(&self) -> Result<bool, String> {
        let mut out_show_arrow: bool = false;
        let status = unsafe { alannotationlocator_show_arrow(self.ptr, &mut out_show_arrow) };
        if status == statusCode::Success {
            Ok(out_show_arrow)
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_left_justify(&self, left_justify: bool) -> Result<(), String> {
        let status = unsafe { alannotationlocator_set_left_justify(self.ptr, left_justify) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn left_justify(&self) -> Result<bool, String> {
        let mut out_left_justify: bool = false;
        let status = unsafe { alannotationlocator_left_justify(self.ptr, &mut out_left_justify) };
        if status == statusCode::Success {
            Ok(out_left_justify)
        } else {
            Err(status.to_string())
        }
    }

    pub fn point(&self) -> Result<(f64, f64, f64), String> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;
        let status = unsafe { alannotationlocator_point(self.ptr, &mut x, &mut y, &mut z) };
        if status == statusCode::Success {
            Ok((x, y, z))
        } else {
            Err(status.to_string())
        }
    }
}

impl AlObjectMethods for AlAnnotationLocator {
    fn as_object_ptr(&self) -> *mut AlObject_ptr {
        self.ptr as *mut AlObject_ptr
    }
}

impl AlLocatorMethods for AlAnnotationLocator {
    fn as_locator_ptr(&self) -> *mut AlLocator_ptr {
        self.ptr as *mut AlLocator_ptr
    }
}

unsafe extern "C" {
    fn alannotationlocator_create() -> *mut AlAnnotationLocator_ptr;
    fn alannotationlocator_destroy(locator: *mut AlAnnotationLocator_ptr);
    fn alannotationlocator_create_locator(
        locator: *mut AlAnnotationLocator_ptr,
        startX: f64,
        startY: f64,
        startZ: f64,
        annotation: *const i8,
        endX: f64,
        endY: f64,
        endZ: f64,
    ) -> statusCode;
    fn alannotationlocator_set_string(
        locator: *mut AlAnnotationLocator_ptr,
        str: *const i8,
    ) -> statusCode;
    fn alannotationlocator_string(locator: *const AlAnnotationLocator_ptr) -> *const i8;
    fn alannotationlocator_set_world_leader_position(
        locator: *mut AlAnnotationLocator_ptr,
        x: f64,
        y: f64,
        z: f64,
    ) -> statusCode;
    fn alannotationlocator_world_leader_position(
        locator: *const AlAnnotationLocator_ptr,
        x: *mut f64,
        y: *mut f64,
        z: *mut f64,
    ) -> statusCode;
    fn alannotationlocator_set_local_leader_position(
        locator: *mut AlAnnotationLocator_ptr,
        x: f64,
        y: f64,
        z: f64,
    ) -> statusCode;
    fn alannotationlocator_local_leader_position(
        locator: *const AlAnnotationLocator_ptr,
        x: *mut f64,
        y: *mut f64,
        z: *mut f64,
    ) -> statusCode;
    fn alannotationlocator_set_offset(
        locator: *mut AlAnnotationLocator_ptr,
        x: f64,
        y: f64,
        z: f64,
    ) -> statusCode;
    fn alannotationlocator_offset(
        locator: *const AlAnnotationLocator_ptr,
        x: *mut f64,
        y: *mut f64,
        z: *mut f64,
    ) -> statusCode;
    fn alannotationlocator_set_length(
        locator: *mut AlAnnotationLocator_ptr,
        length: f64,
    ) -> statusCode;
    fn alannotationlocator_length(
        locator: *const AlAnnotationLocator_ptr,
        outLength: *mut f64,
    ) -> statusCode;
    fn alannotationlocator_set_show_arrow(
        locator: *mut AlAnnotationLocator_ptr,
        showArrow: bool,
    ) -> statusCode;
    fn alannotationlocator_show_arrow(
        locator: *const AlAnnotationLocator_ptr,
        outShowArrow: *mut bool,
    ) -> statusCode;
    fn alannotationlocator_set_left_justify(
        locator: *mut AlAnnotationLocator_ptr,
        leftJustify: bool,
    ) -> statusCode;
    fn alannotationlocator_left_justify(
        locator: *const AlAnnotationLocator_ptr,
        outLeftJustify: *mut bool,
    ) -> statusCode;
    fn alannotationlocator_point(
        locator: *const AlAnnotationLocator_ptr,
        x: *mut f64,
        y: *mut f64,
        z: *mut f64,
    ) -> statusCode;
}
