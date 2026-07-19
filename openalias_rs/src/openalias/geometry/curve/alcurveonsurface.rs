use crate::*;

#[repr(C)]
pub struct AlCurveOnSurface_ptr {
    _private: [u8; 0],
}

#[derive(Debug)]
pub struct AlCurveOnSurface {
    pub ptr: *mut AlCurveOnSurface_ptr,
}

impl AlCurveOnSurface {
    pub fn new() -> AlCurveOnSurface {
        let ptr = unsafe { alcurveonsurface_new() };
        AlCurveOnSurface { ptr }
    }

    pub fn create(
        &self,
        num_cvs: i32,
        form: i32,
        degree: i32,
        knots: &[f64],
        cv_data: &[[f64; 4]],
    ) -> Result<(), String> {
        let status = unsafe {
            alcurveonsurface_create(
                self.ptr,
                num_cvs,
                form,
                degree,
                knots.as_ptr(),
                cv_data.len() as i32,
                cv_data.as_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_from_curves(&self, curves: &[&AlCurve]) -> Result<(), String> {
        let curve_ptrs: Vec<*mut AlCurve_ptr> = curves.iter().map(|c| c.ptr).collect();
        let status = unsafe {
            alcurveonsurface_create_from_curves(self.ptr, curves.len() as i32, curve_ptrs.as_ptr())
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_from_curve(&self, curve: &AlCurve) -> Result<(), String> {
        let status = unsafe { alcurveonsurface_create_from_curve(self.ptr, curve.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn curve_on_surface_data(&self, knots: &mut [f64], cv_data: &mut [[f64; 4]]) -> Result<(), String> {
        let status = unsafe {
            alcurveonsurface_curve_on_surface_data(self.ptr, knots.as_mut_ptr(), cv_data.as_mut_ptr())
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn real_curve_on_surface_data(&self, knots: &mut [f64], cv_data: &mut [[f64; 4]]) -> Result<(), String> {
        let status = unsafe {
            alcurveonsurface_real_curve_on_surface_data(self.ptr, knots.as_mut_ptr(), cv_data.as_mut_ptr())
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn degree(&self) -> i32 {
        unsafe { alcurveonsurface_degree(self.ptr) }
    }

    pub fn form(&self) -> i32 {
        unsafe { alcurveonsurface_form(self.ptr) }
    }

    pub fn number_of_spans(&self) -> i32 {
        unsafe { alcurveonsurface_number_of_spans(self.ptr) }
    }

    pub fn number_of_knots(&self) -> i32 {
        unsafe { alcurveonsurface_number_of_knots(self.ptr) }
    }

    pub fn number_of_control_points(&self) -> i32 {
        unsafe { alcurveonsurface_number_of_control_points(self.ptr) }
    }

    pub fn real_number_of_knots(&self) -> i32 {
        unsafe { alcurveonsurface_real_number_of_knots(self.ptr) }
    }

    pub fn knot_value(&self, index: i32) -> f64 {
        unsafe { alcurveonsurface_knot_value(self.ptr, index) }
    }

    pub fn control_point(&self, index: i32, point: &mut [f64; 4]) -> Result<(), String> {
        let status = unsafe { alcurveonsurface_control_point(self.ptr, index, point.as_mut_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_knot_value(&self, index: i32, value: f64) -> Result<(), String> {
        let status = unsafe { alcurveonsurface_set_knot_value(self.ptr, index, value) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_control_point(&self, index: i32, point: &[f64; 4]) -> Result<(), String> {
        let status = unsafe { alcurveonsurface_set_control_point(self.ptr, index, point.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn in_trim(&self) -> bool {
        unsafe { alcurveonsurface_in_trim(self.ptr) }
    }

    pub fn visible(&self) -> bool {
        unsafe { alcurveonsurface_visible(self.ptr) }
    }

    pub fn set_visible(&self, visible: bool) -> Result<(), String> {
        let status = unsafe { alcurveonsurface_set_visible(self.ptr, visible) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn insert(&self, param: f64) -> Result<(), String> {
        let status = unsafe { alcurveonsurface_insert(self.ptr, param) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn reverse(&self) -> Result<(), String> {
        let status = unsafe { alcurveonsurface_reverse(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn surface(&self) -> Option<AlSurface> {
        let ptr = unsafe { alcurveonsurface_surface(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlSurface { ptr })
        }
    }

    pub fn world_space_3d_copies(&self) -> Result<Vec<AlCurve>, String> {
        let mut num_curves: i32 = 0;
        let mut curves: *mut *mut AlCurve_ptr = std::ptr::null_mut();
        
        let status = unsafe { alcurveonsurface_world_space_3d_copies(self.ptr, &mut num_curves, &mut curves) };
        if status != statusCode::Success {
            return Err(status.to_string());
        }

        let mut result = Vec::with_capacity(num_curves as usize);
        for i in 0..num_curves {
            let curve_ptr = unsafe { *curves.offset(i as isize) };
            if !curve_ptr.is_null() {
                result.push(AlCurve { ptr: curve_ptr });
            }
        }

        Ok(result)
    }

    pub fn unaffected_3d_copies(&self) -> Result<Vec<AlCurve>, String> {
        let mut num_curves: i32 = 0;
        let mut curves: *mut *mut AlCurve_ptr = std::ptr::null_mut();
        
        let status = unsafe { alcurveonsurface_unaffected_3d_copies(self.ptr, &mut num_curves, &mut curves) };
        if status != statusCode::Success {
            return Err(status.to_string());
        }

        let mut result = Vec::with_capacity(num_curves as usize);
        for i in 0..num_curves {
            let curve_ptr = unsafe { *curves.offset(i as isize) };
            if !curve_ptr.is_null() {
                result.push(AlCurve { ptr: curve_ptr });
            }
        }

        Ok(result)
    }

    pub fn affected_3d_copies(&self, tm: &mut AlTM) -> Result<Vec<AlCurve>, String> {
        let mut num_curves: i32 = 0;
        let mut curves: *mut *mut AlCurve_ptr = std::ptr::null_mut();
        
        let status = unsafe { alcurveonsurface_affected_3d_copies(self.ptr, tm, &mut num_curves, &mut curves) };
        if status != statusCode::Success {
            return Err(status.to_string());
        }

        let mut result = Vec::with_capacity(num_curves as usize);
        for i in 0..num_curves {
            let curve_ptr = unsafe { *curves.offset(i as isize) };
            if !curve_ptr.is_null() {
                result.push(AlCurve { ptr: curve_ptr });
            }
        }

        Ok(result)
    }

    pub fn next_curve_on_surface(&self) -> Option<AlCurveOnSurface> {
        let ptr = unsafe { alcurveonsurface_next_curve_on_surface(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlCurveOnSurface { ptr })
        }
    }

    pub fn prev_curve_on_surface(&self) -> Option<AlCurveOnSurface> {
        let ptr = unsafe { alcurveonsurface_prev_curve_on_surface(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlCurveOnSurface { ptr })
        }
    }

    pub fn next_curve_on_surface_d(&self) -> Result<(), String> {
        let status = unsafe { alcurveonsurface_next_curve_on_surface_d(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn prev_curve_on_surface_d(&self) -> Result<(), String> {
        let status = unsafe { alcurveonsurface_prev_curve_on_surface_d(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn persistent_id(&self, ut: i32) -> Result<AlPersistentID, String> {
        let mut id: *mut AlPersistentID = std::ptr::null_mut();
        let status = unsafe { alcurveonsurface_persistent_id(self.ptr, &mut id, ut) };
        if status == statusCode::Success && !id.is_null() {
            Ok(unsafe { *id })
        } else {
            Err(status.to_string())
        }
    }

    pub fn has_persistent_id(&self, ut: i32) -> Result<bool, String> {
        let status = unsafe { alcurveonsurface_has_persistent_id(self.ptr, ut) };
        if status == statusCode::Success {
            Ok(true)
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_persistent_id(&self, id: &AlPersistentID, ut: i32) -> Result<(), String> {
        let status = unsafe { alcurveonsurface_set_persistent_id(self.ptr, id, ut) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn blind_data(&self, tag: i32) -> Result<(i64, String), String> {
        let mut value: i64 = 0;
        let mut data: *const i8 = std::ptr::null();
        let status = unsafe { alcurveonsurface_blind_data(self.ptr, tag, &mut value, &mut data) };
        if status == statusCode::Success {
            let data_str = if data.is_null() {
                String::new()
            } else {
                unsafe { std::ffi::CStr::from_ptr(data).to_string_lossy().to_string() }
            };
            Ok((value, data_str))
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_blind_data(&self, tag: i32, value: i64, data: &str) -> Result<(), String> {
        let c_data = std::ffi::CString::new(data).unwrap();
        let status = unsafe { alcurveonsurface_set_blind_data(self.ptr, tag, value, c_data.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn remove_blind_data(&self, tag: i32) -> Result<(), String> {
        let status = unsafe { alcurveonsurface_remove_blind_data(self.ptr, tag) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }
}

impl Drop for AlCurveOnSurface {
    fn drop(&mut self) {
        unsafe {
            alcurveonsurface_delete(self.ptr);
            self.ptr = std::ptr::null_mut();
        }
    }
}
impl AlObjectMethods for AlCurveOnSurface {
    fn as_object_ptr(&self) -> *mut AlObject_ptr {
        self.ptr as *mut AlObject_ptr
    }
}

unsafe extern "C" {
    fn alcurveonsurface_new() -> *mut AlCurveOnSurface_ptr;
    fn alcurveonsurface_delete(cos: *mut AlCurveOnSurface_ptr);
    
    fn alcurveonsurface_create(cos: *mut AlCurveOnSurface_ptr, num_cvs: i32, form: i32, degree: i32, knots: *const f64, num_cvs_data: i32, cv_data: *const [f64; 4]) -> statusCode;
    fn alcurveonsurface_create_from_curves(cos: *mut AlCurveOnSurface_ptr, num_curves: i32, curves: *const *mut AlCurve_ptr) -> statusCode;
    fn alcurveonsurface_create_from_curve(cos: *mut AlCurveOnSurface_ptr, curve: *mut AlCurve_ptr) -> statusCode;
    
    fn alcurveonsurface_curve_on_surface_data(cos: *mut AlCurveOnSurface_ptr, knots: *mut f64, cv_data: *mut [f64; 4]) -> statusCode;
    fn alcurveonsurface_real_curve_on_surface_data(cos: *mut AlCurveOnSurface_ptr, knots: *mut f64, cv_data: *mut [f64; 4]) -> statusCode;
    
    fn alcurveonsurface_degree(cos: *mut AlCurveOnSurface_ptr) -> i32;
    fn alcurveonsurface_form(cos: *mut AlCurveOnSurface_ptr) -> i32;
    fn alcurveonsurface_number_of_spans(cos: *mut AlCurveOnSurface_ptr) -> i32;
    fn alcurveonsurface_number_of_knots(cos: *mut AlCurveOnSurface_ptr) -> i32;
    fn alcurveonsurface_number_of_control_points(cos: *mut AlCurveOnSurface_ptr) -> i32;
    fn alcurveonsurface_real_number_of_knots(cos: *mut AlCurveOnSurface_ptr) -> i32;
    
    fn alcurveonsurface_knot_value(cos: *mut AlCurveOnSurface_ptr, index: i32) -> f64;
    fn alcurveonsurface_control_point(cos: *mut AlCurveOnSurface_ptr, index: i32, point: *mut f64) -> statusCode;
    
    fn alcurveonsurface_set_knot_value(cos: *mut AlCurveOnSurface_ptr, index: i32, value: f64) -> statusCode;
    fn alcurveonsurface_set_control_point(cos: *mut AlCurveOnSurface_ptr, index: i32, point: *const f64) -> statusCode;
    
    fn alcurveonsurface_in_trim(cos: *mut AlCurveOnSurface_ptr) -> bool;
    fn alcurveonsurface_visible(cos: *mut AlCurveOnSurface_ptr) -> bool;
    fn alcurveonsurface_set_visible(cos: *mut AlCurveOnSurface_ptr, visible: bool) -> statusCode;
    
    fn alcurveonsurface_insert(cos: *mut AlCurveOnSurface_ptr, param: f64) -> statusCode;
    fn alcurveonsurface_reverse(cos: *mut AlCurveOnSurface_ptr) -> statusCode;
    
    fn alcurveonsurface_surface(cos: *mut AlCurveOnSurface_ptr) -> *mut AlSurface_ptr;
    
    fn alcurveonsurface_world_space_3d_copies(cos: *mut AlCurveOnSurface_ptr, out_num_curves: *mut i32, out_curves: *mut *mut *mut AlCurve_ptr) -> statusCode;
    fn alcurveonsurface_unaffected_3d_copies(cos: *mut AlCurveOnSurface_ptr, out_num_curves: *mut i32, out_curves: *mut *mut *mut AlCurve_ptr) -> statusCode;
    fn alcurveonsurface_affected_3d_copies(cos: *mut AlCurveOnSurface_ptr, tm: *mut AlTM, out_num_curves: *mut i32, out_curves: *mut *mut *mut AlCurve_ptr) -> statusCode;
    
    fn alcurveonsurface_next_curve_on_surface(cos: *mut AlCurveOnSurface_ptr) -> *mut AlCurveOnSurface_ptr;
    fn alcurveonsurface_prev_curve_on_surface(cos: *mut AlCurveOnSurface_ptr) -> *mut AlCurveOnSurface_ptr;
    
    fn alcurveonsurface_next_curve_on_surface_d(cos: *mut AlCurveOnSurface_ptr) -> statusCode;
    fn alcurveonsurface_prev_curve_on_surface_d(cos: *mut AlCurveOnSurface_ptr) -> statusCode;
    
    fn alcurveonsurface_persistent_id(cos: *mut AlCurveOnSurface_ptr, out_id: *mut *mut AlPersistentID, ut: i32) -> statusCode;
    fn alcurveonsurface_has_persistent_id(cos: *mut AlCurveOnSurface_ptr, ut: i32) -> statusCode;
    fn alcurveonsurface_set_persistent_id(cos: *mut AlCurveOnSurface_ptr, id: *const AlPersistentID, ut: i32) -> statusCode;
    
    fn alcurveonsurface_blind_data(cos: *mut AlCurveOnSurface_ptr, tag: i32, out_value: *mut i64, out_data: *mut *const i8) -> statusCode;
    fn alcurveonsurface_set_blind_data(cos: *mut AlCurveOnSurface_ptr, tag: i32, value: i64, data: *const i8) -> statusCode;
    fn alcurveonsurface_remove_blind_data(cos: *mut AlCurveOnSurface_ptr, tag: i32) -> statusCode;
}
