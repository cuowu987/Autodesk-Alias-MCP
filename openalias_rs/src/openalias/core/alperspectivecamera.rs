use crate::*;
use base_geometry_lib::*;

#[repr(C)]
pub struct AlPerspectiveCamera_ptr {
    _private: [u8; 0],
}

#[derive(Debug)]
pub struct AlPerspectiveCamera {
    pub ptr: *mut AlPerspectiveCamera_ptr,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum AlCameraWindowFitType {
    FillFit = 0,
    HorizontalFit = 1,
    VerticalFit = 2,
}

impl AlPerspectiveCamera {
    pub fn create() -> Result<AlPerspectiveCamera, String> {
        let mut camera = AlPerspectiveCamera {
            ptr: std::ptr::null_mut(),
        };
        let status = unsafe { alperspectivecamera_create(&mut camera.ptr) };
        if status == statusCode::Success {
            Ok(camera)
        } else {
            Err(status.to_string())
        }
    }

    pub fn eye(&self) -> Option<AlCameraNode> {
        let ptr = unsafe { alperspectivecamera_eye(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlCameraNode { ptr })
        }
    }

    pub fn view(&self) -> Option<AlCameraNode> {
        let ptr = unsafe { alperspectivecamera_view(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlCameraNode { ptr })
        }
    }

    pub fn up(&self) -> Option<AlCameraNode> {
        let ptr = unsafe { alperspectivecamera_up(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlCameraNode { ptr })
        }
    }

    pub fn world_eye(&self) -> Result<RU_3dPoint, String> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;
        let status = unsafe { alperspectivecamera_world_eye(self.ptr, &mut x, &mut y, &mut z) };
        if status == statusCode::Success {
            Ok(RU_3dPoint { x, y, z })
        } else {
            Err(status.to_string())
        }
    }

    pub fn world_view(&self) -> Result<RU_3dPoint, String> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;
        let status = unsafe { alperspectivecamera_world_view(self.ptr, &mut x, &mut y, &mut z) };
        if status == statusCode::Success {
            Ok(RU_3dPoint { x, y, z })
        } else {
            Err(status.to_string())
        }
    }

    pub fn world_up(&self) -> Result<RU_3dPoint, String> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;
        let status = unsafe { alperspectivecamera_world_up(self.ptr, &mut x, &mut y, &mut z) };
        if status == statusCode::Success {
            Ok(RU_3dPoint { x, y, z })
        } else {
            Err(status.to_string())
        }
    }

    pub fn world_eye_view_up(&self) -> Result<(RU_3dPoint, RU_3dPoint, RU_3dPoint), String> {
        let mut eye: RU_3dPoint = RU_3dPoint::default();
        let mut view: RU_3dPoint = RU_3dPoint::default();
        let mut up: RU_3dPoint = RU_3dPoint::default();
        let status = unsafe {
            alperspectivecamera_world_eye_view_up(
                self.ptr,
                &mut eye.x,
                &mut eye.y,
                &mut eye.z,
                &mut view.x,
                &mut view.y,
                &mut view.z,
                &mut up.x,
                &mut up.y,
                &mut up.z,
            )
        };
        if status == statusCode::Success {
            Ok((eye, view, up))
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_world_eye(&self, x: f64, y: f64, z: f64) -> Result<(), String> {
        let status = unsafe { alperspectivecamera_set_world_eye(self.ptr, x, y, z) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_world_view(&self, x: f64, y: f64, z: f64) -> Result<(), String> {
        let status = unsafe { alperspectivecamera_set_world_view(self.ptr, x, y, z) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_world_up(&self, x: f64, y: f64, z: f64) -> Result<(), String> {
        let status = unsafe { alperspectivecamera_set_world_up(self.ptr, x, y, z) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_world_eye_view_up(
        &self,
        eye: [f64; 3],
        view: [f64; 3],
        up: [f64; 3],
    ) -> Result<(), String> {
        let status = unsafe {
            alperspectivecamera_set_world_eye_view_up(
                self.ptr, eye[0], eye[1], eye[2], view[0], view[1], view[2], up[0], up[1], up[2],
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn twist_angle(&self) -> f64 {
        unsafe { alperspectivecamera_twist_angle(self.ptr) }
    }

    pub fn set_twist_angle(&self, angle: f64) -> Result<(), String> {
        let status = unsafe { alperspectivecamera_set_twist_angle(self.ptr, angle) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn change_twist_angle_by(&self, delta: f64) -> Result<(), String> {
        let status = unsafe { alperspectivecamera_change_twist_angle_by(self.ptr, delta) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn film_back(&self) -> Result<(f64, f64), String> {
        let mut width: f64 = 0.0;
        let mut height: f64 = 0.0;
        let status = unsafe { alperspectivecamera_film_back(self.ptr, &mut width, &mut height) };
        if status == statusCode::Success {
            Ok((width, height))
        } else {
            Err(status.to_string())
        }
    }

    pub fn film_offset(&self) -> Result<(f64, f64), String> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let status = unsafe { alperspectivecamera_film_offset(self.ptr, &mut x, &mut y) };
        if status == statusCode::Success {
            Ok((x, y))
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_film_back(&self, width: f64, height: f64) -> Result<(), String> {
        let status = unsafe { alperspectivecamera_set_film_back(self.ptr, width, height) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_film_offset(&self, x: f64, y: f64) -> Result<(), String> {
        let status = unsafe { alperspectivecamera_set_film_offset(self.ptr, x, y) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn focal_length(&self) -> f64 {
        unsafe { alperspectivecamera_focal_length(self.ptr) }
    }

    pub fn set_focal_length(&self, focal_length: f64) -> Result<(), String> {
        let status = unsafe { alperspectivecamera_set_focal_length(self.ptr, focal_length) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn angle_of_view(&self) -> f64 {
        unsafe { alperspectivecamera_angle_of_view(self.ptr) }
    }

    pub fn set_angle_of_view(&self, angle: f64) -> Result<(), String> {
        let status = unsafe { alperspectivecamera_set_angle_of_view(self.ptr, angle) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn depth_of_field(&self) -> Result<(bool, f64, f64), String> {
        let mut enabled: bool = false;
        let mut focal_plane: f64 = 0.0;
        let mut focal_range: f64 = 0.0;
        let status = unsafe {
            alperspectivecamera_depth_of_field(
                self.ptr,
                &mut enabled,
                &mut focal_plane,
                &mut focal_range,
            )
        };
        if status == statusCode::Success {
            Ok((enabled, focal_plane, focal_range))
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_depth_of_field(
        &self,
        enabled: bool,
        focal_plane: f64,
        focal_range: f64,
    ) -> Result<(), String> {
        let status = unsafe {
            alperspectivecamera_set_depth_of_field(self.ptr, enabled, focal_plane, focal_range)
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn placement_fit_code(&self) -> AlCameraWindowFitType {
        let s = unsafe { alperspectivecamera_placement_fit_code(self.ptr) };
        unsafe { std::mem::transmute(s) }
    }

    pub fn placement_shift(&self) -> f64 {
        unsafe { alperspectivecamera_placement_shift(self.ptr) }
    }

    pub fn set_placement_fit_code(&self, fit_type: AlCameraWindowFitType) -> Result<(), String> {
        let status = unsafe { alperspectivecamera_set_placement_fit_code(self.ptr, fit_type) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_placement_shift(&self, shift: f64) -> Result<(), String> {
        let status = unsafe { alperspectivecamera_set_placement_shift(self.ptr, shift) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn auto_focus(&self) -> Result<bool, String> {
        let mut enabled: bool = false;
        let status = unsafe { alperspectivecamera_auto_focus(self.ptr, &mut enabled) };
        if status == statusCode::Success {
            Ok(enabled)
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_auto_focus(&self, enabled: bool) -> Result<(), String> {
        let status = unsafe { alperspectivecamera_set_auto_focus(self.ptr, enabled) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }


    pub fn camera_params(
        &self,
    ) -> Result<
        (
            RU_3dPoint,
            RU_3dPoint,
            RU_3dVector,
            f64,
            f64,
            f64,
            f64,
            f64,
            f64,
            AlCameraWindowFitType,
        ),
        String,
    > {
        let eye_pos = self.world_eye()?;
        let center_pos = self.world_view()?;
        let up_pos = self.world_up()?;

        let (film_back_w, film_back_h) = self.film_back()?;
        let focal_length = self.focal_length();
        let fit_code = self.placement_fit_code();

        let up_vec = up_pos - eye_pos;

        Ok((
            eye_pos,
            center_pos,
            up_vec,
            self.angle_of_view(),
            self.near_clipping_plane()?,
            self.far_clipping_plane()?,
            film_back_w,
            film_back_h,
            focal_length,
            fit_code,
        ))
    }
}

impl AlObjectMethods for AlPerspectiveCamera {
    fn as_object_ptr(&self) -> *mut AlObject_ptr {
        self.ptr as *mut AlObject_ptr
    }
}

impl AlCameraMethods for AlPerspectiveCamera {
    fn as_camera_ptr(&self) -> *mut AlCamera_ptr {
        self.ptr as *mut AlCamera_ptr
    }
}

impl Drop for AlPerspectiveCamera {
    fn drop(&mut self) {
        self.destroy();
        self.ptr = std::ptr::null_mut();
    }
}

unsafe extern "C" {
    fn alperspectivecamera_create(camera: *mut *mut AlPerspectiveCamera_ptr) -> statusCode;
    fn alperspectivecamera_eye(camera: *const AlPerspectiveCamera_ptr) -> *mut AlCameraNode_ptr;
    fn alperspectivecamera_view(camera: *const AlPerspectiveCamera_ptr) -> *mut AlCameraNode_ptr;
    fn alperspectivecamera_up(camera: *const AlPerspectiveCamera_ptr) -> *mut AlCameraNode_ptr;
    fn alperspectivecamera_world_eye(
        camera: *const AlPerspectiveCamera_ptr,
        x: *mut f64,
        y: *mut f64,
        z: *mut f64,
    ) -> statusCode;
    fn alperspectivecamera_world_view(
        camera: *const AlPerspectiveCamera_ptr,
        x: *mut f64,
        y: *mut f64,
        z: *mut f64,
    ) -> statusCode;
    fn alperspectivecamera_world_up(
        camera: *const AlPerspectiveCamera_ptr,
        x: *mut f64,
        y: *mut f64,
        z: *mut f64,
    ) -> statusCode;
    fn alperspectivecamera_world_eye_view_up(
        camera: *const AlPerspectiveCamera_ptr,
        eyeX: *mut f64,
        eyeY: *mut f64,
        eyeZ: *mut f64,
        viewX: *mut f64,
        viewY: *mut f64,
        viewZ: *mut f64,
        upX: *mut f64,
        upY: *mut f64,
        upZ: *mut f64,
    ) -> statusCode;
    fn alperspectivecamera_set_world_eye(
        camera: *mut AlPerspectiveCamera_ptr,
        x: f64,
        y: f64,
        z: f64,
    ) -> statusCode;
    fn alperspectivecamera_set_world_view(
        camera: *mut AlPerspectiveCamera_ptr,
        x: f64,
        y: f64,
        z: f64,
    ) -> statusCode;
    fn alperspectivecamera_set_world_up(
        camera: *mut AlPerspectiveCamera_ptr,
        x: f64,
        y: f64,
        z: f64,
    ) -> statusCode;
    fn alperspectivecamera_set_world_eye_view_up(
        camera: *mut AlPerspectiveCamera_ptr,
        eyeX: f64,
        eyeY: f64,
        eyeZ: f64,
        viewX: f64,
        viewY: f64,
        viewZ: f64,
        upX: f64,
        upY: f64,
        upZ: f64,
    ) -> statusCode;
    fn alperspectivecamera_twist_angle(camera: *const AlPerspectiveCamera_ptr) -> f64;
    fn alperspectivecamera_set_twist_angle(
        camera: *mut AlPerspectiveCamera_ptr,
        angle: f64,
    ) -> statusCode;
    fn alperspectivecamera_change_twist_angle_by(
        camera: *mut AlPerspectiveCamera_ptr,
        delta: f64,
    ) -> statusCode;
    fn alperspectivecamera_film_back(
        camera: *const AlPerspectiveCamera_ptr,
        width: *mut f64,
        height: *mut f64,
    ) -> statusCode;
    fn alperspectivecamera_film_offset(
        camera: *const AlPerspectiveCamera_ptr,
        x: *mut f64,
        y: *mut f64,
    ) -> statusCode;
    fn alperspectivecamera_set_film_back(
        camera: *mut AlPerspectiveCamera_ptr,
        width: f64,
        height: f64,
    ) -> statusCode;
    fn alperspectivecamera_set_film_offset(
        camera: *mut AlPerspectiveCamera_ptr,
        x: f64,
        y: f64,
    ) -> statusCode;
    fn alperspectivecamera_focal_length(camera: *const AlPerspectiveCamera_ptr) -> f64;
    fn alperspectivecamera_set_focal_length(
        camera: *mut AlPerspectiveCamera_ptr,
        focalLength: f64,
    ) -> statusCode;
    fn alperspectivecamera_angle_of_view(camera: *const AlPerspectiveCamera_ptr) -> f64;
    fn alperspectivecamera_set_angle_of_view(
        camera: *mut AlPerspectiveCamera_ptr,
        angle: f64,
    ) -> statusCode;
    fn alperspectivecamera_depth_of_field(
        camera: *const AlPerspectiveCamera_ptr,
        outEnabled: *mut bool,
        outFocalPlane: *mut f64,
        outFocalRange: *mut f64,
    ) -> statusCode;
    fn alperspectivecamera_set_depth_of_field(
        camera: *mut AlPerspectiveCamera_ptr,
        enabled: bool,
        focalPlane: f64,
        focalRange: f64,
    ) -> statusCode;
    fn alperspectivecamera_placement_fit_code(camera: *const AlPerspectiveCamera_ptr) -> i32;
    fn alperspectivecamera_placement_shift(camera: *const AlPerspectiveCamera_ptr) -> f64;
    fn alperspectivecamera_set_placement_fit_code(
        camera: *mut AlPerspectiveCamera_ptr,
        fitType: AlCameraWindowFitType,
    ) -> statusCode;
    fn alperspectivecamera_set_placement_shift(
        camera: *mut AlPerspectiveCamera_ptr,
        shift: f64,
    ) -> statusCode;
    fn alperspectivecamera_auto_focus(
        camera: *const AlPerspectiveCamera_ptr,
        outEnabled: *mut bool,
    ) -> statusCode;
    fn alperspectivecamera_set_auto_focus(
        camera: *mut AlPerspectiveCamera_ptr,
        enabled: bool,
    ) -> statusCode;
}
