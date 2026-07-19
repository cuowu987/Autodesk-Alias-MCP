use crate::*;

#[repr(C)]
pub struct AlSurfaceCV_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlSurfaceCV {
    pub ptr: *mut AlSurfaceCV_ptr,
}

impl AlSurfaceCV {
    /// 获取 U 方向的索引
    pub fn u_index(&self) -> i32 {
        unsafe { alsurfacecv_u_index(self.ptr) }
    }

    /// 获取 V 方向的索引
    pub fn v_index(&self) -> i32 {
        unsafe { alsurfacecv_v_index(self.ptr) }
    }

    /// 获取 U 方向的下一个 CV
    pub fn next_in_u(&self) -> Option<AlSurfaceCV> {
        let ptr = unsafe { alsurfacecv_next_in_u(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlSurfaceCV { ptr })
        }
    }

    /// 获取 V 方向的下一个 CV
    pub fn next_in_v(&self) -> Option<AlSurfaceCV> {
        let ptr = unsafe { alsurfacecv_next_in_v(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlSurfaceCV { ptr })
        }
    }

    /// 获取 U 方向的前一个 CV
    pub fn prev_in_u(&self) -> Option<AlSurfaceCV> {
        let ptr = unsafe { alsurfacecv_prev_in_u(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlSurfaceCV { ptr })
        }
    }

    /// 获取 V 方向的前一个 CV
    pub fn prev_in_v(&self) -> Option<AlSurfaceCV> {
        let ptr = unsafe { alsurfacecv_prev_in_v(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlSurfaceCV { ptr })
        }
    }

    /// 删除 U 方向的下一个 CV
    pub fn next_in_u_d(&self) -> Result<(), String> {
        let status = unsafe { alsurfacecv_next_in_u_d(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    /// 删除 V 方向的下一个 CV
    pub fn next_in_v_d(&self) -> Result<(), String> {
        let status = unsafe { alsurfacecv_next_in_v_d(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    /// 删除 U 方向的前一个 CV
    pub fn prev_in_u_d(&self) -> Result<(), String> {
        let status = unsafe { alsurfacecv_prev_in_u_d(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    /// 删除 V 方向的前一个 CV
    pub fn prev_in_v_d(&self) -> Result<(), String> {
        let status = unsafe { alsurfacecv_prev_in_v_d(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    /// 获取 U 方向的重数
    pub fn multiplicity_in_u(&self) -> i32 {
        unsafe { alsurfacecv_multiplicity_in_u(self.ptr) }
    }

    /// 获取 V 方向的重数
    pub fn multiplicity_in_v(&self) -> i32 {
        unsafe { alsurfacecv_multiplicity_in_v(self.ptr) }
    }

    /// 获取世界坐标位置 (4D: u, v, w, weight)
    pub fn world_position(&self) -> Result<[f64; 4], String> {
        let mut u: f64 = 0.0;
        let mut v: f64 = 0.0;
        let mut w: f64 = 0.0;
        let mut weight: f64 = 0.0;
        let status =
            unsafe { alsurfacecv_world_position(self.ptr, &mut u, &mut v, &mut w, &mut weight) };
        if status == statusCode::Success {
            Ok([u, v, w, weight])
        } else {
            Err(status.to_string())
        }
    }

    /// 获取受影响的坐标位置 (4D: u, v, w, weight)
    pub fn affected_position(&self, tm: &AlTM) -> Result<[f64; 4], String> {
        let mut u: f64 = 0.0;
        let mut v: f64 = 0.0;
        let mut w: f64 = 0.0;
        let mut weight: f64 = 0.0;
        let status = unsafe {
            alsurfacecv_affected_position(self.ptr, tm, &mut u, &mut v, &mut w, &mut weight)
        };
        if status == statusCode::Success {
            Ok([u, v, w, weight])
        } else {
            Err(status.to_string())
        }
    }

    /// 获取不受影响的坐标位置 (4D: u, v, w, weight)
    pub fn unaffected_position(&self) -> Result<[f64; 4], String> {
        let mut u: f64 = 0.0;
        let mut v: f64 = 0.0;
        let mut w: f64 = 0.0;
        let mut weight: f64 = 0.0;
        let status = unsafe {
            alsurfacecv_unaffected_position(self.ptr, &mut u, &mut v, &mut w, &mut weight)
        };
        if status == statusCode::Success {
            Ok([u, v, w, weight])
        } else {
            Err(status.to_string())
        }
    }

    /// 设置重数
    pub fn set_multiplicity(&mut self, mult_u: i32, mult_v: i32) -> Result<(), String> {
        let status = unsafe { alsurfacecv_set_multiplicity(self.ptr, mult_u, mult_v) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    /// 设置 4D 世界坐标位置
    pub fn set_world_position_4d(
        &mut self,
        u: f64,
        v: f64,
        w: f64,
        weight: f64,
        unaffected: bool,
    ) -> Result<(), String> {
        let status =
            unsafe { alsurfacecv_set_world_position_4d(self.ptr, u, v, w, weight, unaffected) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    /// 设置 4D 世界坐标位置（带变换矩阵）
    pub fn set_world_position_4d_tm(
        &mut self,
        u: f64,
        v: f64,
        w: f64,
        weight: f64,
        tm: &AlTM,
    ) -> Result<(), String> {
        let status = unsafe { alsurfacecv_set_world_position_4d_tm(self.ptr, u, v, w, weight, tm) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    /// 设置 3D 世界坐标位置
    pub fn set_world_position_3d(
        &mut self,
        u: f64,
        v: f64,
        w: f64,
        unaffected: bool,
    ) -> Result<(), String> {
        let status = unsafe { alsurfacecv_set_world_position_3d(self.ptr, u, v, w, unaffected) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    /// 设置 3D 世界坐标位置（带变换矩阵）
    pub fn set_world_position_3d_tm(
        &mut self,
        u: f64,
        v: f64,
        w: f64,
        tm: &AlTM,
    ) -> Result<(), String> {
        let status = unsafe { alsurfacecv_set_world_position_3d_tm(self.ptr, u, v, w, tm) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    /// 设置不受影响的坐标位置
    pub fn set_unaffected_position(
        &mut self,
        u: f64,
        v: f64,
        w: f64,
        weight: f64,
    ) -> Result<(), String> {
        let status = unsafe { alsurfacecv_set_unaffected_position(self.ptr, u, v, w, weight) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    /// 获取所属表面
    pub fn surface(&self) -> Result<AlSurface, String> {
        let ptr = unsafe { alsurfacecv_surface(self.ptr) };
        if ptr.is_null() {
            Err("surface is null".to_string())
        } else {
            Ok(AlSurface { ptr })
        }
    }

    /// 获取盲数据
    pub fn blind_data(&self, type_: i32) -> Result<(i64, String), String> {
        let mut data: i64 = 0;
        let mut str_ptr: *const i8 = std::ptr::null();
        let status = unsafe { alsurfacecv_blind_data(self.ptr, type_, &mut data, &mut str_ptr) };
        if status == statusCode::Success {
            let str = if str_ptr.is_null() {
                String::new()
            } else {
                unsafe {
                    std::ffi::CStr::from_ptr(str_ptr)
                        .to_string_lossy()
                        .into_owned()
                }
            };
            Ok((data, str))
        } else {
            Err(status.to_string())
        }
    }

    /// 设置盲数据
    pub fn set_blind_data(&mut self, type_: i32, data: i64, str: &str) -> Result<(), String> {
        let c_str = std::ffi::CString::new(str).unwrap();
        let status = unsafe { alsurfacecv_set_blind_data(self.ptr, type_, data, c_str.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    /// 删除盲数据
    pub fn remove_blind_data(&mut self, type_: i32) -> Result<(), String> {
        let status = unsafe { alsurfacecv_remove_blind_data(self.ptr, type_) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    /// 应用迭代器到 U 方向的 CVs
    pub fn apply_iterator_to_cvs_in_u(&self, iter: &AlIterator) -> Result<i32, String> {
        let mut rc: i32 = 0;
        let status = unsafe { alsurfacecv_apply_iterator_to_cvs_in_u(self.ptr, iter.ptr, &mut rc) };
        if status == statusCode::Success {
            Ok(rc)
        } else {
            Err(status.to_string())
        }
    }

    /// 应用迭代器到 V 方向的 CVs
    pub fn apply_iterator_to_cvs_in_v(&self, iter: &AlIterator) -> Result<i32, String> {
        let mut rc: i32 = 0;
        let status = unsafe { alsurfacecv_apply_iterator_to_cvs_in_v(self.ptr, iter.ptr, &mut rc) };
        if status == statusCode::Success {
            Ok(rc)
        } else {
            Err(status.to_string())
        }
    }

    /// 设置更新状态
    pub fn do_updates(&mut self, new_state: bool) -> Result<(), String> {
        let status = unsafe { alsurfacecv_do_updates(self.ptr, new_state) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }
}

impl AlObjectMethods for AlSurfaceCV {
    fn as_object_ptr(&self) -> *mut AlObject_ptr {
        self.ptr as *mut AlObject_ptr
    }

    fn name_ex(&self) -> String {
        let name = self.name();
        if !name.is_empty() {
            return name;
        }
        let surface = self.surface().unwrap();
        let surfacenode = surface.surface_node().unwrap();
        format!("{}_{:?}_cv[{},{}]", surfacenode.name(), self.type_(), self.u_index(), self.v_index())
    }
}

impl AlPickableMethods for AlSurfaceCV {
    fn as_pickable_ptr(&self) -> *mut AlPickable_ptr {
        self.ptr as *mut AlPickable_ptr
    }
}

unsafe extern "C" {
    fn alsurfacecv_u_index(cv: *mut AlSurfaceCV_ptr) -> i32;
    fn alsurfacecv_v_index(cv: *mut AlSurfaceCV_ptr) -> i32;

    fn alsurfacecv_next_in_u(cv: *mut AlSurfaceCV_ptr) -> *mut AlSurfaceCV_ptr;
    fn alsurfacecv_next_in_v(cv: *mut AlSurfaceCV_ptr) -> *mut AlSurfaceCV_ptr;
    fn alsurfacecv_prev_in_u(cv: *mut AlSurfaceCV_ptr) -> *mut AlSurfaceCV_ptr;
    fn alsurfacecv_prev_in_v(cv: *mut AlSurfaceCV_ptr) -> *mut AlSurfaceCV_ptr;

    fn alsurfacecv_next_in_u_d(cv: *mut AlSurfaceCV_ptr) -> statusCode;
    fn alsurfacecv_next_in_v_d(cv: *mut AlSurfaceCV_ptr) -> statusCode;
    fn alsurfacecv_prev_in_u_d(cv: *mut AlSurfaceCV_ptr) -> statusCode;
    fn alsurfacecv_prev_in_v_d(cv: *mut AlSurfaceCV_ptr) -> statusCode;

    fn alsurfacecv_multiplicity_in_u(cv: *mut AlSurfaceCV_ptr) -> i32;
    fn alsurfacecv_multiplicity_in_v(cv: *mut AlSurfaceCV_ptr) -> i32;
    fn alsurfacecv_world_position(
        cv: *mut AlSurfaceCV_ptr,
        u: *mut f64,
        v: *mut f64,
        w: *mut f64,
        weight: *mut f64,
    ) -> statusCode;
    fn alsurfacecv_affected_position(
        cv: *mut AlSurfaceCV_ptr,
        tm: *const AlTM,
        u: *mut f64,
        v: *mut f64,
        w: *mut f64,
        weight: *mut f64,
    ) -> statusCode;
    fn alsurfacecv_unaffected_position(
        cv: *mut AlSurfaceCV_ptr,
        u: *mut f64,
        v: *mut f64,
        w: *mut f64,
        weight: *mut f64,
    ) -> statusCode;

    fn alsurfacecv_set_multiplicity(
        cv: *mut AlSurfaceCV_ptr,
        mult_u: i32,
        mult_v: i32,
    ) -> statusCode;
    fn alsurfacecv_set_world_position_4d(
        cv: *mut AlSurfaceCV_ptr,
        u: f64,
        v: f64,
        w: f64,
        weight: f64,
        unaffected: bool,
    ) -> statusCode;
    fn alsurfacecv_set_world_position_4d_tm(
        cv: *mut AlSurfaceCV_ptr,
        u: f64,
        v: f64,
        w: f64,
        weight: f64,
        tm: *const AlTM,
    ) -> statusCode;
    fn alsurfacecv_set_world_position_3d(
        cv: *mut AlSurfaceCV_ptr,
        u: f64,
        v: f64,
        w: f64,
        unaffected: bool,
    ) -> statusCode;
    fn alsurfacecv_set_world_position_3d_tm(
        cv: *mut AlSurfaceCV_ptr,
        u: f64,
        v: f64,
        w: f64,
        tm: *const AlTM,
    ) -> statusCode;
    fn alsurfacecv_set_unaffected_position(
        cv: *mut AlSurfaceCV_ptr,
        u: f64,
        v: f64,
        w: f64,
        weight: f64,
    ) -> statusCode;

    fn alsurfacecv_surface(cv: *mut AlSurfaceCV_ptr) -> *mut super::alsurface::AlSurface_ptr;

    fn alsurfacecv_blind_data(
        cv: *mut AlSurfaceCV_ptr,
        type_: i32,
        data: *mut i64,
        str: *mut *const i8,
    ) -> statusCode;
    fn alsurfacecv_set_blind_data(
        cv: *mut AlSurfaceCV_ptr,
        type_: i32,
        data: i64,
        str: *const i8,
    ) -> statusCode;
    fn alsurfacecv_remove_blind_data(cv: *mut AlSurfaceCV_ptr, type_: i32) -> statusCode;

    fn alsurfacecv_apply_iterator_to_cvs_in_u(
        cv: *mut AlSurfaceCV_ptr,
        iter: *mut AlIterator_ptr,
        rc: *mut i32,
    ) -> statusCode;
    fn alsurfacecv_apply_iterator_to_cvs_in_v(
        cv: *mut AlSurfaceCV_ptr,
        iter: *mut AlIterator_ptr,
        rc: *mut i32,
    ) -> statusCode;

    fn alsurfacecv_do_updates(cv: *mut AlSurfaceCV_ptr, new_state: bool) -> statusCode;
}
