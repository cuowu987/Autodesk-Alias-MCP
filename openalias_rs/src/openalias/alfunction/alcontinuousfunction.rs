use crate::*;
#[repr(C)]
pub struct AlContinuousFunction_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlContinuousFunction {
    pub ptr: *mut AlContinuousFunction_ptr
}
impl AlFunctionMethods for AlContinuousFunction {
    fn as_AlFunction_ptr(&self) -> *mut AlFunction_ptr {
        self.ptr as *mut AlFunction_ptr
    }
}



impl AlContinuousFunction {
    pub fn new() -> AlContinuousFunction {
        let ptr = unsafe { alcontinuousfunction_new() };
        AlContinuousFunction { ptr }
    }

    pub fn create(
        &mut self,
        init: Option<AlUndoCallbackType>,
        down: Option<AlMouseButtonFunction>,
        move_func: Option<AlMouseButtonFunction>,
        up: Option<AlMouseButtonFunction>,
        cleanup: Option<AlUndoCallbackType>,
        manipulates_pick_list: bool,
    ) -> Result<(), String> {
        let status = unsafe {
            alcontinuousfunction_create(
                self.ptr,
                init,
                down,
                move_func,
                up,
                cleanup,
                manipulates_pick_list,
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_with_command(
        &mut self,
        command: &str,
        init: Option<AlUndoCallbackType>,
        down: Option<AlMouseButtonFunction>,
        move_func: Option<AlMouseButtonFunction>,
        up: Option<AlMouseButtonFunction>,
        cleanup: Option<AlUndoCallbackType>,
        manipulates_pick_list: bool,
    ) -> Result<(), String> {
        let c_command = std::ffi::CString::new(command).unwrap();
        let status = unsafe {
            alcontinuousfunction_create_with_command(
                self.ptr,
                c_command.as_ptr(),
                init,
                down,
                move_func,
                up,
                cleanup,
                manipulates_pick_list,
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_pre_init_function(&mut self, pre_init: Option<extern "C" fn()>) -> Result<(), String> {
        let status = unsafe { alcontinuousfunction_set_pre_init_function(self.ptr, pre_init) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_post_cleanup_function(
        &mut self,
        post_cleanup: Option<extern "C" fn()>,
    ) -> Result<(), String> {
        let status =
            unsafe { alcontinuousfunction_set_post_cleanup_function(self.ptr, post_cleanup) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_prompt_static(
        &mut self,
        static_prompt: &str,
        filter_type: AlFilterType,
    ) -> Result<(), String> {
        let c_prompt = std::ffi::CString::new(static_prompt).unwrap();
        let status = unsafe {
            alcontinuousfunction_set_prompt_static(self.ptr, c_prompt.as_ptr(), filter_type)
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_prompt_dynamic(
        &mut self,
        output_string_func: Option<extern "C" fn() -> *const i8>,
        filter_type: AlFilterType,
    ) -> Result<(), String> {
        let status = unsafe {
            alcontinuousfunction_set_prompt_dynamic(self.ptr, output_string_func, filter_type)
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn update_prompt(&self, prompt: &str, filter_type: AlFilterType) -> Result<(), String> {
        let c_prompt = std::ffi::CString::new(prompt).unwrap();
        let status =
            unsafe { alcontinuousfunction_update_prompt(self.ptr, c_prompt.as_ptr(), filter_type) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn get_prompt_input(&self) -> Option<String> {
        let c_str = unsafe { alcontinuousfunction_get_prompt_input(self.ptr) };
        if c_str.is_null() {
            None
        } else {
            Some(unsafe {
                std::ffi::CStr::from_ptr(c_str)
                    .to_string_lossy()
                    .to_string()
            })
        }
    }

    pub fn set_behaviour(&self, behaviour_type: AlBehaviourType) -> Result<(), String> {
        let status = unsafe { alcontinuousfunction_set_behaviour(self.ptr, behaviour_type) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_mouse_coordinate_type(
        &self,
        coordinate_type: AlCoordinateType,
    ) -> Result<(), String> {
        let status =
            unsafe { alcontinuousfunction_set_mouse_coordinate_type(self.ptr, coordinate_type) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn behaviour(&self) -> AlBehaviourType {
        unsafe { alcontinuousfunction_behaviour(self.ptr) }
    }

    pub fn is_active_continuous_function(&self) -> bool {
        unsafe { alcontinuousfunction_is_active_continuous_function(self.ptr) }
    }

    pub fn finished(&self) -> Result<(), String> {
        let status = unsafe { alcontinuousfunction_finished(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_go_button_simple(pressed: Option<extern "C" fn()>) -> Result<(), String> {
        let status = unsafe { alcontinuousfunction_create_go_button_simple(pressed) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_go_button_ext(
        pressed: Option<extern "C" fn() -> i32>,
        persistence: bool,
        button1: &str,
        button2: &str,
        button3: &str,
        button4: &str,
        button5: &str,
    ) -> Result<(), String> {
        let c_button1 = std::ffi::CString::new(button1).unwrap();
        let c_button2 = std::ffi::CString::new(button2).unwrap();
        let c_button3 = std::ffi::CString::new(button3).unwrap();
        let c_button4 = std::ffi::CString::new(button4).unwrap();
        let c_button5 = std::ffi::CString::new(button5).unwrap();
        let status = unsafe {
            alcontinuousfunction_create_go_button_ext(
                pressed,
                persistence,
                c_button1.as_ptr(),
                c_button2.as_ptr(),
                c_button3.as_ptr(),
                c_button4.as_ptr(),
                c_button5.as_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn clear_go_button(do_redraw: bool) -> Result<(), String> {
        let status = unsafe { alcontinuousfunction_clear_go_button(do_redraw) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn enable_go_button(button: &str, enable: bool) -> Result<(), String> {
        let c_button = std::ffi::CString::new(button).unwrap();
        let status = unsafe { alcontinuousfunction_enable_go_button(c_button.as_ptr(), enable) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn go_button_pressed() -> Option<String> {
        let c_str = unsafe { alcontinuousfunction_go_button_pressed() };
        if c_str.is_null() {
            None
        } else {
            Some(unsafe {
                std::ffi::CStr::from_ptr(c_str)
                    .to_string_lossy()
                    .to_string()
            })
        }
    }

    pub fn set_undo_function(func: AlUndoCallbackType) -> Result<(), String> {
        let status = unsafe { alcontinuousfunction_set_undo_function(func) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn undo_function() -> AlUndoCallbackType {
        unsafe { alcontinuousfunction_undo_function() }
    }

    pub fn init_pick_box(x: i32, y: i32) {
        unsafe { alcontinuousfunction_init_pick_box(x, y) }
    }

    pub fn move_pick_box(x: i32, y: i32) {
        unsafe { alcontinuousfunction_move_pick_box(x, y) }
    }

    pub fn end_pick_box(x: i32, y: i32) {
        unsafe { alcontinuousfunction_end_pick_box(x, y) }
    }

    pub fn keyboard_coordinate_mode() -> AlCoordinateType {
        unsafe { alcontinuousfunction_keyboard_coordinate_mode() }
    }

    pub fn translate_input(event: i32, out_button: &mut i32) -> AlInputType {
        unsafe { alcontinuousfunction_translate_input(event, out_button) }
    }

    pub fn input_modifier_mask() -> i32 {
        unsafe { alcontinuousfunction_input_modifier_mask() }
    }
}




unsafe extern "C" {
    fn alcontinuousfunction_new() -> *mut AlContinuousFunction_ptr;
    fn alcontinuousfunction_create(
        func: *mut AlContinuousFunction_ptr,
        init: Option<AlUndoCallbackType>,
        down: Option<AlMouseButtonFunction>,
        move_func: Option<AlMouseButtonFunction>,
        up: Option<AlMouseButtonFunction>,
        cleanup: Option<AlUndoCallbackType>,
        manipulates_pick_list: bool,
    ) -> statusCode;
    fn alcontinuousfunction_create_with_command(
        func: *mut AlContinuousFunction_ptr,
        command: *const i8,
        init: Option<AlUndoCallbackType>,
        down: Option<AlMouseButtonFunction>,
        move_func: Option<AlMouseButtonFunction>,
        up: Option<AlMouseButtonFunction>,
        cleanup: Option<AlUndoCallbackType>,
        manipulates_pick_list: bool,
    ) -> statusCode;
    fn alcontinuousfunction_set_pre_init_function(
        func: *mut AlContinuousFunction_ptr,
        pre_init: Option<extern "C" fn()>,
    ) -> statusCode;
    fn alcontinuousfunction_set_post_cleanup_function(
        func: *mut AlContinuousFunction_ptr,
        post_cleanup: Option<extern "C" fn()>,
    ) -> statusCode;
    fn alcontinuousfunction_set_prompt_static(
        func: *mut AlContinuousFunction_ptr,
        static_prompt: *const i8,
        filter_type: AlFilterType,
    ) -> statusCode;
    fn alcontinuousfunction_set_prompt_dynamic(
        func: *mut AlContinuousFunction_ptr,
        output_string_func: Option<extern "C" fn() -> *const i8>,
        filter_type: AlFilterType,
    ) -> statusCode;
    fn alcontinuousfunction_update_prompt(
        func: *mut AlContinuousFunction_ptr,
        prompt: *const i8,
        filter_type: AlFilterType,
    ) -> statusCode;
    fn alcontinuousfunction_get_prompt_input(func: *mut AlContinuousFunction_ptr) -> *const i8;
    fn alcontinuousfunction_set_behaviour(
        func: *mut AlContinuousFunction_ptr,
        behaviour_type: AlBehaviourType,
    ) -> statusCode;
    fn alcontinuousfunction_set_mouse_coordinate_type(
        func: *mut AlContinuousFunction_ptr,
        coordinate_type: AlCoordinateType,
    ) -> statusCode;
    fn alcontinuousfunction_behaviour(func: *const AlContinuousFunction_ptr) -> AlBehaviourType;
    fn alcontinuousfunction_create_go_button_simple(pressed: Option<extern "C" fn()>)
    -> statusCode;
    fn alcontinuousfunction_create_go_button_ext(
        pressed: Option<extern "C" fn() -> i32>,
        persistence: bool,
        button1: *const i8,
        button2: *const i8,
        button3: *const i8,
        button4: *const i8,
        button5: *const i8,
    ) -> statusCode;
    fn alcontinuousfunction_clear_go_button(do_redraw: bool) -> statusCode;
    fn alcontinuousfunction_enable_go_button(button: *const i8, enable: bool) -> statusCode;
    fn alcontinuousfunction_go_button_pressed() -> *const i8;
    fn alcontinuousfunction_set_undo_function(func: AlUndoCallbackType) -> statusCode;
    fn alcontinuousfunction_undo_function() -> AlUndoCallbackType;
    fn alcontinuousfunction_is_active_continuous_function(
        func: *const AlContinuousFunction_ptr,
    ) -> bool;
    fn alcontinuousfunction_init_pick_box(x: i32, y: i32);
    fn alcontinuousfunction_move_pick_box(x: i32, y: i32);
    fn alcontinuousfunction_end_pick_box(x: i32, y: i32);
    fn alcontinuousfunction_keyboard_coordinate_mode() -> AlCoordinateType;
    fn alcontinuousfunction_translate_input(event: i32, out_button: *mut i32) -> AlInputType;
    fn alcontinuousfunction_input_modifier_mask() -> i32;
    fn alcontinuousfunction_finished(func: *mut AlContinuousFunction_ptr) -> statusCode;
}