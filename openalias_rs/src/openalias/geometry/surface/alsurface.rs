
use crate::*;
use base_geometry_lib::RU_4dPoint;


#[allow(non_camel_case_types)]
#[repr(C)]
pub struct AlSurface_ptr {
    _private: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlSurfaceDirection {
    #[allow(non_camel_case_types)]
    kU = 0,
    #[allow(non_camel_case_types)]
    kV = 1,
}

pub const AL_UNPILE_START_U: i32 = 1;
pub const AL_UNPILE_START_V: i32 = 2;
pub const AL_UNPILE_END_U: i32 = 4;
pub const AL_UNPILE_END_V: i32 = 8;

#[derive(Debug)]
pub struct AlSurface {
    pub ptr: *mut AlSurface_ptr,
}

impl Drop for AlSurface {
    fn drop(&mut self) {
        unsafe {
            alsurface_delete(self.ptr);
            self.ptr = std::ptr::null_mut();
        }
    }
}

impl AlObjectMethods for AlSurface {
    fn as_object_ptr(&self) -> *mut AlObject_ptr {
        self.ptr as *mut _
    }
    fn name_ex(&self) -> String {
        let name = self.name();
        if !name.is_empty() {
            return name;
        }
        let surface_node = self.surface_node().unwrap();
        format!("{}_{:?}", surface_node.name(), self.type_())
    }
}

impl AlSurface {
    pub fn new() -> AlSurface {
        let ptr = unsafe { alsurface_new() };
        AlSurface { ptr }
    }

    pub fn create(
        &mut self,
        u_num_cvs: i32,
        v_num_cvs: i32,
        u_form: i32,
        v_form: i32,
        u_degree: i32,
        v_degree: i32,
        u_knots: &[f64],
        v_knots: &[f64],
        cv_data: &[f64],
        u_mult: &[i32],
        v_mult: &[i32],
    ) -> Result<(), String> {
        let status = unsafe {
            alsurface_create(
                self.ptr,
                u_num_cvs,
                v_num_cvs,
                u_form,
                v_form,
                u_degree,
                v_degree,
                u_knots.as_ptr(),
                v_knots.as_ptr(),
                cv_data.len() as i32 / 4,
                4,
                cv_data.as_ptr(),
                u_mult.as_ptr(),
                v_mult.as_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_1(
        &mut self,
        uDeg: usize,
        vDeg: usize,
        uForm: curveFormType,
        vForm: curveFormType,
        uNumKnots: usize,
        vNumKnots: usize,
        uKnotVector: &Vec<f64>,
        vKnotVector: &Vec<f64>,
        uNumControlPts: usize,
        vNumControlPts: usize,
        controlPoints: &Vec<RU_4dPoint>,
    ) -> Result<&Self, String> {
        unsafe {
            let uKnotVector = uKnotVector.as_ptr();
            let vKnotVector = vKnotVector.as_ptr();
            let mut cvs = Vec::with_capacity(uNumControlPts * vNumControlPts * 4);
            for cv in controlPoints {
                cvs.push(cv.x);
                cvs.push(cv.y);
                cvs.push(cv.z);
                cvs.push(cv.w);
            }
            let uMult = vec![1; uNumControlPts as usize];
            let vMult = vec![1; vNumControlPts as usize];
            let ret = alsurface_create(
                self.ptr,
                uDeg as i32,
                vDeg as i32,
                uForm as i32,
                vForm as i32,
                uNumKnots as i32,
                vNumKnots as i32,
                uKnotVector,
                vKnotVector,
                uNumControlPts as i32,
                vNumControlPts as i32,
                cvs.as_ptr(),
                uMult.as_ptr(),
                vMult.as_ptr(),
            );
            if ret == statusCode::Success {
                Ok(self)
            } else {
                Err("alsurface_create failed".to_string())
            }
        }
    }

    pub fn create_simple(
        &mut self,
        u_num_cvs: i32,
        v_num_cvs: i32,
        u_form: i32,
        v_form: i32,
        u_knots: &[f64],
        v_knots: &[f64],
        cv_data: &[f64],
    ) -> Result<(), String> {
        let status = unsafe {
            alsurface_create_simple(
                &mut self.ptr,
                u_num_cvs,
                v_num_cvs,
                u_form,
                v_form,
                u_knots.as_ptr(),
                v_knots.as_ptr(),
                cv_data.len() as i32 / 4,
                4,
                cv_data.as_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn destroy(&mut self) {
        if !self.ptr.is_null() {
            <Self as AlObjectMethods>::destroy(self);
            self.ptr = std::ptr::null_mut();
        }
    }

    pub fn replace(
        &mut self,
        u_num_cvs: i32,
        v_num_cvs: i32,
        u_form: i32,
        v_form: i32,
        u_degree: i32,
        v_degree: i32,
        u_knots: &[f64],
        v_knots: &[f64],
        cv_data: &[f64],
        u_mult: &[i32],
        v_mult: &[i32],
    ) -> Result<(), String> {
        let status = unsafe {
            alsurface_replace(
                self.ptr,
                u_num_cvs,
                v_num_cvs,
                u_form,
                v_form,
                u_degree,
                v_degree,
                u_knots.as_ptr(),
                v_knots.as_ptr(),
                cv_data.len() as i32 / 4,
                4,
                cv_data.as_ptr(),
                u_mult.as_ptr(),
                v_mult.as_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn replace_simple(
        &mut self,
        u_num_cvs: i32,
        v_num_cvs: i32,
        u_form: i32,
        v_form: i32,
        u_knots: &[f64],
        v_knots: &[f64],
        cv_data: &[f64],
    ) -> Result<(), String> {
        let status = unsafe {
            alsurface_replace_simple(
                self.ptr,
                u_num_cvs,
                v_num_cvs,
                u_form,
                v_form,
                u_knots.as_ptr(),
                v_knots.as_ptr(),
                cv_data.len() as i32 / 4,
                4,
                cv_data.as_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_revolved(
        &mut self,
        axis_start: [f64; 3],
        axis_end: [f64; 3],
        start_angle: f64,
        end_angle: f64,
        curve: &AlCurve,
    ) -> Result<(), String> {
        let status = unsafe {
            alsurface_create_revolved(
                self.ptr,
                axis_start.as_ptr(),
                axis_end.as_ptr(),
                start_angle,
                end_angle,
                curve.ptr,
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_extrusion(
        &mut self,
        direction: [f64; 3],
        length: f64,
        curve: &AlCurve,
    ) -> Result<(), String> {
        let status =
            unsafe { alsurface_create_extrusion(self.ptr, direction.as_ptr(), length, curve.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_torus(
        &mut self,
        origin: [f64; 3],
        direction: [f64; 3],
        major_radius: f64,
        minor_radius: f64,
        zero: [f64; 3],
        rot_start: f64,
        rot_end: f64,
        arc_start: f64,
        arc_end: f64,
    ) -> Result<(), String> {
        let status = unsafe {
            alsurface_create_torus(
                self.ptr,
                origin.as_ptr(),
                direction.as_ptr(),
                major_radius,
                minor_radius,
                zero.as_ptr(),
                rot_start,
                rot_end,
                arc_start,
                arc_end,
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_sphere(&mut self, center: [f64; 3], radius: f64) -> Result<(), String> {
        let status = unsafe { alsurface_create_sphere(self.ptr, center.as_ptr(), radius) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_cylinder(
        &mut self,
        base: [f64; 3],
        apex: [f64; 3],
        radius: f64,
    ) -> Result<(), String> {
        let status =
            unsafe { alsurface_create_cylinder(self.ptr, base.as_ptr(), apex.as_ptr(), radius) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_cone(
        &mut self,
        base: [f64; 3],
        base_radius: f64,
        apex: [f64; 3],
        apex_radius: f64,
    ) -> Result<(), String> {
        let status = unsafe {
            alsurface_create_cone(
                self.ptr,
                base.as_ptr(),
                base_radius,
                apex.as_ptr(),
                apex_radius,
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn surface_node(&self) -> Option<AlSurfaceNode> {
        let ptr = unsafe { alsurface_surface_node(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlSurfaceNode { ptr })
        }
    }

    pub fn u_form(&self) -> i32 {
        unsafe { alsurface_u_form(self.ptr) }
    }

    pub fn v_form(&self) -> i32 {
        unsafe { alsurface_v_form(self.ptr) }
    }

    pub fn u_degree(&self) -> i32 {
        unsafe { alsurface_u_degree(self.ptr) }
    }

    pub fn v_degree(&self) -> i32 {
        unsafe { alsurface_v_degree(self.ptr) }
    }

    pub fn u_num_spans(&self) -> i32 {
        unsafe { alsurface_u_num_spans(self.ptr) }
    }

    pub fn v_num_spans(&self) -> i32 {
        unsafe { alsurface_v_num_spans(self.ptr) }
    }

    pub fn u_num_cvs(&self) -> i32 {
        unsafe { alsurface_u_num_cvs(self.ptr) }
    }

    pub fn v_num_cvs(&self) -> i32 {
        unsafe { alsurface_v_num_cvs(self.ptr) }
    }

    pub fn first_cv(&self) -> Option<AlSurfaceCV> {
        let ptr = unsafe { alsurface_first_cv(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlSurfaceCV { ptr })
        }
    }

    pub fn get_cv(&self, index_u: i32, index_v: i32) -> Option<AlSurfaceCV> {
        let ptr = unsafe { alsurface_get_cv(self.ptr, index_u, index_v) };
        if ptr.is_null() {
            None
        } else {
            Some(AlSurfaceCV { ptr })
        }
    }

    pub fn first_attribute(&self) -> Option<AlAttributes> {
        let ptr = unsafe { alsurface_first_attribute(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlAttributes { ptr })
        }
    }

    pub fn is_construction_history_result(&self) -> bool {
        unsafe { alsurface_is_construction_history_result(self.ptr) }
    }

    pub fn set_cvs_unaffected_position(&mut self, cv_data: &[f64]) -> Result<(), String> {
        let status = unsafe { alsurface_set_cvs_unaffected_position(self.ptr, cv_data.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_u_knot_vector(&mut self, knots: &[f64]) -> Result<(), String> {
        let status = unsafe { alsurface_set_u_knot_vector(self.ptr, knots.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_v_knot_vector(&mut self, knots: &[f64]) -> Result<(), String> {
        let status = unsafe { alsurface_set_v_knot_vector(self.ptr, knots.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_cvs_unaffected_position_incl_multiples(
        &mut self,
        cv_data: &[f64],
    ) -> Result<(), String> {
        let status = unsafe {
            alsurface_set_cvs_unaffected_position_incl_multiples(self.ptr, cv_data.as_ptr())
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_real_u_knot_vector(&mut self, knots: &[f64]) -> Result<(), String> {
        let status = unsafe { alsurface_set_real_u_knot_vector(self.ptr, knots.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_real_v_knot_vector(&mut self, knots: &[f64]) -> Result<(), String> {
        let status = unsafe { alsurface_set_real_v_knot_vector(self.ptr, knots.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn unpile_end_knots(&mut self, flags: i32, values: &[f64]) -> Result<(), String> {
        let status = unsafe { alsurface_unpile_end_knots(self.ptr, flags, values.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn is_point_active(&self, u: f64, v: f64) -> bool {
        unsafe { alsurface_is_point_active(self.ptr, u, v) }
    }

    pub fn cvs_world_position(
        &self,
        // cv_data: &mut [f64],
        // u_mult: &mut [i32],
        // v_mult: &mut [i32],
    ) -> Result<(Vec<[f64; 4]>, Vec<i32>, Vec<i32>), String> {
        let u_cv_num = self.u_num_cvs() as usize;
        let v_cv_num = self.v_num_cvs() as usize;
        let total_cvs = u_cv_num * v_cv_num;
        let mut cv_data = vec![[0.0, 0.0, 0.0, 0.0]; total_cvs];
        let mut u_mult = vec![0; u_cv_num];
        let mut v_mult = vec![0; v_cv_num];

        let status = unsafe {
            alsurface_cvs_world_position(
                self.ptr,
                cv_data.as_mut_ptr() as *mut f64,
                u_mult.as_mut_ptr(),
                v_mult.as_mut_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok((cv_data, u_mult, v_mult))
        } else {
            Err(status.to_string())
        }
    }

    pub fn cvs_affected_position(
        &self,
        tm: &AlTM,
    ) -> Result<(Vec<[f64; 4]>, Vec<i32>, Vec<i32>), String> {
        let u_cv_num = self.u_num_cvs() as usize;
        let v_cv_num = self.v_num_cvs() as usize;
        let total_cvs = u_cv_num * v_cv_num;
        let mut cv_data = vec![[0.0, 0.0, 0.0, 0.0]; total_cvs];
        let mut u_mult = vec![0; u_cv_num];
        let mut v_mult = vec![0; v_cv_num];
        let status = unsafe {
            alsurface_cvs_affected_position(
                self.ptr,
                tm,
                cv_data.as_mut_ptr() as *mut f64,
                u_mult.as_mut_ptr(),
                v_mult.as_mut_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok((cv_data, u_mult, v_mult))
        } else {
            Err(status.to_string())
        }
    }

    pub fn cvs_unaffected_position(&self) -> Result<(Vec<[f64; 4]>, Vec<i32>, Vec<i32>), String> {
        let u_cv_num = self.u_num_cvs() as usize;
        let v_cv_num = self.v_num_cvs() as usize;
        let total_cvs = u_cv_num * v_cv_num;
        let mut cv_data = vec![[0.0, 0.0, 0.0, 0.0]; total_cvs];
        let mut u_mult = vec![0; u_cv_num];
        let mut v_mult = vec![0; v_cv_num];

        let status = unsafe {
            alsurface_cvs_unaffected_position(
                self.ptr,
                cv_data.as_mut_ptr() as *mut f64,
                u_mult.as_mut_ptr(),
                v_mult.as_mut_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok((cv_data, u_mult, v_mult))
        } else {
            Err(status.to_string())
        }
    }

    pub fn u_num_knots(&self) -> i32 {
        unsafe { alsurface_u_num_knots(self.ptr) }
    }

    pub fn v_num_knots(&self) -> i32 {
        unsafe { alsurface_v_num_knots(self.ptr) }
    }

    pub fn u_knot_vector(&self) -> Result<Vec<f64>, String> {
        let u_knot_num = self.u_num_knots() as usize;
        let mut knots = vec![0.0; u_knot_num];

        let status = unsafe { alsurface_u_knot_vector(self.ptr, knots.as_mut_ptr()) };
        if status == statusCode::Success {
            Ok(knots)
        } else {
            Err(status.to_string())
        }
    }

    pub fn v_knot_vector(&self) -> Result<Vec<f64>, String> {
        let v_knot_num = self.v_num_knots() as usize;
        let mut knots = vec![0.0; v_knot_num];

        let status = unsafe { alsurface_v_knot_vector(self.ptr, knots.as_mut_ptr()) };
        if status == statusCode::Success {
            Ok(knots)
        } else {
            Err(status.to_string())
        }
    }

    pub fn u_num_cvs_incl_multiples(&self) -> i32 {
        unsafe { alsurface_u_num_cvs_incl_multiples(self.ptr) }
    }

    pub fn v_num_cvs_incl_multiples(&self) -> i32 {
        unsafe { alsurface_v_num_cvs_incl_multiples(self.ptr) }
    }

    pub fn cvs_world_position_incl_multiples(&self, cv_data: &mut [f64]) -> Result<(), String> {
        let status =
            unsafe { alsurface_cvs_world_position_incl_multiples(self.ptr, cv_data.as_mut_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn cvs_affected_position_incl_multiples(
        &self,
        tm: &AlTM,
        cv_data: &mut [f64],
    ) -> Result<(), String> {
        let status = unsafe {
            alsurface_cvs_affected_position_incl_multiples(self.ptr, tm, cv_data.as_mut_ptr())
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn cvs_unaffected_position_incl_multiples(
        &self,
        cv_data: &mut [f64],
    ) -> Result<(), String> {
        let status = unsafe {
            alsurface_cvs_unaffected_position_incl_multiples(self.ptr, cv_data.as_mut_ptr())
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn real_u_num_knots(&self) -> i32 {
        unsafe { alsurface_real_u_num_knots(self.ptr) }
    }

    pub fn real_v_num_knots(&self) -> i32 {
        unsafe { alsurface_real_v_num_knots(self.ptr) }
    }

    pub fn real_u_knot_vector(&self, knots: &mut [f64]) -> Result<(), String> {
        let status = unsafe { alsurface_real_u_knot_vector(self.ptr, knots.as_mut_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn real_v_knot_vector(&self, knots: &mut [f64]) -> Result<(), String> {
        let status = unsafe { alsurface_real_v_knot_vector(self.ptr, knots.as_mut_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn area(&self, world_coords: Option<bool>, tolerance: Option<f64>) -> Result<f64, String> {
        let world_coords = world_coords.unwrap_or(true);
        let tolerance = tolerance.unwrap_or(0.001);
        let mut out_area: f64 = 0.0;
        let status = unsafe { alsurface_area(self.ptr, &mut out_area, world_coords, tolerance) };
        if status == statusCode::Success {
            Ok(out_area)
        } else {
            Err(status.to_string())
        }
    }

    pub fn circumference(&self, world_coords: bool, tolerance: f64) -> Result<f64, String> {
        let mut out_circum: f64 = 0.0;
        let status =
            unsafe { alsurface_circumference(self.ptr, &mut out_circum, world_coords, tolerance) };
        if status == statusCode::Success {
            Ok(out_circum)
        } else {
            Err(status.to_string())
        }
    }

    pub fn eval(
        &self,
        u: f64,
        v: f64,
        world_coords: bool,
        p: &mut [f64; 3],
        pu: Option<&mut [f64; 3]>,
        pv: Option<&mut [f64; 3]>,
        n: Option<&mut [f64; 3]>,
        compute_derivs: bool,
        normalize_normal: bool,
    ) -> Result<(), String> {
        let pu_ptr = pu.map_or(std::ptr::null_mut(), |x| x.as_mut_ptr());
        let pv_ptr = pv.map_or(std::ptr::null_mut(), |x| x.as_mut_ptr());
        let n_ptr = n.map_or(std::ptr::null_mut(), |x| x.as_mut_ptr());
        let status = unsafe {
            alsurface_eval(
                self.ptr,
                u,
                v,
                world_coords,
                p.as_mut_ptr(),
                pu_ptr,
                pv_ptr,
                n_ptr,
                compute_derivs,
                normalize_normal,
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn first_shader(&self) -> Option<AlShader> {
        let ptr = unsafe { alsurface_first_shader(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlShader { ptr })
        }
    }

    pub fn next_shader(&self, prev: &AlShader) -> Option<AlShader> {
        let ptr = unsafe { alsurface_next_shader(self.ptr, prev.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlShader { ptr })
        }
    }

    pub fn get_switch_shader(&self) -> Option<AlSwitchShader> {
        let ptr = unsafe { alsurface_get_switch_shader(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlSwitchShader { ptr })
        }
    }

    pub fn get_layered_shader(&self) -> Option<AlLayeredShader> {
        let ptr = unsafe { alsurface_get_layered_shader(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlLayeredShader { ptr })
        }
    }

    pub fn next_shader_d(&self, shader: &mut AlShader) -> Result<(), String> {
        let status = unsafe { alsurface_next_shader_d(self.ptr, shader.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn assign_shader(&mut self, shader: &AlShader) -> Result<(), String> {
        let status = unsafe { alsurface_assign_shader(self.ptr, shader.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn assign_switch_shader(&mut self, shader: &AlSwitchShader) -> Result<(), String> {
        let status = unsafe { alsurface_assign_switch_shader(self.ptr, shader.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn layer_shader(&mut self, shader: &AlShader) -> Result<(), String> {
        let status = unsafe { alsurface_layer_shader(self.ptr, shader.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn render_info(&self, render_info: &mut AlRenderInfo) -> Result<(), String> {
        let status = unsafe { alsurface_render_info(self.ptr, render_info) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_render_info(&mut self, render_info: &AlRenderInfo) -> Result<(), String> {
        let status = unsafe { alsurface_set_render_info(self.ptr, render_info) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn trimmed(&self) -> bool {
        unsafe { alsurface_trimmed(self.ptr) }
    }

    pub fn is_target_surface(&self) -> bool {
        unsafe { alsurface_is_target_surface(self.ptr) }
    }

    pub fn project(
        &mut self,
        curve_node: &AlCurveNode,
        params: &mut [f64; 3],
        create_curve: bool,
    ) -> Result<(), String> {
        let status = unsafe {
            alsurface_project(self.ptr, curve_node.ptr, params.as_mut_ptr(), create_curve)
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn project_normal(
        &mut self,
        curve_node: &AlCurveNode,
        create_curve: bool,
    ) -> Result<(), String> {
        let status = unsafe { alsurface_project_normal(self.ptr, curve_node.ptr, create_curve) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn intersect(&mut self, other: &AlSurface, create_curves: bool) -> Result<(), String> {
        let status = unsafe { alsurface_intersect(self.ptr, other.ptr, create_curves) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn trim_param(&mut self, u: &[f64], v: &[f64], create_region: bool) -> Result<(), String> {
        let status = unsafe {
            alsurface_trim_param(
                self.ptr,
                u.len() as i32,
                u.as_ptr(),
                v.as_ptr(),
                create_region,
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn trim_param_no_region(&mut self, u: &[f64], v: &[f64]) -> Result<(), String> {
        let status = unsafe {
            alsurface_trim_param_no_region(self.ptr, u.len() as i32, u.as_ptr(), v.as_ptr())
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn trim_cos(
        &mut self,
        curves: &[&AlCurveOnSurface],
        create_region: bool,
    ) -> Result<(), String> {
        let curve_ptrs: Vec<*mut AlCurveOnSurface_ptr> = curves.iter().map(|c| c.ptr).collect();
        let status = if create_region {
            unsafe {
                alsurface_trim_cos_create(
                    self.ptr,
                    curves.len() as i32,
                    curve_ptrs.as_ptr() as *const _ as *const _,
                    create_region,
                )
            }
        } else {
            unsafe {
                alsurface_trim_cos(
                    self.ptr,
                    curves.len() as i32,
                    curve_ptrs.as_ptr() as *const _ as *const _,
                )
            }
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn trim_cos_flags(
        &mut self,
        flags: &[i32],
        curves: &[&AlCurveOnSurface],
    ) -> Result<(), String> {
        let curve_ptrs: Vec<*mut AlCurveOnSurface_ptr> = curves.iter().map(|c| c.ptr).collect();
        let status = unsafe {
            alsurface_trim_cos_flags(
                self.ptr,
                curves.len() as i32,
                flags.as_ptr(),
                curve_ptrs.as_ptr() as *const _ as *const _,
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn uniform_rebuild(
        &mut self,
        nu: i32,
        nv: i32,
        in_u: bool,
        in_v: bool,
        keep_trim: bool,
    ) -> Result<AlSurfaceNode, String> {
        let mut new_surface_node: *mut super::alsurfacenode::AlSurfaceNode_ptr =
            std::ptr::null_mut();
        let status = unsafe {
            alsurface_uniform_rebuild(
                self.ptr,
                &mut new_surface_node,
                nu,
                nv,
                in_u,
                in_v,
                keep_trim,
            )
        };
        if status == statusCode::Success {
            Ok(AlSurfaceNode {
                ptr: new_surface_node,
            })
        } else {
            Err(status.to_string())
        }
    }

    pub fn periodic_to_non_periodic(&mut self, u_ends: i32, v_ends: i32) -> Result<(), String> {
        let status = unsafe { alsurface_periodic_to_non_periodic(self.ptr, u_ends, v_ends) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn periodic_to_non_periodic_all(&mut self) -> Result<(), String> {
        let status = unsafe { alsurface_periodic_to_non_periodic_all(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn untrimmed_to_trimmed(&self) -> Option<AlSurface> {
        let ptr = unsafe { alsurface_untrimmed_to_trimmed(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlSurface { ptr })
        }
    }

    pub fn trim_surface_to_region(&self) -> Option<AlTrimRegion> {
        let ptr = unsafe { alsurface_trim_surface_to_region(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlTrimRegion { ptr })
        }
    }

    pub fn first_trim_region(&self) -> Option<AlTrimRegion> {
        let ptr = unsafe { alsurface_first_trim_region(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlTrimRegion { ptr })
        }
    }

    pub fn local_boundary(&self, which: i32) -> Option<AlCurve> {
        let ptr = unsafe { alsurface_local_boundary(self.ptr, which) };
        if ptr.is_null() {
            None
        } else {
            Some(AlCurve { ptr })
        }
    }

    pub fn model_space_boundary(&self) -> Option<AlCurve> {
        let ptr = unsafe { alsurface_model_space_boundary(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlCurve { ptr })
        }
    }

    pub fn parameter_boundary(&self, which: i32) -> Option<AlCurve> {
        let ptr = unsafe { alsurface_parameter_boundary(self.ptr, which) };
        if ptr.is_null() {
            None
        } else {
            Some(AlCurve { ptr })
        }
    }

    pub fn param_space_boundary(&self) -> Option<AlCurve> {
        let ptr = unsafe { alsurface_param_space_boundary(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlCurve { ptr })
        }
    }

    pub fn first_curve_on_surface(&self) -> Option<AlCurveOnSurface> {
        let ptr = unsafe { alsurface_first_curve_on_surface(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlCurveOnSurface { ptr })
        }
    }

    pub fn add_curve_on_surface(
        &mut self,
        curve_on_surface: &AlCurveOnSurface,
    ) -> Result<(), String> {
        let status = unsafe { alsurface_add_curve_on_surface(self.ptr, curve_on_surface.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn remove_curve_on_surface(
        &mut self,
        curve_on_surface: &AlCurveOnSurface,
    ) -> Result<(), String> {
        let status = unsafe { alsurface_remove_curve_on_surface(self.ptr, curve_on_surface.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn is_display_mode_set(&self, display_mode: AlDisplayModeType) -> bool {
        unsafe { alsurface_is_display_mode_set(self.ptr, display_mode) }
    }

    pub fn set_display_mode(&mut self, display_mode: AlDisplayModeType, enable: bool) -> Result<(), String> {
        let status = unsafe { alsurface_set_display_mode(self.ptr, display_mode, enable) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn do_updates(&mut self, new_state: bool) -> Result<(), String> {
        let status = unsafe { alsurface_do_updates(self.ptr, new_state) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn insert(&mut self, param: f64, direction: AlSurfaceDirection) -> Result<(), String> {
        let status = unsafe { alsurface_insert(self.ptr, param, direction as i32) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn reverse_direction(&mut self, in_u: bool, in_v: bool) -> Result<(), String> {
        let status = unsafe { alsurface_reverse_direction(self.ptr, in_u, in_v) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }
}

unsafe extern "C" {
    fn alsurface_new() -> *mut AlSurface_ptr;
    fn alsurface_delete(surface: *mut AlSurface_ptr);

    fn alsurface_create(
        surface: *mut AlSurface_ptr,
        u_num_cvs: i32,
        v_num_cvs: i32,
        u_form: i32,
        v_form: i32,
        u_degree: i32,
        v_degree: i32,
        u_knots: *const f64,
        v_knots: *const f64,
        num_cvs: i32,
        stride: i32,
        cv_data: *const f64,
        u_mult: *const i32,
        v_mult: *const i32,
    ) -> statusCode;
    fn alsurface_create_simple(
        surface: *mut *mut AlSurface_ptr,
        u_num_cvs: i32,
        v_num_cvs: i32,
        u_form: i32,
        v_form: i32,
        u_knots: *const f64,
        v_knots: *const f64,
        num_cvs: i32,
        stride: i32,
        cv_data: *const f64,
    ) -> statusCode;

    fn alsurface_replace(
        surface: *mut AlSurface_ptr,
        u_num_cvs: i32,
        v_num_cvs: i32,
        u_form: i32,
        v_form: i32,
        u_degree: i32,
        v_degree: i32,
        u_knots: *const f64,
        v_knots: *const f64,
        num_cvs: i32,
        stride: i32,
        cv_data: *const f64,
        u_mult: *const i32,
        v_mult: *const i32,
    ) -> statusCode;
    fn alsurface_replace_simple(
        surface: *mut AlSurface_ptr,
        u_num_cvs: i32,
        v_num_cvs: i32,
        u_form: i32,
        v_form: i32,
        u_knots: *const f64,
        v_knots: *const f64,
        num_cvs: i32,
        stride: i32,
        cv_data: *const f64,
    ) -> statusCode;

    fn alsurface_create_revolved(
        surface: *mut AlSurface_ptr,
        axis_start: *const f64,
        axis_end: *const f64,
        start_angle: f64,
        end_angle: f64,
        curve: *const AlCurve_ptr,
    ) -> statusCode;
    fn alsurface_create_extrusion(
        surface: *mut AlSurface_ptr,
        direction: *const f64,
        length: f64,
        curve: *const AlCurve_ptr,
    ) -> statusCode;
    /*    OPENALIAS_C_API statusCode alsurface_create_torus(AlSurface* surface, const double center[3],
    const double axis[3], double majorRadius, double minorRadius, const double startAngle[3],
    double sweepAngle, double uMin, double uMax, double vMin, double vMax); */
    fn alsurface_create_torus(
        surface: *mut AlSurface_ptr,
        origin: *const f64,
        direction: *const f64,
        major_radius: f64,
        minor_radius: f64,
        zero: *const f64,
        rot_start: f64,
        rot_end: f64,
        arc_start: f64,
        arc_end: f64,
    ) -> statusCode;
    fn alsurface_create_sphere(
        surface: *mut AlSurface_ptr,
        center: *const f64,
        radius: f64,
    ) -> statusCode;
    fn alsurface_create_cylinder(
        surface: *mut AlSurface_ptr,
        base: *const f64,
        apex: *const f64,
        radius: f64,
    ) -> statusCode;
    fn alsurface_create_cone(
        surface: *mut AlSurface_ptr,
        base: *const f64,
        base_radius: f64,
        apex: *const f64,
        apex_radius: f64,
    ) -> statusCode;

    fn alsurface_surface_node(
        surface: *const AlSurface_ptr,
    ) -> *mut super::alsurfacenode::AlSurfaceNode_ptr;

    fn alsurface_u_form(surface: *const AlSurface_ptr) -> i32;
    fn alsurface_v_form(surface: *const AlSurface_ptr) -> i32;
    fn alsurface_u_degree(surface: *const AlSurface_ptr) -> i32;
    fn alsurface_v_degree(surface: *const AlSurface_ptr) -> i32;
    fn alsurface_u_num_spans(surface: *const AlSurface_ptr) -> i32;
    fn alsurface_v_num_spans(surface: *const AlSurface_ptr) -> i32;
    fn alsurface_u_num_cvs(surface: *const AlSurface_ptr) -> i32;
    fn alsurface_v_num_cvs(surface: *const AlSurface_ptr) -> i32;

    fn alsurface_first_cv(
        surface: *const AlSurface_ptr,
    ) -> *mut super::alsurfacecv::AlSurfaceCV_ptr;
    fn alsurface_get_cv(
        surface: *const AlSurface_ptr,
        index_u: i32,
        index_v: i32,
    ) -> *mut super::alsurfacecv::AlSurfaceCV_ptr;
    fn alsurface_first_attribute(surface: *const AlSurface_ptr) -> *mut AlAttributes_ptr;

    fn alsurface_is_construction_history_result(surface: *const AlSurface_ptr) -> bool;

    fn alsurface_set_cvs_unaffected_position(
        surface: *mut AlSurface_ptr,
        cv_data: *const f64,
    ) -> statusCode;
    fn alsurface_set_u_knot_vector(surface: *mut AlSurface_ptr, knots: *const f64) -> statusCode;
    fn alsurface_set_v_knot_vector(surface: *mut AlSurface_ptr, knots: *const f64) -> statusCode;
    fn alsurface_set_cvs_unaffected_position_incl_multiples(
        surface: *mut AlSurface_ptr,
        cv_data: *const f64,
    ) -> statusCode;
    fn alsurface_set_real_u_knot_vector(
        surface: *mut AlSurface_ptr,
        knots: *const f64,
    ) -> statusCode;
    fn alsurface_set_real_v_knot_vector(
        surface: *mut AlSurface_ptr,
        knots: *const f64,
    ) -> statusCode;
    fn alsurface_unpile_end_knots(
        surface: *mut AlSurface_ptr,
        flags: i32,
        values: *const f64,
    ) -> statusCode;

    fn alsurface_is_point_active(surface: *const AlSurface_ptr, u: f64, v: f64) -> bool;

    fn alsurface_cvs_world_position(
        surface: *const AlSurface_ptr,
        cv_data: *mut f64,
        u_mult: *mut i32,
        v_mult: *mut i32,
    ) -> statusCode;
    fn alsurface_cvs_affected_position(
        surface: *const AlSurface_ptr,
        tm: *const AlTM,
        cv_data: *mut f64,
        u_mult: *mut i32,
        v_mult: *mut i32,
    ) -> statusCode;
    fn alsurface_cvs_unaffected_position(
        surface: *const AlSurface_ptr,
        cv_data: *mut f64,
        u_mult: *mut i32,
        v_mult: *mut i32,
    ) -> statusCode;

    fn alsurface_u_num_knots(surface: *const AlSurface_ptr) -> i32;
    fn alsurface_v_num_knots(surface: *const AlSurface_ptr) -> i32;
    fn alsurface_u_knot_vector(surface: *const AlSurface_ptr, knots: *mut f64) -> statusCode;
    fn alsurface_v_knot_vector(surface: *const AlSurface_ptr, knots: *mut f64) -> statusCode;

    fn alsurface_u_num_cvs_incl_multiples(surface: *const AlSurface_ptr) -> i32;
    fn alsurface_v_num_cvs_incl_multiples(surface: *const AlSurface_ptr) -> i32;
    fn alsurface_cvs_world_position_incl_multiples(
        surface: *const AlSurface_ptr,
        cv_data: *mut f64,
    ) -> statusCode;
    fn alsurface_cvs_affected_position_incl_multiples(
        surface: *const AlSurface_ptr,
        tm: *const AlTM,
        cv_data: *mut f64,
    ) -> statusCode;
    fn alsurface_cvs_unaffected_position_incl_multiples(
        surface: *const AlSurface_ptr,
        cv_data: *mut f64,
    ) -> statusCode;

    fn alsurface_real_u_num_knots(surface: *const AlSurface_ptr) -> i32;
    fn alsurface_real_v_num_knots(surface: *const AlSurface_ptr) -> i32;
    fn alsurface_real_u_knot_vector(surface: *const AlSurface_ptr, knots: *mut f64) -> statusCode;
    fn alsurface_real_v_knot_vector(surface: *const AlSurface_ptr, knots: *mut f64) -> statusCode;

    fn alsurface_area(
        surface: *const AlSurface_ptr,
        out_area: *mut f64,
        world_coords: bool,
        tolerance: f64,
    ) -> statusCode;
    fn alsurface_circumference(
        surface: *const AlSurface_ptr,
        out_circum: *mut f64,
        world_coords: bool,
        tolerance: f64,
    ) -> statusCode;

    fn alsurface_eval(
        surface: *const AlSurface_ptr,
        u: f64,
        v: f64,
        world_coords: bool,
        p: *mut f64,
        pu: *mut f64,
        pv: *mut f64,
        n: *mut f64,
        compute_derivs: bool,
        normalize_normal: bool,
    ) -> statusCode;

    fn alsurface_first_shader(surface: *const AlSurface_ptr) -> *mut AlShader_ptr;
    fn alsurface_next_shader(
        surface: *const AlSurface_ptr,
        prev: *const AlShader_ptr,
    ) -> *mut AlShader_ptr;
    fn alsurface_get_switch_shader(surface: *const AlSurface_ptr) -> *mut AlSwitchShader_ptr;
    fn alsurface_get_layered_shader(surface: *const AlSurface_ptr) -> *mut AlLayeredShader_ptr;
    fn alsurface_next_shader_d(
        surface: *const AlSurface_ptr,
        shader: *mut AlShader_ptr,
    ) -> statusCode;

    fn alsurface_assign_shader(
        surface: *mut AlSurface_ptr,
        shader: *const AlShader_ptr,
    ) -> statusCode;
    fn alsurface_assign_switch_shader(
        surface: *mut AlSurface_ptr,
        shader: *const AlSwitchShader_ptr,
    ) -> statusCode;
    fn alsurface_layer_shader(
        surface: *mut AlSurface_ptr,
        shader: *const AlShader_ptr,
    ) -> statusCode;

    fn alsurface_render_info(
        surface: *const AlSurface_ptr,
        render_info: *mut AlRenderInfo,
    ) -> statusCode;
    fn alsurface_set_render_info(
        surface: *mut AlSurface_ptr,
        render_info: *const AlRenderInfo,
    ) -> statusCode;

    fn alsurface_trimmed(surface: *const AlSurface_ptr) -> bool;
    fn alsurface_is_target_surface(surface: *const AlSurface_ptr) -> bool;

    fn alsurface_project(
        surface: *mut AlSurface_ptr,
        curve_node: *const AlCurveNode_ptr,
        params: *mut f64,
        create_curve: bool,
    ) -> statusCode;
    fn alsurface_project_normal(
        surface: *mut AlSurface_ptr,
        curve_node: *const AlCurveNode_ptr,
        create_curve: bool,
    ) -> statusCode;
    fn alsurface_intersect(
        surface: *mut AlSurface_ptr,
        other: *const AlSurface_ptr,
        create_curves: bool,
    ) -> statusCode;

    fn alsurface_trim_param(
        surface: *mut AlSurface_ptr,
        num_points: i32,
        u: *const f64,
        v: *const f64,
        create_region: bool,
    ) -> statusCode;
    fn alsurface_trim_param_no_region(
        surface: *mut AlSurface_ptr,
        num_points: i32,
        u: *const f64,
        v: *const f64,
    ) -> statusCode;
    fn alsurface_trim_cos(
        surface: *mut AlSurface_ptr,
        num_curves: i32,
        curves: *const *const AlCurveOnSurface_ptr,
    ) -> statusCode;
    fn alsurface_trim_cos_create(
        surface: *mut AlSurface_ptr,
        num_curves: i32,
        curves: *const *const AlCurveOnSurface_ptr,
        create_region: bool,
    ) -> statusCode;
    fn alsurface_trim_cos_flags(
        surface: *mut AlSurface_ptr,
        num_curves: i32,
        flags: *const i32,
        curves: *const *const AlCurveOnSurface_ptr,
    ) -> statusCode;

    fn alsurface_uniform_rebuild(
        surface: *mut AlSurface_ptr,
        new_surface_node: *mut *mut super::alsurfacenode::AlSurfaceNode_ptr,
        nu: i32,
        nv: i32,
        in_u: bool,
        in_v: bool,
        keep_trim: bool,
    ) -> statusCode;

    fn alsurface_periodic_to_non_periodic(
        surface: *mut AlSurface_ptr,
        u_ends: i32,
        v_ends: i32,
    ) -> statusCode;
    fn alsurface_periodic_to_non_periodic_all(surface: *mut AlSurface_ptr) -> statusCode;

    fn alsurface_untrimmed_to_trimmed(surface: *const AlSurface_ptr) -> *mut AlSurface_ptr;
    fn alsurface_trim_surface_to_region(
        surface: *const AlSurface_ptr,
    ) -> *mut super::altrimregion::AlTrimRegion_ptr;
    fn alsurface_first_trim_region(
        surface: *const AlSurface_ptr,
    ) -> *mut super::altrimregion::AlTrimRegion_ptr;
    fn alsurface_local_boundary(surface: *const AlSurface_ptr, which: i32) -> *mut AlCurve_ptr;
    fn alsurface_model_space_boundary(surface: *const AlSurface_ptr) -> *mut AlCurve_ptr;
    fn alsurface_parameter_boundary(surface: *const AlSurface_ptr, which: i32) -> *mut AlCurve_ptr;
    fn alsurface_param_space_boundary(surface: *const AlSurface_ptr) -> *mut AlCurve_ptr;
    fn alsurface_first_curve_on_surface(surface: *const AlSurface_ptr)
    -> *mut AlCurveOnSurface_ptr;
    fn alsurface_add_curve_on_surface(
        surface: *mut AlSurface_ptr,
        curve_on_surface: *const AlCurveOnSurface_ptr,
    ) -> statusCode;
    fn alsurface_remove_curve_on_surface(
        surface: *mut AlSurface_ptr,
        curve_on_surface: *const AlCurveOnSurface_ptr,
    ) -> statusCode;

    fn alsurface_is_display_mode_set(surface: *const AlSurface_ptr, display_mode: AlDisplayModeType) -> bool;
    fn alsurface_set_display_mode(
        surface: *mut AlSurface_ptr,
        display_mode: AlDisplayModeType,
        enable: bool,
    ) -> statusCode;

    fn alsurface_do_updates(surface: *mut AlSurface_ptr, new_state: bool) -> statusCode;
    fn alsurface_insert(surface: *mut AlSurface_ptr, param: f64, direction: i32) -> statusCode;
    fn alsurface_reverse_direction(
        surface: *mut AlSurface_ptr,
        in_u: bool,
        in_v: bool,
    ) -> statusCode;
}
