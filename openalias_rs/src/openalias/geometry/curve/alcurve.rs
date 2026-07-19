#![allow(non_camel_case_types)]
use crate::*;
use base_geometry_lib::{
    NurbsCurve_Base_Trait, NurbsCurve_Trait, Point_Trait, RU_3dPoint, curve_form_type,
};
#[repr(C)]
pub struct AlCurve_ptr {
    _private: [u8; 0],
}

#[derive(Debug)]
pub struct AlCurve {
    pub ptr: *mut AlCurve_ptr,
}
unsafe impl Send for AlCurve {}
unsafe impl Sync for AlCurve {}

impl NurbsCurve_Base_Trait for AlCurve {
    fn Knots(&self) -> Vec<f64> {
        self.real_knot_vector().unwrap()
    }
    fn Degree_NurbsCurveTraitbase(&self) -> usize {
        self.degree() as usize
    }
    fn CVs(&self) -> Vec<RU_3dPoint> {
        self.cvs_world_position()
            .unwrap()
            .0
            .iter()
            .map(|p| p.ToRU_3dPoint())
            .collect()
    }

    fn Weights(&self) -> Vec<f64> {
        self.cvs_world_position()
            .unwrap()
            .0
            .iter()
            .map(|p| p[3])
            .collect()
    }
    fn Form(&self) -> curve_form_type {
        self.form()
    }
}
impl NurbsCurve_Trait for AlCurve {
    fn SetCVs(&mut self, cvs: &Vec<RU_3dPoint>) -> Result<(), String> {
        if self.number_of_cvs() != cvs.len() {
            return Err("CVs length mismatch".to_string());
        }
        let world_cvs = self
            .cvs_world_position()
            .unwrap()
            .0
            .iter()
            .map(|p| p.ToRU_3dPoint())
            .collect::<Vec<_>>();
        let Unaffected_cvs = self
            .cvs_unaffected_position()
            .unwrap()
            .0
            .iter()
            .map(|p| p.ToRU_3dPoint())
            .collect::<Vec<_>>();
        let cur_unaffected_cvs = cvs
            .iter()
            .enumerate()
            .map(|(index, &cv)| {
                let world_cv = world_cvs[index];
                let vector = cv - world_cv;
                let unaffected_cv = Unaffected_cvs[index];
                let rel = unaffected_cv + vector;
                [rel.x, rel.y, rel.z, 1.0]
            })
            .collect::<Vec<_>>();
        self.set_cvs_unaffected_position(&cur_unaffected_cvs)
    }
}
pub trait NurbsCurve_Base_Trait_ToAlCurve {
    fn to_alcurve(&self) -> Result<AlCurve, String>;
}
impl<T: NurbsCurve_Base_Trait> NurbsCurve_Base_Trait_ToAlCurve for T {
    fn to_alcurve(&self) -> Result<AlCurve, String> {
        let aliasKnots = self.AliasKnots();
        let aliasCVs = self
            .AliasCVs()
            .into_iter()
            .map(|p| [p.x, p.y, p.z, p.w])
            .collect::<Vec<_>>();
        let degree = self.Degree_NurbsCurveTraitbase();
        let form = self.Form();
        let mut alcurve = AlCurve::new();
        let mult = vec![1; aliasCVs.len()];
        alcurve.create(degree as i32, form, &aliasKnots, &aliasCVs, &mult)?;
        Ok(alcurve)
    }
}

impl AlCurve {
    pub fn new() -> AlCurve {
        let ptr = unsafe { alcurve_new() };
        AlCurve { ptr }
    }

    pub fn create(
        &mut self,
        degree: i32,
        form: curve_form_type,
        knots: &[f64],
        cv_data: &[[f64; 4]],
        multiplicities: &[i32],
    ) -> Result<(), String> {
        let status = unsafe {
            alcurve_create(
                self.ptr,
                degree,
                form as i32,
                knots.len() as i32,
                knots.as_ptr(),
                cv_data.len() as i32,
                cv_data.as_ptr(),
                multiplicities.as_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_simple(
        &mut self,
        num_cvs: i32,
        form: curve_form_type,
        knots: &[f64],
        cv_data: &[[f64; 4]],
    ) -> Result<(), String> {
        let status = unsafe {
            alcurve_create_simple(
                self.ptr,
                num_cvs,
                form as i32,
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

    pub fn replace(
        &self,
        num_cvs: i32,
        form: i32,
        degree: i32,
        knots: &[f64],
        cv_data: &[[f64; 4]],
        multiplicities: Option<&[i32]>,
    ) -> Result<(), String> {
        let multiplicities_ptr = multiplicities.map_or(std::ptr::null(), |m| m.as_ptr());
        let status = unsafe {
            alcurve_replace(
                self.ptr,
                num_cvs,
                form,
                degree,
                knots.as_ptr(),
                cv_data.len() as i32,
                cv_data.as_ptr(),
                multiplicities_ptr,
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn replace_simple(
        &self,
        num_cvs: i32,
        form: i32,
        knots: &[f64],
        cv_data: &[[f64; 4]],
    ) -> Result<(), String> {
        let status = unsafe {
            alcurve_replace_simple(
                self.ptr,
                num_cvs,
                form,
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

    pub fn create_line(&mut self, start: RU_3dPoint, end: RU_3dPoint) -> Result<(), String> {
        let status = unsafe { alcurve_create_line(self.ptr, &start.x, &end.x) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }
    pub fn create_line_deg1(start: RU_3dPoint, end: RU_3dPoint) -> Result<AlCurve, String> {
        let mut new_curve = AlCurve::new();
        new_curve.create(
            1,
            curve_form_type::kOpen,
            &[0.0, 1.0],
            &[[start.x, start.y, start.z, 1.0], [end.x, end.y, end.z, 1.0]],
            &[1, 1],
        )?;
        Ok(new_curve)
    }

    pub fn create_arc_3pt(
        &self,
        start: &[f64; 3],
        center: &[f64; 3],
        end: &[f64; 3],
        create_curve: bool,
    ) -> Result<(), String> {
        let status = unsafe {
            alcurve_create_arc_3pt(
                self.ptr,
                start.as_ptr(),
                center.as_ptr(),
                end.as_ptr(),
                create_curve,
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_arc_params(
        &self,
        start: &mut [f64; 3],
        center: &mut [f64; 3],
        end: &mut [f64; 3],
        major_radius: f64,
        minor_radius: f64,
        angle: f64,
        num_spans: i32,
        create_curve: bool,
    ) -> Result<(), String> {
        let status = unsafe {
            alcurve_create_arc_params(
                self.ptr,
                start.as_mut_ptr(),
                center.as_mut_ptr(),
                end.as_mut_ptr(),
                major_radius,
                minor_radius,
                angle,
                num_spans,
                create_curve,
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_conic(
        &self,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
        start: &[f64; 3],
        end: &[f64; 3],
    ) -> Result<(), String> {
        let status = unsafe {
            alcurve_create_conic(self.ptr, a, b, c, d, e, f, start.as_ptr(), end.as_ptr())
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_parabola(
        &self,
        focus: &[f64; 3],
        apex: &[f64; 3],
        start: &[f64; 3],
        end: &[f64; 3],
    ) -> Result<(), String> {
        let status = unsafe {
            alcurve_create_parabola(
                self.ptr,
                focus.as_ptr(),
                apex.as_ptr(),
                start.as_ptr(),
                end.as_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_ellipse(
        &self,
        center: &[f64; 3],
        major_axis: &[f64; 3],
        minor_axis: &[f64; 3],
        start: &[f64; 3],
        end: &[f64; 3],
    ) -> Result<(), String> {
        let status = unsafe {
            alcurve_create_ellipse(
                self.ptr,
                center.as_ptr(),
                major_axis.as_ptr(),
                minor_axis.as_ptr(),
                start.as_ptr(),
                end.as_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_hyperbola(
        &self,
        center: &[f64; 3],
        major_axis: &[f64; 3],
        minor_axis: &[f64; 3],
        start: &[f64; 3],
        end: &[f64; 3],
    ) -> Result<(), String> {
        let status = unsafe {
            alcurve_create_hyperbola(
                self.ptr,
                center.as_ptr(),
                major_axis.as_ptr(),
                minor_axis.as_ptr(),
                start.as_ptr(),
                end.as_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_point(&self, point: &[f64; 3]) -> Result<(), String> {
        let status = unsafe { alcurve_create_point(self.ptr, point.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn curve_node(&self) -> Option<AlCurveNode> {
        let ptr = unsafe { alcurve_curve_node(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlCurveNode { ptr })
        }
    }

    pub fn form(&self) -> curve_form_type {
        let s = unsafe { alcurve_form(self.ptr) };
        unsafe { std::mem::transmute(s) }
    }

    pub fn degree(&self) -> i32 {
        unsafe { alcurve_degree(self.ptr) }
    }

    pub fn number_of_spans(&self) -> usize {
        let s = unsafe { alcurve_number_of_spans(self.ptr) };
        s as usize
    }

    pub fn number_of_cvs(&self) -> usize {
        let s = unsafe { alcurve_number_of_cvs(self.ptr) };
        s as usize
    }

    pub fn first_cv(&self) -> Option<AlCurveCV> {
        let ptr = unsafe { alcurve_first_cv(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlCurveCV { ptr })
        }
    }

    pub fn get_cv(&self, index: i32) -> Option<AlCurveCV> {
        let ptr = unsafe { alcurve_get_cv(self.ptr, index) };
        if ptr.is_null() {
            None
        } else {
            Some(AlCurveCV { ptr })
        }
    }

    pub fn first_attribute(&self) -> Option<AlAttributes> {
        let ptr = unsafe { alcurve_first_attribute(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlAttributes { ptr })
        }
    }

    pub fn cvs_world_position(&self) -> Result<(Vec<[f64; 4]>, Vec<i32>), String> {
        let cv_num = self.number_of_cvs();
        let mut cv_data = Vec::new();
        cv_data.resize(cv_num, [0.0; 4]);
        let mut multiplicities = Vec::new();
        multiplicities.resize(cv_num, 0);
        let status = unsafe {
            alcurve_cvs_world_position(self.ptr, cv_data.as_mut_ptr(), multiplicities.as_mut_ptr())
        };
        if status == statusCode::Success {
            Ok((cv_data, multiplicities))
        } else {
            Err(status.to_string())
        }
    }

    pub fn cvs_affected_position(
        &self,
        tm: &mut AlTM,
    ) -> Result<(Vec<[f64; 4]>, Vec<i32>), String> {
        let cv_num = self.number_of_cvs();
        let mut cv_data = Vec::with_capacity(cv_num as usize);
        let mut multiplicities = Vec::with_capacity(cv_num as usize);
        let status = unsafe {
            alcurve_cvs_affected_position(
                self.ptr,
                tm,
                cv_data.as_mut_ptr(),
                multiplicities.as_mut_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok((cv_data, multiplicities))
        } else {
            Err(status.to_string())
        }
    }

    pub fn cvs_unaffected_position(&self) -> Result<(Vec<[f64; 4]>, Vec<i32>), String> {
        let cv_num = self.number_of_cvs();
        let mut cv_data = Vec::with_capacity(cv_num as usize);
        let mut multiplicities = Vec::with_capacity(cv_num as usize);
        let status = unsafe {
            alcurve_cvs_unaffected_position(
                self.ptr,
                cv_data.as_mut_ptr(),
                multiplicities.as_mut_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok((cv_data, multiplicities))
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_cvs_unaffected_position(&self, cv_data: &[[f64; 4]]) -> Result<(), String> {
        let status = unsafe { alcurve_set_cvs_unaffected_position(self.ptr, cv_data.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn length(
        &self,
        world_coordinates: Option<bool>,
        tolerance: Option<f64>,
    ) -> Result<f64, String> {
        let world_coordinates = world_coordinates.unwrap_or(true);
        let tolerance = tolerance.unwrap_or(0.001);
        let mut length: f64 = 0.0;
        let status = unsafe { alcurve_length(self.ptr, &mut length, world_coordinates, tolerance) };
        if status == statusCode::Success {
            Ok(length)
        } else {
            Err(status.to_string())
        }
    }

    pub fn eval(
        &self,
        param: f64,
        world_coordinates: bool,
        p: &mut [f64; 3],
        dp: &mut [f64; 3],
    ) -> Result<(), String> {
        let status = unsafe {
            alcurve_eval(
                self.ptr,
                param,
                world_coordinates,
                p.as_mut_ptr(),
                dp.as_mut_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn eval_dist(
        &self,
        dist: f64,
        p: &mut [f64; 3],
        out_t: &mut f64,
        world_coordinates: bool,
        tolerance: f64,
    ) -> Result<(), String> {
        let status = unsafe {
            alcurve_eval_dist(
                self.ptr,
                dist,
                p.as_mut_ptr(),
                out_t,
                world_coordinates,
                tolerance,
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn number_of_knots(&self) -> i32 {
        unsafe { alcurve_number_of_knots(self.ptr) }
    }

    pub fn knot_vector(&self) -> Result<Vec<f64>, String> {
        let knot_num = self.number_of_knots() as usize;
        let mut knots = Vec::with_capacity(knot_num);
        knots.resize(knot_num, 0.0);
        let status = unsafe { alcurve_knot_vector(self.ptr, knots.as_mut_ptr()) };
        if status == statusCode::Success {
            Ok(knots)
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_knot_vector(&self, knots: &[f64]) -> Result<(), String> {
        let status = unsafe { alcurve_set_knot_vector(self.ptr, knots.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn number_of_cvs_incl_multiples(&self) -> i32 {
        unsafe { alcurve_number_of_cvs_incl_multiples(self.ptr) }
    }

    pub fn cvs_world_position_incl_multiples(
        &self,
        cv_data: &mut [[f64; 4]],
    ) -> Result<(), String> {
        let status =
            unsafe { alcurve_cvs_world_position_incl_multiples(self.ptr, cv_data.as_mut_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn cvs_affected_position_incl_multiples(
        &self,
        tm: &mut AlTM,
        cv_data: &mut [[f64; 4]],
    ) -> Result<(), String> {
        let status = unsafe {
            alcurve_cvs_affected_position_incl_multiples(self.ptr, tm, cv_data.as_mut_ptr())
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn cvs_unaffected_position_incl_multiples(
        &self,
        cv_data: &mut [[f64; 4]],
    ) -> Result<(), String> {
        let status = unsafe {
            alcurve_cvs_unaffected_position_incl_multiples(self.ptr, cv_data.as_mut_ptr())
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_cvs_unaffected_position_incl_multiples(
        &self,
        cv_data: &[[f64; 4]],
    ) -> Result<(), String> {
        let status = unsafe {
            alcurve_set_cvs_unaffected_position_incl_multiples(self.ptr, cv_data.as_ptr())
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn real_number_of_knots(&self) -> i32 {
        unsafe { alcurve_real_number_of_knots(self.ptr) }
    }

    pub fn real_knot_vector(&self) -> Result<Vec<f64>, String> {
        let knot_num = self.real_number_of_knots() as usize;
        let mut knots = Vec::new();
        knots.resize(knot_num, 0.0);
        let status = unsafe { alcurve_real_knot_vector(self.ptr, knots.as_mut_ptr()) };
        if status == statusCode::Success {
            Ok(knots)
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_real_knot_vector(&self, knots: &[f64]) -> Result<(), String> {
        let status = unsafe { alcurve_set_real_knot_vector(self.ptr, knots.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn is_display_mode_set(&self, display_mode: AlDisplayModeType) -> bool {
        unsafe { alcurve_is_display_mode_set(self.ptr, display_mode) }
    }

    pub fn set_display_mode(&self, display_mode: AlDisplayModeType, enable: bool) -> Result<(), String> {
        let status = unsafe { alcurve_set_display_mode(self.ptr, display_mode , enable) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn do_updates(&self, new_state: bool) -> Result<(), String> {
        let status = unsafe { alcurve_do_updates(self.ptr, new_state) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn periodic_to_non_periodic(&self) -> Result<(), String> {
        let status = unsafe { alcurve_periodic_to_non_periodic(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn normal(&self, normal: &mut [f64; 3]) -> Result<(), String> {
        let status = unsafe { alcurve_normal(self.ptr, normal.as_mut_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn insert(&self, param: f64) -> Result<(), String> {
        let status = unsafe { alcurve_insert(self.ptr, param) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn append(&self, pos: &mut [f64; 4], point_type: i32) -> Result<(), String> {
        let status = unsafe { alcurve_append(self.ptr, pos.as_mut_ptr(), point_type) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn reverse_curve(&self) -> Result<(), String> {
        let status = unsafe { alcurve_reverse_curve(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn extend_curve(&self, param: f64, pos: &mut [f64; 4]) -> Result<(), String> {
        let status = unsafe { alcurve_extend_curve(self.ptr, param, pos.as_mut_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn increment_degree(&self) -> Result<(), String> {
        let status = unsafe { alcurve_increment_degree(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn adjust_end_span(&self, param: f64) -> Result<(), String> {
        let status = unsafe { alcurve_adjust_end_span(self.ptr, param) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn trim_curve(&self, start_param: f64, end_param: f64) -> Result<(), String> {
        let status = unsafe { alcurve_trim_curve(self.ptr, start_param, end_param) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }
}
impl AlObjectMethods for AlCurve {
    fn as_object_ptr(&self) -> *mut crate::AlObject_ptr {
        self.ptr as *mut crate::AlObject_ptr
    }
    fn name_ex(&self) -> String {
        let name = self.name();
        if !name.is_empty() {
            return name;
        }
        let node = self.curve_node().unwrap();
        format!("{}_{:?}", node.name(), self.type_())
    }
}

impl Drop for AlCurve {
    fn drop(&mut self) {
        unsafe {
            alcurve_delete(self.ptr);
            self.ptr = std::ptr::null_mut();
        }
    }
}

unsafe extern "C" {
    fn alcurve_new() -> *mut AlCurve_ptr;
    fn alcurve_delete(curve: *mut AlCurve_ptr);
    /*
       OPENALIAS_C_API statusCode alcurve_create(AlCurve* alcurve, int deg, int form, int numKnots,
       const double knotVector[], int numControlPts, const double controlPoint[][4], const int multiplicity[]);
    */
    fn alcurve_create(
        curve: *mut AlCurve_ptr,
        degree: i32,
        form: i32,
        numKnots: i32,
        knots: *const f64,
        numControlPts: i32,
        cv_data: *const [f64; 4],
        multiplicities: *const i32,
    ) -> statusCode;
    fn alcurve_create_simple(
        curve: *mut AlCurve_ptr,
        num_cvs: i32,
        form: i32,
        knots: *const f64,
        num_cvs_data: i32,
        cv_data: *const [f64; 4],
    ) -> statusCode;

    fn alcurve_replace(
        curve: *mut AlCurve_ptr,
        num_cvs: i32,
        form: i32,
        degree: i32,
        knots: *const f64,
        num_cvs_data: i32,
        cv_data: *const [f64; 4],
        multiplicities: *const i32,
    ) -> statusCode;
    fn alcurve_replace_simple(
        curve: *mut AlCurve_ptr,
        num_cvs: i32,
        form: i32,
        knots: *const f64,
        num_cvs_data: i32,
        cv_data: *const [f64; 4],
    ) -> statusCode;

    fn alcurve_create_line(
        curve: *mut AlCurve_ptr,
        start: *const f64,
        end: *const f64,
    ) -> statusCode;
    fn alcurve_create_arc_3pt(
        curve: *mut AlCurve_ptr,
        start: *const f64,
        center: *const f64,
        end: *const f64,
        create_curve: bool,
    ) -> statusCode;
    fn alcurve_create_arc_params(
        curve: *mut AlCurve_ptr,
        start: *mut f64,
        center: *mut f64,
        end: *mut f64,
        major_radius: f64,
        minor_radius: f64,
        angle: f64,
        num_spans: i32,
        create_curve: bool,
    ) -> statusCode;
    fn alcurve_create_conic(
        curve: *mut AlCurve_ptr,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
        start: *const f64,
        end: *const f64,
    ) -> statusCode;
    fn alcurve_create_parabola(
        curve: *mut AlCurve_ptr,
        focus: *const f64,
        apex: *const f64,
        start: *const f64,
        end: *const f64,
    ) -> statusCode;
    fn alcurve_create_ellipse(
        curve: *mut AlCurve_ptr,
        center: *const f64,
        major_axis: *const f64,
        minor_axis: *const f64,
        start: *const f64,
        end: *const f64,
    ) -> statusCode;
    fn alcurve_create_hyperbola(
        curve: *mut AlCurve_ptr,
        center: *const f64,
        major_axis: *const f64,
        minor_axis: *const f64,
        start: *const f64,
        end: *const f64,
    ) -> statusCode;
    fn alcurve_create_point(curve: *mut AlCurve_ptr, point: *const f64) -> statusCode;

    fn alcurve_curve_node(curve: *mut AlCurve_ptr) -> *mut AlCurveNode_ptr;

    fn alcurve_form(curve: *mut AlCurve_ptr) -> i32;
    fn alcurve_degree(curve: *mut AlCurve_ptr) -> i32;
    fn alcurve_number_of_spans(curve: *mut AlCurve_ptr) -> i32;
    fn alcurve_number_of_cvs(curve: *mut AlCurve_ptr) -> i32;

    fn alcurve_first_cv(curve: *mut AlCurve_ptr) -> *mut AlCurveCV_ptr;
    fn alcurve_get_cv(curve: *mut AlCurve_ptr, index: i32) -> *mut AlCurveCV_ptr;
    fn alcurve_first_attribute(curve: *mut AlCurve_ptr) -> *mut AlAttributes_ptr;

    fn alcurve_cvs_world_position(
        curve: *mut AlCurve_ptr,
        cv_data: *mut [f64; 4],
        multiplicities: *mut i32,
    ) -> statusCode;
    fn alcurve_cvs_affected_position(
        curve: *mut AlCurve_ptr,
        tm: *mut AlTM,
        cv_data: *mut [f64; 4],
        multiplicities: *mut i32,
    ) -> statusCode;
    fn alcurve_cvs_unaffected_position(
        curve: *mut AlCurve_ptr,
        cv_data: *mut [f64; 4],
        multiplicities: *mut i32,
    ) -> statusCode;
    fn alcurve_set_cvs_unaffected_position(
        curve: *mut AlCurve_ptr,
        cv_data: *const [f64; 4],
    ) -> statusCode;

    fn alcurve_length(
        curve: *mut AlCurve_ptr,
        out_length: *mut f64,
        world_coordinates: bool,
        tolerance: f64,
    ) -> statusCode;
    fn alcurve_eval(
        curve: *mut AlCurve_ptr,
        param: f64,
        world_coordinates: bool,
        p: *mut f64,
        dp: *mut f64,
    ) -> statusCode;
    fn alcurve_eval_dist(
        curve: *mut AlCurve_ptr,
        dist: f64,
        p: *mut f64,
        out_t: *mut f64,
        world_coordinates: bool,
        tolerance: f64,
    ) -> statusCode;

    fn alcurve_number_of_knots(curve: *mut AlCurve_ptr) -> i32;
    fn alcurve_knot_vector(curve: *mut AlCurve_ptr, knots: *mut f64) -> statusCode;
    fn alcurve_set_knot_vector(curve: *mut AlCurve_ptr, knots: *const f64) -> statusCode;

    fn alcurve_number_of_cvs_incl_multiples(curve: *mut AlCurve_ptr) -> i32;
    fn alcurve_cvs_world_position_incl_multiples(
        curve: *mut AlCurve_ptr,
        cv_data: *mut [f64; 4],
    ) -> statusCode;
    fn alcurve_cvs_affected_position_incl_multiples(
        curve: *mut AlCurve_ptr,
        tm: *mut AlTM,
        cv_data: *mut [f64; 4],
    ) -> statusCode;
    fn alcurve_cvs_unaffected_position_incl_multiples(
        curve: *mut AlCurve_ptr,
        cv_data: *mut [f64; 4],
    ) -> statusCode;
    fn alcurve_set_cvs_unaffected_position_incl_multiples(
        curve: *mut AlCurve_ptr,
        cv_data: *const [f64; 4],
    ) -> statusCode;

    fn alcurve_real_number_of_knots(curve: *mut AlCurve_ptr) -> i32;
    fn alcurve_real_knot_vector(curve: *mut AlCurve_ptr, knots: *mut f64) -> statusCode;
    fn alcurve_set_real_knot_vector(curve: *mut AlCurve_ptr, knots: *const f64) -> statusCode;

    fn alcurve_is_display_mode_set(curve: *mut AlCurve_ptr, display_mode: AlDisplayModeType) -> bool;
    fn alcurve_set_display_mode(
        curve: *mut AlCurve_ptr,
        display_mode: AlDisplayModeType,
        enable: bool,
    ) -> statusCode;

    fn alcurve_do_updates(curve: *mut AlCurve_ptr, new_state: bool) -> statusCode;

    fn alcurve_periodic_to_non_periodic(curve: *mut AlCurve_ptr) -> statusCode;
    fn alcurve_normal(curve: *mut AlCurve_ptr, normal: *mut f64) -> statusCode;

    fn alcurve_insert(curve: *mut AlCurve_ptr, param: f64) -> statusCode;
    fn alcurve_append(curve: *mut AlCurve_ptr, pos: *mut f64, point_type: i32) -> statusCode;
    fn alcurve_reverse_curve(curve: *mut AlCurve_ptr) -> statusCode;
    fn alcurve_extend_curve(curve: *mut AlCurve_ptr, param: f64, pos: *mut f64) -> statusCode;
    fn alcurve_increment_degree(curve: *mut AlCurve_ptr) -> statusCode;
    fn alcurve_adjust_end_span(curve: *mut AlCurve_ptr, param: f64) -> statusCode;
    fn alcurve_trim_curve(curve: *mut AlCurve_ptr, start_param: f64, end_param: f64) -> statusCode;
}
