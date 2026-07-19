#![allow(non_camel_case_types)]
use base_geometry_lib::*;

use crate::*;

#[repr(C)]
pub struct AlWindow_ptr {
    _private: [u8; 0],
}

#[derive(Debug)]
pub struct AlWindow {
    pub ptr: *mut AlWindow_ptr,
}

impl AlWindow {
    pub fn new() -> AlWindow {
        let ptr = unsafe { alwindow_create() };
        if ptr.is_null() {
            AlWindow {
                ptr: std::ptr::null_mut(),
            }
        } else {
            AlWindow { ptr }
        }
    }

    pub fn destroy(&mut self) {
        unsafe { alwindow_destroy(self.ptr) };
    }

    pub fn create_window(&self, view_type: AlWindowViewType) -> Result<(), String> {
        let status = unsafe { alwindow_create_window(self.ptr, view_type) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn next(&self) -> Option<AlWindow> {
        let ptr = unsafe { alwindow_next(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlWindow { ptr })
        }
    }

    pub fn prev(&self) -> Option<AlWindow> {
        let ptr = unsafe { alwindow_prev(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlWindow { ptr })
        }
    }

    pub fn next_d(&self) -> Result<(), String> {
        let status = unsafe { alwindow_next_d(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn prev_d(&self) -> Result<(), String> {
        let status = unsafe { alwindow_prev_d(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn window_type(&self) -> Result<AlWindowViewType, String> {
        let mut view_type = AlWindowViewType::kViewInvalid;
        let status = unsafe { alwindow_window_type(self.ptr, &mut view_type) };
        if status == statusCode::Success {
            Ok(view_type)
        } else {
            Err(status.to_string())
        }
    }

    pub fn is_zoom(&self) -> Result<bool, String> {
        let mut is_zoom = false;
        let status = unsafe { alwindow_is_zoom(self.ptr, &mut is_zoom) };
        if status == statusCode::Success {
            Ok(is_zoom)
        } else {
            Err(status.to_string())
        }
    }

    pub fn is_visible(&self) -> Result<bool, String> {
        let mut is_visible = false;
        let status = unsafe { alwindow_is_visible(self.ptr, &mut is_visible) };
        if status == statusCode::Success {
            Ok(is_visible)
        } else {
            Err(status.to_string())
        }
    }
    /// 是否是正交投影(但没有效果)
    #[deprecated = "无效，不建议使用"]
    pub fn orthographic(&self) -> Result<bool, String> {
        let mut orthographic = false;
        let status = unsafe { alwindow_orthographic(self.ptr, &mut orthographic) };
        if status == statusCode::Success {
            Ok(orthographic)
        } else {
            Err(status.to_string())
        }
    }

    pub fn camera(&self) -> Option<AlCamera> {
        let ptr = unsafe { alwindow_camera(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlCamera { ptr })
        }
    }

    pub fn set_camera(&self, camera: &AlPerspectiveCamera) -> Result<(), String> {
        let status = unsafe { alwindow_set_camera(self.ptr, camera.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }
    /// 这是相对于外层窗口工作区域中，alwindow左下角的相对位置
    pub fn position(&self) -> Result<(i32, i32), String> {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let status = unsafe { alwindow_position(self.ptr, &mut x, &mut y) };
        if status == statusCode::Success {
            Ok((x, y))
        } else {
            Err(status.to_string())
        }
    }
    /// 这是相对于屏幕的绝对位置，alwindow左上角的绝对位置
    /// 返回 (x, y, w, h) 分别是 alwindow 左上角的绝对位置和客户窗口的尺寸
    pub fn position_absolute(&self) -> Result<(i32, i32, i32, i32), String> {
        pub use windows::{
            Win32::Foundation::{POINT, RECT},
            Win32::Graphics::Gdi::ClientToScreen,
            Win32::UI::WindowsAndMessaging::{GetClientRect, GetForegroundWindow, GetParent},
        };
        unsafe {
            let hwnd_fg = GetForegroundWindow();
            let mut hwnd_top = hwnd_fg;
            // 找到顶层窗口
            while {
                let p = GetParent(hwnd_top);
                if p.0 != 0 {
                    hwnd_top = p;
                    true
                } else {
                    false
                }
            } {}
            // 获取客户窗口矩形
            let mut cr = RECT::default();
            let _ = GetClientRect(hwnd_top, &mut cr);
            // 获取客户窗口左上角屏幕坐标
            let mut cp = POINT { x: 0, y: 0 };
            let _ = ClientToScreen(hwnd_top, &mut cp);

            let (x, y) = self.position()?;
            let (w, h) = self.resolution()?;
            //window 左上坐标
            let x = cp.x + x;
            let y = cp.y + (cr.bottom - cr.top) - y - h;

            println!(
                "║  │ 顶层窗口 客户区 左上角屏幕 坐标: ({:>5},{:>5}) 客户区尺寸 WxH: {:>5} x {:<5}",
                x, y, w, h
            );
            Ok((x, y, w, h))
        }
    }

    pub fn set_position(&self, x: i32, y: i32) -> Result<(), String> {
        let status = unsafe { alwindow_set_position(self.ptr, x, y) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn resolution(&self) -> Result<(i32, i32), String> {
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        let status = unsafe { alwindow_resolution(self.ptr, &mut width, &mut height) };
        if status == statusCode::Success {
            Ok((width, height))
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_resolution(
        &self,
        width: i32,
        height: i32,
        corner: AlWindowCornerType,
        aspect: AlWindowAspectType,
    ) -> Result<(), String> {
        let status = unsafe { alwindow_set_resolution(self.ptr, width, height, corner, aspect) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn priority(&self) -> Result<i32, String> {
        let mut priority = 0;
        let status = unsafe { alwindow_priority(self.ptr, &mut priority) };
        if status == statusCode::Success {
            Ok(priority)
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_priority(&self, priority: i32) -> Result<(), String> {
        let status = unsafe { alwindow_set_priority(self.ptr, priority) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn grid_size(&self) -> f64 {
        unsafe { alwindow_grid_size(self.ptr) }
    }

    pub fn set_grid_size(&self, size: f64) -> Result<(), String> {
        let status = unsafe { alwindow_set_grid_size(self.ptr, size) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn world_space_bounds(&self) -> Result<([f64; 3], [f64; 3]), String> {
        let mut min_x: f64 = 0.0;
        let mut min_y: f64 = 0.0;
        let mut min_z: f64 = 0.0;
        let mut max_x: f64 = 0.0;
        let mut max_y: f64 = 0.0;
        let mut max_z: f64 = 0.0;
        let status = unsafe {
            alwindow_world_space_bounds(
                self.ptr, &mut min_x, &mut min_y, &mut min_z, &mut max_x, &mut max_y, &mut max_z,
            )
        };
        if status == statusCode::Success {
            Ok(([min_x, min_y, min_z], [max_x, max_y, max_z]))
        } else {
            Err(status.to_string())
        }
    }

    pub fn map_to_world_space(&self, screen_x: i32, screen_y: i32) -> Result<RU_Line, String> {
        let mut world_x: f64 = 0.0;
        let mut world_y: f64 = 0.0;
        let mut world_z: f64 = 0.0;
        let mut normal_x: f64 = 0.0;
        let mut normal_y: f64 = 0.0;
        let mut normal_z: f64 = 0.0;
        let status = unsafe {
            alwindow_map_to_world_space(
                self.ptr,
                screen_x,
                screen_y,
                &mut world_x,
                &mut world_y,
                &mut world_z,
                &mut normal_x,
                &mut normal_y,
                &mut normal_z,
            )
        };
        if status == statusCode::Success {
            Ok(RU_Line::new(
                RU_3dPoint::new(world_x, world_y, world_z),
                RU_3dPoint::new(normal_x, normal_y, normal_z),
            ))
        } else {
            Err(status.to_string())
        }
    }
    pub fn map_to_world_postion(
        &self,
        screen_x: i32,
        screen_y: i32,
        plane: RU_Plane,
    ) -> Option<(RU_3dPoint, f64)> {
        let line = self.map_to_world_space(screen_x, screen_y).ok()?;
        plane.IntersectLine(line)
    }

    pub fn window_toggle(&self, item: AlWindowToggle) -> Result<bool, String> {
        let mut state = false;
        let status = unsafe { alwindow_window_toggle(self.ptr, item, &mut state) };
        if status == statusCode::Success {
            Ok(state)
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_window_toggle(&self, item: AlWindowToggle, state: bool) -> Result<(), String> {
        let status = unsafe { alwindow_set_window_toggle(self.ptr, item, state) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn alias_window_size() -> Result<(i32, i32), String> {
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        let status = unsafe { alwindow_alias_window_size(&mut width, &mut height) };
        if status == statusCode::Success {
            Ok((width, height))
        } else {
            Err(status.to_string())
        }
    }

    pub fn alias_window_position() -> Result<(i32, i32), String> {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let status = unsafe { alwindow_alias_window_position(&mut x, &mut y) };
        if status == statusCode::Success {
            Ok((x, y))
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_default_windows() {
        unsafe { alwindow_create_default_windows() };
    }
    /// 获取当前窗口的视图平面
    pub fn eye_plane(&self) -> Result<RU_Plane, String> {
        let (w, h) = self.resolution()?;
        let screen_center = (w as f32 / 2.0, h as f32 / 2.0);
        let screen_center_pos = self
            .map_to_world_space(
                screen_center.0.round() as i32,
                screen_center.1.round() as i32,
            )?
            .from;

        let screen_origin_pos = self.map_to_world_space(0, 0)?.from;
        let screen_right_pos = self.map_to_world_space(w, 0)?.from;
        let screen_up_pos = self.map_to_world_space(0, h)?.from;

        let x_axis = screen_right_pos - screen_origin_pos;
        let y_axis = screen_up_pos - screen_origin_pos;

        RU_Plane::new(screen_center_pos, x_axis, y_axis)
    }

    /// 获取正交相机参数
    /// 返回：(origin, right, up, top_right, center, view_dir)
    pub fn orthographic_camera_params(
        &self,
    ) -> Result<
        (
            RU_3dPoint,
            RU_3dPoint,
            RU_3dPoint,
            RU_3dPoint,
            RU_3dPoint,
            RU_3dVector,
        ),
        String,
    > {
        let (w, h) = self.resolution()?;
        let screen_center = (w as f32 / 2.0, h as f32 / 2.0);

        let center_line = self.map_to_world_space(
            screen_center.0.round() as i32,
            screen_center.1.round() as i32,
        )?;

        let screen_origin_pt = self.map_to_world_space(0, 0)?.from;
        let screen_right_pt = self.map_to_world_space(w as i32, 0)?.from;
        let screen_up_pt = self.map_to_world_space(0, h as i32)?.from;
        let screen_top_right_pt = self.map_to_world_space(w as i32, h as i32)?.from;

        let view_dir = center_line.to - center_line.from;

        Ok((
            screen_origin_pt,
            screen_right_pt,
            screen_up_pt,
            screen_top_right_pt,
            center_line.from,
            view_dir,
        ))
    }
    /// 将屏幕坐标转换为世界坐标
    /// # Arguments
    /// * `self` - 当前窗口
    /// * `screen_x` - 屏幕坐标x轴
    /// * `screen_y` - 屏幕坐标y轴
    /// * `target` - 目标点
    /// # Returns
    /// * `Result<RU_3dPoint, String>` - 世界坐标
    pub fn screen_pos_to_world_pos(
        &self,
        screen_x: i32,
        screen_y: i32,
        target: RU_3dPoint,
    ) -> Result<RU_3dPoint, String> {
        // 计算目标点平面
        let mut plane = self.eye_plane()?;
        plane.origin = target;

        let screen_pos = [screen_x, screen_y];
        let view_line = self.map_to_world_space(screen_pos[0], screen_pos[1])?;
        Ok(plane.IntersectLine(view_line).ok_or("project is null")?.0)
    }
    pub fn world_pos_to_screen_pos(
        &self,
        world_pos: RU_3dPoint,
    ) -> Result<((i32, i32), (f64, f64)), String> {
        let (w, h) = self.resolution()?;
        let origin_pos = self.screen_pos_to_world_pos(0, 0, world_pos)?;
        let x_pos = self.screen_pos_to_world_pos(w, 0, world_pos)?;
        let y_pos = self.screen_pos_to_world_pos(0, h, world_pos)?;

        let x_axis = x_pos - origin_pos;
        let y_axis = y_pos - origin_pos;

        let point_1 = world_pos - origin_pos;

        let x_rate = (point_1 * x_axis.UnitVector()) / x_axis.Length();
        let y_rate = (point_1 * y_axis.UnitVector()) / y_axis.Length();

        let screen_x = (x_rate * w as f64).round() as i32;
        let screen_y = (y_rate * h as f64).round() as i32;
        Ok(((screen_x, screen_y), (x_rate, y_rate)))
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum AlWindowViewType {
    kViewInvalid = 0,
    kViewOther = 1,
    kFront = 2,
    kBack = 3, /* unused */
    kRight = 4,
    kLeft = 5, /* unused */
    kTop = 6,
    kBottom = 7, /* unused */
    kSbd = 8,
    kPerspective = 9,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum AlWindowCornerType {
    TopLeft = 0,
    TopRight = 1,
    BottomLeft = 2,
    BottomRight = 3,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum AlWindowAspectType {
    KeepAspectRatio = 0,
    IgnoreAspectRatio = 1,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum AlWindowToggle {
    kModel = 0,
    kModelOnly = 1,
    kPivots = 2,
    kGrid = 3,
    kGuidelines = 4,
    kLocators = 5,
    kConstructionObjects = 6,
    kCanvases = 7,
    kLights = 8,
    kTextures = 9,
    kCameras = 10,
    kImagePlanes = 11,
    kClouds = 12,
    kNonProportional = 13,
}

unsafe extern "C" {
    fn alwindow_create() -> *mut AlWindow_ptr;
    fn alwindow_destroy(window: *mut AlWindow_ptr);
    fn alwindow_create_window(window: *mut AlWindow_ptr, viewType: AlWindowViewType) -> statusCode;
    fn alwindow_next(window: *mut AlWindow_ptr) -> *mut AlWindow_ptr;
    fn alwindow_prev(window: *mut AlWindow_ptr) -> *mut AlWindow_ptr;
    fn alwindow_next_d(window: *mut AlWindow_ptr) -> statusCode;
    fn alwindow_prev_d(window: *mut AlWindow_ptr) -> statusCode;
    fn alwindow_window_type(
        window: *mut AlWindow_ptr,
        outViewType: &mut AlWindowViewType,
    ) -> statusCode;
    fn alwindow_is_zoom(window: *mut AlWindow_ptr, outIsZoom: &mut bool) -> statusCode;
    fn alwindow_is_visible(window: *mut AlWindow_ptr, outIsVisible: &mut bool) -> statusCode;
    fn alwindow_orthographic(window: *mut AlWindow_ptr, outOrthographic: &mut bool) -> statusCode;
    fn alwindow_camera(window: *mut AlWindow_ptr) -> *mut crate::openalias::AlCamera_ptr;
    fn alwindow_set_camera(
        window: *mut AlWindow_ptr,
        camera: *mut crate::openalias::AlPerspectiveCamera_ptr,
    ) -> statusCode;
    fn alwindow_position(window: *mut AlWindow_ptr, outX: &mut i32, outY: &mut i32) -> statusCode;
    fn alwindow_set_position(window: *mut AlWindow_ptr, x: i32, y: i32) -> statusCode;
    fn alwindow_resolution(
        window: *mut AlWindow_ptr,
        outWidth: &mut i32,
        outHeight: &mut i32,
    ) -> statusCode;
    fn alwindow_set_resolution(
        window: *mut AlWindow_ptr,
        width: i32,
        height: i32,
        corner: AlWindowCornerType,
        aspect: AlWindowAspectType,
    ) -> statusCode;
    fn alwindow_priority(window: *mut AlWindow_ptr, outPriority: &mut i32) -> statusCode;
    fn alwindow_set_priority(window: *mut AlWindow_ptr, priority: i32) -> statusCode;
    fn alwindow_grid_size(window: *mut AlWindow_ptr) -> f64;
    fn alwindow_set_grid_size(window: *mut AlWindow_ptr, size: f64) -> statusCode;
    fn alwindow_world_space_bounds(
        window: *mut AlWindow_ptr,
        outMinX: &mut f64,
        outMinY: &mut f64,
        outMinZ: &mut f64,
        outMaxX: &mut f64,
        outMaxY: &mut f64,
        outMaxZ: &mut f64,
    ) -> statusCode;
    fn alwindow_map_to_world_space(
        window: *mut AlWindow_ptr,
        screenX: i32,
        screenY: i32,
        outWorldX: &mut f64,
        outWorldY: &mut f64,
        outWorldZ: &mut f64,
        outNormalX: &mut f64,
        outNormalY: &mut f64,
        outNormalZ: &mut f64,
    ) -> statusCode;
    fn alwindow_window_toggle(
        window: *mut AlWindow_ptr,
        item: AlWindowToggle,
        outState: &mut bool,
    ) -> statusCode;
    fn alwindow_set_window_toggle(
        window: *mut AlWindow_ptr,
        item: AlWindowToggle,
        state: bool,
    ) -> statusCode;
    fn alwindow_alias_window_size(outWidth: &mut i32, outHeight: &mut i32) -> statusCode;
    fn alwindow_alias_window_position(outX: &mut i32, outY: &mut i32) -> statusCode;
    fn alwindow_create_default_windows();
}

impl Drop for AlWindow {
    fn drop(&mut self) {
        self.destroy();
        self.ptr = std::ptr::null_mut();
    }
}

impl AlObjectMethods for AlWindow {
    fn as_object_ptr(&self) -> *mut AlObject_ptr {
        self.ptr as *mut AlObject_ptr
    }
}
