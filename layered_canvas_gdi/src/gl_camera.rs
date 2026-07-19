use base_geometry_lib::{RU_3dPoint, RU_3dVector};

use super::*;
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CameraFitType {
    FillFit,       // 填充模式：根据窗口宽高比与胶片背宽高比的最大值调整
    HorizontalFit, // 水平适配：保持水平 FOV 不变
    VerticalFit,   // 垂直适配：保持垂直 FOV 不变
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CameraType {
    Perspective,  // 透视相机（近大远小）
    Orthographic, // 正交相机（平行投影，无透视）
}

#[derive(Debug, Clone, Copy)]
pub struct GlCamera {
    pub eye: RU_3dPoint,
    pub center: RU_3dPoint,
    pub up: RU_3dVector,
    pub fov: f64,
    pub near: f64,
    pub far: f64,
    pub film_back_w: f64,
    pub film_back_h: f64,
    pub focal_length: f64,
    pub fit_type: CameraFitType,
    pub camera_type: CameraType,
    pub world_min: RU_3dPoint,
    pub world_max: RU_3dPoint,
    pub view_plane: base_geometry_lib::RU_Plane,
    pub view_plane_u_min: f64,
    pub view_plane_u_max: f64,
    pub view_plane_v_min: f64,
    pub view_plane_v_max: f64,
}
//打印显示内容
impl std::fmt::Display for GlCamera {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let info = match self.camera_type {
            CameraType::Orthographic => format!(
                "Camera: Orthographic\n\
             Eye: {}\n\
             Center: {}\n\
             View Plane Origin: ({:.2}, {:.2}, {:.2})\n\
             X Axis: ({:.2}, {:.2}, {:.2})\n\
             Y Axis: ({:.2}, {:.2}, {:.2})",
                self.eye,
                self.center,
                self.view_plane.origin.x,
                self.view_plane.origin.y,
                self.view_plane.origin.z,
                self.view_plane.xaxis.x,
                self.view_plane.xaxis.y,
                self.view_plane.xaxis.z,
                self.view_plane.yaxis.x,
                self.view_plane.yaxis.y,
                self.view_plane.yaxis.z
            ),
            CameraType::Perspective => format!(
                "Camera: Perspective\n\
             Eye: {}\n\
             Center: {}\n\
             FOV: {:.4}deg\n\
             Film: {:.4}x{:.4}, Focal: {:.4}\n\
             Fit: {:?}",
                self.eye,
                self.center,
                self.fov,
                self.film_back_w,
                self.film_back_h,
                self.focal_length,
                self.fit_type,
            ),
        };

        write!(f, "{}", info)
    }
}

impl Default for GlCamera {
    fn default() -> Self {
        Self {
            eye: RU_3dPoint::new(3.0, 2.0, 5.0),
            center: RU_3dPoint::new(0.0, 0.0, 0.0),
            up: RU_3dVector::new(0.0, 1.0, 0.0),
            fov: 45.0,
            near: 0.1,
            far: 100.0,
            film_back_w: 36.0,
            film_back_h: 24.0,
            focal_length: 50.0,
            fit_type: CameraFitType::HorizontalFit,
            camera_type: CameraType::Perspective,
            world_min: RU_3dPoint::new(-10.0, -10.0, -10.0),
            world_max: RU_3dPoint::new(10.0, 10.0, 10.0),
            view_plane: base_geometry_lib::RU_Plane::XY,
            view_plane_u_min: -1.0,
            view_plane_u_max: 1.0,
            view_plane_v_min: -1.0,
            view_plane_v_max: 1.0,
        }
    }
}

impl GlCamera {
    /// 通过投影参数创建正交相机
    pub fn from_orthographic(
        origin: base_geometry_lib::RU_3dPoint,
        right: base_geometry_lib::RU_3dPoint,
        up: base_geometry_lib::RU_3dPoint,
        top_right: base_geometry_lib::RU_3dPoint,
        center: base_geometry_lib::RU_3dPoint,
        view_dir: base_geometry_lib::RU_3dVector,
    ) -> Self {
        let x_axis = (right - origin).UnitVector();
        let y_axis_raw = up - origin;

        let view_plane = base_geometry_lib::RU_Plane::new(origin, x_axis, y_axis_raw)
            .unwrap_or(base_geometry_lib::RU_Plane::XY);

        let view_dir_normalized = if view_dir.Length() > 1e-12 {
            view_dir.UnitVector()
        } else {
            base_geometry_lib::RU_3dVector::Z_AXIS
        };
        let eye = center - view_dir_normalized * 1000.0;

        let vec_to_right = right - origin;
        let vec_to_up = up - origin;
        let vec_to_top_right = top_right - origin;

        let u_right = vec_to_right * view_plane.xaxis;
        let u_up = vec_to_up * view_plane.xaxis;
        let u_top_right = vec_to_top_right * view_plane.xaxis;

        let v_right = vec_to_right * view_plane.yaxis;
        let v_up = vec_to_up * view_plane.yaxis;
        let v_top_right = vec_to_top_right * view_plane.yaxis;

        let view_plane_u_min = [0.0, u_right, u_up, u_top_right]
            .iter()
            .cloned()
            .fold(f64::INFINITY, f64::min);
        let view_plane_u_max = [0.0, u_right, u_up, u_top_right]
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let view_plane_v_min = [0.0, v_right, v_up, v_top_right]
            .iter()
            .cloned()
            .fold(f64::INFINITY, f64::min);
        let view_plane_v_max = [0.0, v_right, v_up, v_top_right]
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);

        Self {
            eye,
            center,
            up: view_plane.yaxis,
            fov: 0.0,
            near: -10000.0,
            far: 10000.0,
            film_back_w: 0.0,
            film_back_h: 0.0,
            focal_length: 0.0,
            fit_type: CameraFitType::HorizontalFit,
            camera_type: CameraType::Orthographic,
            world_min: origin,
            world_max: top_right,
            view_plane,
            view_plane_u_min,
            view_plane_u_max,
            view_plane_v_min,
            view_plane_v_max,
        }
    }

    /// 通过投影参数创建透视相机
    pub fn from_perspective(
        eye: base_geometry_lib::RU_3dPoint,
        center: base_geometry_lib::RU_3dPoint,
        up: base_geometry_lib::RU_3dVector,
        fov: f64,
        near: f64,
        far: f64,
        film_back_w: f64,
        film_back_h: f64,
        focal_length: f64,
        fit_type: i32,
    ) -> Self {
        let up_normalized = if up.Length() > 1e-12 {
            up.UnitVector()
        } else {
            base_geometry_lib::RU_3dVector::Y_AXIS
        };

        Self {
            eye,
            center,
            up: up_normalized,
            fov,
            near,
            far,
            film_back_w,
            film_back_h,
            focal_length,
            fit_type: unsafe { std::mem::transmute(fit_type) },
            camera_type: CameraType::Perspective,
            world_min: RU_3dPoint::new(-10.0, -10.0, -10.0),
            world_max: RU_3dPoint::new(10.0, 10.0, 10.0),
            view_plane: base_geometry_lib::RU_Plane::XY,
            view_plane_u_min: -1.0,
            view_plane_u_max: 1.0,
            view_plane_v_min: -1.0,
            view_plane_v_max: 1.0,
        }
    }
}
