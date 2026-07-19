use crate::*;

#[repr(C)]
pub struct AlLayer_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlLayer {
    pub ptr: *mut AlLayer_ptr,
}

impl AlLayer {
    pub fn new() -> AlLayer {
        let ptr = unsafe { allayer_new() };
        AlLayer { ptr }
    }

    pub fn is_folder(&self) -> bool {
        unsafe { allayer_is_folder(self.ptr) }
    }

    pub fn child_layer(&self) -> Option<AlLayer> {
        let ptr = unsafe { allayer_child_layer(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlLayer { ptr })
        }
    }

    pub fn parent_layer(&self) -> Option<AlLayer> {
        let ptr = unsafe { allayer_parent_layer(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlLayer { ptr })
        }
    }

    pub fn next_layer(&self) -> Option<AlLayer> {
        let ptr = unsafe { allayer_next_layer(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlLayer { ptr })
        }
    }

    pub fn prev_layer(&self) -> Option<AlLayer> {
        let ptr = unsafe { allayer_prev_layer(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlLayer { ptr })
        }
    }

    pub fn create(&mut self, name: &str) -> Result<(), String> {
        let c_name = std::ffi::CString::new(name).unwrap();
        let status = unsafe { allayer_create(self.ptr, c_name.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_number(&mut self, name: &str) -> Result<i32, String> {
        let c_name = std::ffi::CString::new(name).unwrap();
        let mut number: i32 = 0;
        let status = unsafe { allayer_create_number(self.ptr, &mut number, c_name.as_ptr()) };
        if status == statusCode::Success {
            Ok(number)
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_with_number(&mut self, number: i32) -> Result<(), String> {
        let status = unsafe { allayer_create_with_number(self.ptr, number) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_folder(&mut self, name: &str) -> Result<(), String> {
        let c_name = std::ffi::CString::new(name).unwrap();
        let status = unsafe { allayer_create_folder(self.ptr, c_name.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_folder_number(&mut self, name: &str) -> Result<i32, String> {
        let c_name = std::ffi::CString::new(name).unwrap();
        let mut number: i32 = 0;
        let status = unsafe { allayer_create_folder_number(self.ptr, &mut number, c_name.as_ptr()) };
        if status == statusCode::Success {
            Ok(number)
        } else {
            Err(status.to_string())
        }
    }

    pub fn assign_child(&mut self, number: i32) -> Result<(), String> {
        let status = unsafe { allayer_assign_child(self.ptr, number) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn assign_parent(&mut self, number: i32) -> Result<(), String> {
        let status = unsafe { allayer_assign_parent(self.ptr, number) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn assign_sibling_on_right(&mut self, number: i32) -> Result<(), String> {
        let status = unsafe { allayer_assign_sibling_on_right(self.ptr, number) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn assign_sibling_on_left(&mut self, number: i32) -> Result<(), String> {
        let status = unsafe { allayer_assign_sibling_on_left(self.ptr, number) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn invisible(&self) -> bool {
        unsafe { allayer_invisible(self.ptr) }
    }

    pub fn set_invisible(&mut self, invisible: bool) -> Result<(), String> {
        let status = unsafe { allayer_set_invisible(self.ptr, invisible) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn number(&self) -> i32 {
        unsafe { allayer_number(self.ptr) }
    }

    pub fn name(&self) -> String {
        let ptr = unsafe { allayer_name(self.ptr) };
        if ptr.is_null() {
            "".to_string()
        } else {
            unsafe { std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned() }
        }
    }
    /*
    pub fn set_name(&mut self, name: &str) -> Result<(), String> {
        let c_name = std::ffi::CString::new(name).unwrap();
        let status = unsafe { allayer_set_name(self.ptr, c_name.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }
 */
    pub fn pickability(&self) -> Result<i32, String> {
        let mut pick_type: i32 = 0;
        let status = unsafe { allayer_pickability(self.ptr, &mut pick_type) };
        if status == statusCode::Success {
            Ok(pick_type)
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_pickability(&mut self, pick_type: i32) -> Result<(), String> {
        let status = unsafe { allayer_set_pickability(self.ptr, pick_type) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn playback(&self) -> bool {
        unsafe { allayer_playback(self.ptr) }
    }

    pub fn set_playback(&mut self, playback: bool) -> Result<(), String> {
        let status = unsafe { allayer_set_playback(self.ptr, playback) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn draw_instances(&self) -> bool {
        unsafe { allayer_draw_instances(self.ptr) }
    }

    pub fn set_draw_instances(&mut self, draw: bool) -> Result<(), String> {
        let status = unsafe { allayer_set_draw_instances(self.ptr, draw) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn visible_in_layer_bar(&self) -> bool {
        unsafe { allayer_visible_in_layer_bar(self.ptr) }
    }

    pub fn set_visible_in_layer_bar(&mut self, visible: bool) -> Result<(), String> {
        let status = unsafe { allayer_set_visible_in_layer_bar(self.ptr, visible) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn color(&self) -> i32 {
        unsafe { allayer_color(self.ptr) }
    }

    pub fn set_color(&mut self, color: i32) -> Result<(), String> {
        let status = unsafe { allayer_set_color(self.ptr, color) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_custom_color(&mut self, r: u8, g: u8, b: u8, a: u8) -> Result<(), String> {
        let status = unsafe { allayer_set_custom_color(self.ptr, r, g, b, a) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn custom_color(&self) -> Result<(u8, u8, u8, u8), String> {
        let mut r: u8 = 0;
        let mut g: u8 = 0;
        let mut b: u8 = 0;
        let mut a: u8 = 0;
        let status = unsafe { allayer_custom_color(self.ptr, &mut r, &mut g, &mut b, &mut a) };
        if status == statusCode::Success {
            Ok((r, g, b, a))
        } else {
            Err(status.to_string())
        }
    }

    pub fn is_picked(&self) -> bool {
        unsafe { allayer_is_picked(self.ptr) }
    }

    pub fn pick(&mut self) -> Result<(), String> {
        let status = unsafe { allayer_pick(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn unpick(&mut self) -> Result<(), String> {
        let status = unsafe { allayer_unpick(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn start_number() -> i32 {
        unsafe { allayer_start_number() }
    }

    pub fn set_start_number(number: i32) -> Result<(), String> {
        let status = unsafe { allayer_set_start_number(number) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_symmetric(&mut self, symmetric: bool) -> Result<(), String> {
        let status = unsafe { allayer_set_symmetric(self.ptr, symmetric) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn is_symmetric(&self) -> bool {
        unsafe { allayer_is_symmetric(self.ptr) }
    }

    pub fn set_symmetric_origin(&mut self, x: f64, y: f64, z: f64) -> Result<(), String> {
        let status = unsafe { allayer_set_symmetric_origin(self.ptr, x, y, z) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_symmetric_normal(&mut self, x: f64, y: f64, z: f64) -> Result<(), String> {
        let status = unsafe { allayer_set_symmetric_normal(self.ptr, x, y, z) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn symmetric_origin(&self) -> Result<[f64; 3], String> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;
        let status = unsafe { allayer_symmetric_origin(self.ptr, &mut x, &mut y, &mut z) };
        if status == statusCode::Success {
            Ok([x, y, z])
        } else {
            Err(status.to_string())
        }
    }

    pub fn symmetric_normal(&self) -> Result<[f64; 3], String> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;
        let status = unsafe { allayer_symmetric_normal(self.ptr, &mut x, &mut y, &mut z) };
        if status == statusCode::Success {
            Ok([x, y, z])
        } else {
            Err(status.to_string())
        }
    }

    pub fn pick_nodes_on_layer(&mut self) -> Result<(), String> {
        let status = unsafe { allayer_pick_nodes_on_layer(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }
}

impl AlObjectMethods for AlLayer {
    fn as_object_ptr(&self) -> *mut AlObject_ptr {
        self.ptr as *mut AlObject_ptr
    }
    fn set_name(&self, name: &str) -> Result<(), String> {
         let c_name = std::ffi::CString::new(name).unwrap();
        let status = unsafe { allayer_set_name(self.ptr, c_name.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }
}

impl AlPickableMethods for AlLayer {
    fn as_pickable_ptr(&self) -> *mut AlPickable_ptr {
        self.ptr as *mut AlPickable_ptr
    }
}

impl Drop for AlLayer {
    fn drop(&mut self) {
        unsafe {
            allayer_delete(self.ptr);
            self.ptr = std::ptr::null_mut();
        }
    }
}

unsafe extern "C" {
    fn allayer_new() -> *mut AlLayer_ptr;
    fn allayer_delete(layer: *mut AlLayer_ptr);

    fn allayer_is_folder(layer: *mut AlLayer_ptr) -> bool;
    fn allayer_child_layer(layer: *mut AlLayer_ptr) -> *mut AlLayer_ptr;
    fn allayer_parent_layer(layer: *mut AlLayer_ptr) -> *mut AlLayer_ptr;
    fn allayer_next_layer(layer: *mut AlLayer_ptr) -> *mut AlLayer_ptr;
    fn allayer_prev_layer(layer: *mut AlLayer_ptr) -> *mut AlLayer_ptr;

    fn allayer_create(layer: *mut AlLayer_ptr, name: *const i8) -> statusCode;
    fn allayer_create_number(layer: *mut AlLayer_ptr, number: *mut i32, name: *const i8) -> statusCode;
    fn allayer_create_with_number(layer: *mut AlLayer_ptr, number: i32) -> statusCode;

    fn allayer_create_folder(layer: *mut AlLayer_ptr, name: *const i8) -> statusCode;
    fn allayer_create_folder_number(layer: *mut AlLayer_ptr, number: *mut i32, name: *const i8) -> statusCode;

    fn allayer_assign_child(layer: *mut AlLayer_ptr, number: i32) -> statusCode;
    fn allayer_assign_parent(layer: *mut AlLayer_ptr, number: i32) -> statusCode;
    fn allayer_assign_sibling_on_right(layer: *mut AlLayer_ptr, number: i32) -> statusCode;
    fn allayer_assign_sibling_on_left(layer: *mut AlLayer_ptr, number: i32) -> statusCode;

    fn allayer_invisible(layer: *mut AlLayer_ptr) -> bool;
    fn allayer_set_invisible(layer: *mut AlLayer_ptr, invisible: bool) -> statusCode;

    fn allayer_number(layer: *mut AlLayer_ptr) -> i32;

    fn allayer_name(layer: *mut AlLayer_ptr) -> *const i8;
    fn allayer_set_name(layer: *mut AlLayer_ptr, name: *const i8) -> statusCode;

    fn allayer_pickability(layer: *mut AlLayer_ptr, pickType: *mut i32) -> statusCode;
    fn allayer_set_pickability(layer: *mut AlLayer_ptr, pickType: i32) -> statusCode;

    fn allayer_playback(layer: *mut AlLayer_ptr) -> bool;
    fn allayer_set_playback(layer: *mut AlLayer_ptr, playback: bool) -> statusCode;

    fn allayer_draw_instances(layer: *mut AlLayer_ptr) -> bool;
    fn allayer_set_draw_instances(layer: *mut AlLayer_ptr, draw: bool) -> statusCode;

    fn allayer_visible_in_layer_bar(layer: *mut AlLayer_ptr) -> bool;
    fn allayer_set_visible_in_layer_bar(layer: *mut AlLayer_ptr, visible: bool) -> statusCode;

    fn allayer_color(layer: *mut AlLayer_ptr) -> i32;
    fn allayer_set_color(layer: *mut AlLayer_ptr, color: i32) -> statusCode;

    fn allayer_set_custom_color(layer: *mut AlLayer_ptr, r: u8, g: u8, b: u8, a: u8) -> statusCode;
    fn allayer_custom_color(layer: *mut AlLayer_ptr, r: *mut u8, g: *mut u8, b: *mut u8, a: *mut u8) -> statusCode;

    fn allayer_is_picked(layer: *mut AlLayer_ptr) -> bool;
    fn allayer_pick(layer: *mut AlLayer_ptr) -> statusCode;
    fn allayer_unpick(layer: *mut AlLayer_ptr) -> statusCode;

    fn allayer_start_number() -> i32;
    fn allayer_set_start_number(number: i32) -> statusCode;

    fn allayer_set_symmetric(layer: *mut AlLayer_ptr, symmetric: bool) -> statusCode;
    fn allayer_is_symmetric(layer: *mut AlLayer_ptr) -> bool;

    fn allayer_set_symmetric_origin(layer: *mut AlLayer_ptr, x: f64, y: f64, z: f64) -> statusCode;
    fn allayer_set_symmetric_normal(layer: *mut AlLayer_ptr, x: f64, y: f64, z: f64) -> statusCode;
    fn allayer_symmetric_origin(layer: *mut AlLayer_ptr, x: *mut f64, y: *mut f64, z: *mut f64) -> statusCode;
    fn allayer_symmetric_normal(layer: *mut AlLayer_ptr, x: *mut f64, y: *mut f64, z: *mut f64) -> statusCode;

    fn allayer_pick_nodes_on_layer(layer: *mut AlLayer_ptr) -> statusCode;
}