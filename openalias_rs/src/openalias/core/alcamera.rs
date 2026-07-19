use crate::*;

#[repr(C)]
pub struct AlCamera_ptr {
    _private: [u8; 0],
}

#[derive(Debug)]
pub struct AlCamera {
    pub ptr: *mut AlCamera_ptr,
}

pub trait AlCameraMethods: AlObjectMethods {
    fn as_camera_ptr(&self) -> *mut AlCamera_ptr;

    fn first_window(&self) -> Option<AlWindow> {
        let ptr = unsafe { alcamera_first_window(self.as_camera_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(AlWindow { ptr })
        }
    }

    fn next_window(&self, last_window: &AlWindow) -> Option<AlWindow> {
        let ptr = unsafe { alcamera_next_window(self.as_camera_ptr(), last_window.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlWindow { ptr })
        }
    }

    fn next_window_d(&self, last_window: &AlWindow) -> Result<(), String> {
        let status = unsafe { alcamera_next_window_d(self.as_camera_ptr(), last_window.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn near_clipping_plane(&self) -> Result<f64, String> {
        let mut out_value: f64 = 0.0;
        let status = unsafe { alcamera_near_clipping_plane(self.as_camera_ptr(), &mut out_value) };
        if status == statusCode::Success {
            Ok(out_value)
        } else {
            Err(status.to_string())
        }
    }

    fn far_clipping_plane(&self) -> Result<f64, String> {
        let mut out_value: f64 = 0.0;
        let status = unsafe { alcamera_far_clipping_plane(self.as_camera_ptr(), &mut out_value) };
        if status == statusCode::Success {
            Ok(out_value)
        } else {
            Err(status.to_string())
        }
    }

    fn set_near_clipping_plane(&self, value: f64) -> Result<(), String> {
        let status = unsafe { alcamera_set_near_clipping_plane(self.as_camera_ptr(), value) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn set_far_clipping_plane(&self, value: f64) -> Result<(), String> {
        let status = unsafe { alcamera_set_far_clipping_plane(self.as_camera_ptr(), value) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn stereo_view(&self) -> Result<(bool, f64), String> {
        let mut out_enabled: bool = false;
        let mut out_eye_separation: f64 = 0.0;
        let status = unsafe { alcamera_stereo_view(self.as_camera_ptr(), &mut out_enabled, &mut out_eye_separation) };
        if status == statusCode::Success {
            Ok((out_enabled, out_eye_separation))
        } else {
            Err(status.to_string())
        }
    }

    fn set_stereo_view(&self, enabled: bool, eye_separation: f64) -> Result<(), String> {
        let status = unsafe { alcamera_set_stereo_view(self.as_camera_ptr(), enabled, eye_separation) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn motion_blur(&self) -> Result<bool, String> {
        let mut out_enabled: bool = false;
        let status = unsafe { alcamera_motion_blur(self.as_camera_ptr(), &mut out_enabled) };
        if status == statusCode::Success {
            Ok(out_enabled)
        } else {
            Err(status.to_string())
        }
    }

    fn set_motion_blur(&self, enabled: bool) -> Result<(), String> {
        let status = unsafe { alcamera_set_motion_blur(self.as_camera_ptr(), enabled) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }
}

impl AlObjectMethods for AlCamera {
    fn as_object_ptr(&self) -> *mut AlObject_ptr {
        self.ptr as *mut AlObject_ptr
    }
}

impl AlCameraMethods for AlCamera {
    fn as_camera_ptr(&self) -> *mut AlCamera_ptr {
        self.ptr
    }
}

impl Drop for AlCamera {
    fn drop(&mut self) {
        self.destroy();
        self.ptr = std::ptr::null_mut();
    }
}

unsafe extern "C" {
    fn alcamera_first_window(camera: *mut AlCamera_ptr) -> *mut AlWindow_ptr;
    fn alcamera_next_window(camera: *mut AlCamera_ptr, lastWindow: *mut AlWindow_ptr) -> *mut AlWindow_ptr;
    fn alcamera_next_window_d(camera: *mut AlCamera_ptr, lastWindow: *mut AlWindow_ptr) -> statusCode;
    fn alcamera_near_clipping_plane(camera: *const AlCamera_ptr, outValue: *mut f64) -> statusCode;
    fn alcamera_far_clipping_plane(camera: *const AlCamera_ptr, outValue: *mut f64) -> statusCode;
    fn alcamera_set_near_clipping_plane(camera: *mut AlCamera_ptr, value: f64) -> statusCode;
    fn alcamera_set_far_clipping_plane(camera: *mut AlCamera_ptr, value: f64) -> statusCode;
    fn alcamera_stereo_view(camera: *const AlCamera_ptr, outEnabled: *mut bool, outEyeSeparation: *mut f64) -> statusCode;
    fn alcamera_set_stereo_view(camera: *mut AlCamera_ptr, enabled: bool, eyeSeparation: f64) -> statusCode;
    fn alcamera_motion_blur(camera: *const AlCamera_ptr, outEnabled: *mut bool) -> statusCode;
    fn alcamera_set_motion_blur(camera: *mut AlCamera_ptr, enabled: bool) -> statusCode;
}
