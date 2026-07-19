use crate::*;

#[repr(C)]
pub struct AlSurfaceCurve_ptr {
    _private: [u8; 0],
}

#[derive(Debug)]
pub struct AlSurfaceCurve {
    pub ptr: *mut AlSurfaceCurve_ptr,
}

impl AlSurfaceCurve {
    pub fn create(knot_value: f64, curve_type: AlSurfaceCurveType) -> AlSurfaceCurve {
        let ptr = unsafe { alsurfacecurve_create(knot_value, curve_type) };
        AlSurfaceCurve { ptr }
    }

    pub fn create_object(&self) -> Result<(), String> {
        let status = unsafe { alsurfacecurve_create_object(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn get_surface_node(&self) -> Option<AlSurfaceNode> {
        let ptr = unsafe { alsurfacecurve_get_surface_node(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlSurfaceNode { ptr })
        }
    }

    pub fn get_surface(&self) -> Option<AlSurface> {
        let ptr = unsafe { alsurfacecurve_get_surface(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlSurface { ptr })
        }
    }

    pub fn get_knot_value(&self) -> f64 {
        unsafe { alsurfacecurve_get_knot_value(self.ptr) }
    }

    pub fn get_curve_direction(&self) -> AlSurfaceCurveType {
        unsafe { alsurfacecurve_get_curve_direction(self.ptr) }
    }
}

impl AlObjectMethods for AlSurfaceCurve {
    fn as_object_ptr(&self) -> *mut AlObject_ptr {
        self.ptr as *mut AlObject_ptr
    }
    fn name_ex(&self) -> String {
        let node = self.get_surface_node().unwrap();
        format!("{}_{:?}", node.name(), self.type_())
    }
}

impl Drop for AlSurfaceCurve {
    fn drop(&mut self) {
        unsafe {
            alsurfacecurve_destroy(self.ptr);
            self.ptr = std::ptr::null_mut();
        }
    }
}

unsafe extern "C" {
    fn alsurfacecurve_create(
        knot_value: f64,
        curve_type: AlSurfaceCurveType,
    ) -> *mut AlSurfaceCurve_ptr;
    fn alsurfacecurve_destroy(surface_curve: *mut AlSurfaceCurve_ptr);
    fn alsurfacecurve_create_object(surface_curve: *mut AlSurfaceCurve_ptr) -> statusCode;
    fn alsurfacecurve_get_surface_node(
        surface_curve: *mut AlSurfaceCurve_ptr,
    ) -> *mut AlSurfaceNode_ptr;
    fn alsurfacecurve_get_surface(surface_curve: *mut AlSurfaceCurve_ptr) -> *mut AlSurface_ptr;
    fn alsurfacecurve_get_knot_value(surface_curve: *mut AlSurfaceCurve_ptr) -> f64;
    fn alsurfacecurve_get_curve_direction(
        surface_curve: *mut AlSurfaceCurve_ptr,
    ) -> AlSurfaceCurveType;
}
