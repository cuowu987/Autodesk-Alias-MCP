use crate::*;

#[repr(C)]
pub struct AlCurveCV_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlCurveCV {
    pub ptr: *mut AlCurveCV_ptr,
}

impl AlCurveCV {
    /// 获取下一个 CV
    pub fn next(&self) -> Option<AlCurveCV> {
        let ptr = unsafe { alcurvecv_next(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlCurveCV { ptr })
        }
    }

    /// 获取前一个 CV
    pub fn prev(&self) -> Option<AlCurveCV> {
        let ptr = unsafe { alcurvecv_prev(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlCurveCV { ptr })
        }
    }

    /// 删除下一个 CV
    pub fn next_d(&self) -> Result<(), String> {
        let status = unsafe { alcurvecv_next_d(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    /// 删除前一个 CV
    pub fn prev_d(&self) -> Result<(), String> {
        let status = unsafe { alcurvecv_prev_d(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    /// 获取索引
    pub fn index(&self) -> i32 {
        unsafe { alcurvecv_index(self.ptr) }
    }

    /// 获取重数
    pub fn multiplicity(&self) -> i32 {
        unsafe { alcurvecv_multiplicity(self.ptr) }
    }

    /// 获取世界坐标位置 (4D: x, y, z, weight)
    pub fn world_position(&self) -> Result<[f64; 4], String> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;
        let mut weight: f64 = 0.0;
        let status =
            unsafe { alcurvecv_world_position(self.ptr, &mut x, &mut y, &mut z, &mut weight) };
        if status == statusCode::Success {
            Ok([x, y, z, weight])
        } else {
            Err(status.to_string())
        }
    }

    /// 获取受影响的坐标位置 (4D: x, y, z, weight)
    pub fn affected_position(&self, tm: &AlTM) -> Result<[f64; 4], String> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;
        let mut weight: f64 = 0.0;
        let status = unsafe {
            alcurvecv_affected_position(self.ptr, tm, &mut x, &mut y, &mut z, &mut weight)
        };
        if status == statusCode::Success {
            Ok([x, y, z, weight])
        } else {
            Err(status.to_string())
        }
    }

    /// 获取不受影响的坐标位置 (4D: x, y, z, weight)
    pub fn unaffected_position(&self) -> Result<[f64; 4], String> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;
        let mut weight: f64 = 0.0;
        let status =
            unsafe { alcurvecv_unaffected_position(self.ptr, &mut x, &mut y, &mut z, &mut weight) };
        if status == statusCode::Success {
            Ok([x, y, z, weight])
        } else {
            Err(status.to_string())
        }
    }

    /// 设置重数
    pub fn set_multiplicity(&mut self, mult: i32) -> Result<(), String> {
        let status = unsafe { alcurvecv_set_multiplicity(self.ptr, mult) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    /// 设置 4D 世界坐标位置
    pub fn set_world_position_4d(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        weight: f64,
        includeWorld: bool,
    ) -> Result<(), String> {
        let status =
            unsafe { alcurvecv_set_world_position_4d(self.ptr, x, y, z, weight, includeWorld) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    /// 设置 4D 世界坐标位置（带变换矩阵）
    pub fn set_world_position_4d_tm(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        weight: f64,
        tm: &AlTM,
    ) -> Result<(), String> {
        let status = unsafe { alcurvecv_set_world_position_4d_tm(self.ptr, x, y, z, weight, tm) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    /// 设置 3D 世界坐标位置
    pub fn set_world_position_3d(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        includeWorld: bool,
    ) -> Result<(), String> {
        let status = unsafe { alcurvecv_set_world_position_3d(self.ptr, x, y, z, includeWorld) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    /// 设置 3D 世界坐标位置（带变换矩阵）
    pub fn set_world_position_3d_tm(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        tm: &AlTM,
    ) -> Result<(), String> {
        let status = unsafe { alcurvecv_set_world_position_3d_tm(self.ptr, x, y, z, tm) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    /// 设置不受影响的坐标位置
    pub fn set_unaffected_position(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        weight: f64,
    ) -> Result<(), String> {
        let status = unsafe { alcurvecv_set_unaffected_position(self.ptr, x, y, z, weight) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    /// 获取所属曲线
    pub fn curve(&self) -> Result<AlCurve, String> {
        let ptr = unsafe { alcurvecv_curve(self.ptr) };
        if ptr.is_null() {
            Err("curve is not valid".to_string())
        } else {
            Ok(AlCurve { ptr })
        }
    }

    /// 获取盲数据
    pub fn blind_data(&self, type_: i32) -> Result<(i64, String), String> {
        let mut data: i64 = 0;
        let mut str_ptr: *const i8 = std::ptr::null();
        let status = unsafe { alcurvecv_blind_data(self.ptr, type_, &mut data, &mut str_ptr) };
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
        let status = unsafe { alcurvecv_set_blind_data(self.ptr, type_, data, c_str.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    /// 删除盲数据
    pub fn remove_blind_data(&mut self, type_: i32) -> Result<(), String> {
        let status = unsafe { alcurvecv_remove_blind_data(self.ptr, type_) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    /// 设置更新状态
    pub fn do_updates(&mut self, new_state: bool) -> Result<(), String> {
        let status = unsafe { alcurvecv_do_updates(self.ptr, new_state) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }
}

impl AlObjectMethods for AlCurveCV {
    fn as_object_ptr(&self) -> *mut AlObject_ptr {
        self.ptr as *mut AlObject_ptr
    }

    fn name_ex(&self) -> String {
        let name = self.name();
        if !name.is_empty() {
            return name;
        }
        let node = self.curve().unwrap().curve_node().unwrap();
        format!("{}_{:?}_cv[{}]", node.name(), self.type_(), self.index())
    }
}

impl AlPickableMethods for AlCurveCV {
    fn as_pickable_ptr(&self) -> *mut AlPickable_ptr {
        self.ptr as *mut AlPickable_ptr
    }
}

unsafe extern "C" {
    fn alcurvecv_next(cv: *mut AlCurveCV_ptr) -> *mut AlCurveCV_ptr;
    fn alcurvecv_prev(cv: *mut AlCurveCV_ptr) -> *mut AlCurveCV_ptr;
    fn alcurvecv_next_d(cv: *mut AlCurveCV_ptr) -> statusCode;
    fn alcurvecv_prev_d(cv: *mut AlCurveCV_ptr) -> statusCode;

    fn alcurvecv_index(cv: *mut AlCurveCV_ptr) -> i32;
    fn alcurvecv_multiplicity(cv: *mut AlCurveCV_ptr) -> i32;
    fn alcurvecv_world_position(
        cv: *mut AlCurveCV_ptr,
        x: *mut f64,
        y: *mut f64,
        z: *mut f64,
        weight: *mut f64,
    ) -> statusCode;
    fn alcurvecv_affected_position(
        cv: *mut AlCurveCV_ptr,
        tm: *const AlTM,
        x: *mut f64,
        y: *mut f64,
        z: *mut f64,
        weight: *mut f64,
    ) -> statusCode;
    fn alcurvecv_unaffected_position(
        cv: *mut AlCurveCV_ptr,
        x: *mut f64,
        y: *mut f64,
        z: *mut f64,
        weight: *mut f64,
    ) -> statusCode;

    fn alcurvecv_set_multiplicity(cv: *mut AlCurveCV_ptr, mult: i32) -> statusCode;
    fn alcurvecv_set_world_position_4d(
        cv: *mut AlCurveCV_ptr,
        x: f64,
        y: f64,
        z: f64,
        weight: f64,
        unaffected: bool,
    ) -> statusCode;
    fn alcurvecv_set_world_position_4d_tm(
        cv: *mut AlCurveCV_ptr,
        x: f64,
        y: f64,
        z: f64,
        weight: f64,
        tm: *const AlTM,
    ) -> statusCode;
    fn alcurvecv_set_world_position_3d(
        cv: *mut AlCurveCV_ptr,
        x: f64,
        y: f64,
        z: f64,
        unaffected: bool,
    ) -> statusCode;
    fn alcurvecv_set_world_position_3d_tm(
        cv: *mut AlCurveCV_ptr,
        x: f64,
        y: f64,
        z: f64,
        tm: *const AlTM,
    ) -> statusCode;
    fn alcurvecv_set_unaffected_position(
        cv: *mut AlCurveCV_ptr,
        x: f64,
        y: f64,
        z: f64,
        weight: f64,
    ) -> statusCode;

    fn alcurvecv_curve(cv: *mut AlCurveCV_ptr) -> *mut super::alcurve::AlCurve_ptr;

    fn alcurvecv_blind_data(
        cv: *mut AlCurveCV_ptr,
        type_: i32,
        data: *mut i64,
        str: *mut *const i8,
    ) -> statusCode;
    fn alcurvecv_set_blind_data(
        cv: *mut AlCurveCV_ptr,
        type_: i32,
        data: i64,
        str: *const i8,
    ) -> statusCode;
    fn alcurvecv_remove_blind_data(cv: *mut AlCurveCV_ptr, type_: i32) -> statusCode;

    fn alcurvecv_do_updates(cv: *mut AlCurveCV_ptr, new_state: bool) -> statusCode;
}
