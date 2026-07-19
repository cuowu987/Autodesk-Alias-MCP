use crate::*;

#[repr(C)]
pub struct AlFunctionHandle_ptr {
    _private: [u8; 0],
}

#[derive(Debug)]
pub struct AlFunctionHandle {
    pub ptr: *mut AlFunctionHandle_ptr,
}

impl AlFunctionHandle {
    pub fn new() -> AlFunctionHandle {
        let ptr = unsafe { alfunctionhandle_create() };
        AlFunctionHandle { ptr }
    }

    pub fn create_with_names(&mut self, command: &str, function_name: &str) -> Result<(), String> {
        let c_command = std::ffi::CString::new(command).unwrap();
        let c_function_name = std::ffi::CString::new(function_name).unwrap();
        let status = unsafe {
            alfunctionhandle_create_with_names(
                self.ptr,
                c_command.as_ptr(),
                c_function_name.as_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_with_function<C>(&mut self, command: &str, func: &C) -> Result<(), String>
    where
        C: AlFunctionMethods,
    {
        let c_command = std::ffi::CString::new(command).unwrap();
        let status = unsafe {
            alfunctionhandle_create_with_function(
                self.ptr,
                c_command.as_ptr(),
                func.as_AlFunction_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn assign(&mut self, other: &AlFunctionHandle) {
        unsafe { alfunctionhandle_assign(self.ptr, other.ptr) }
    }

    pub fn is_valid(&self) -> bool {
        unsafe { alfunctionhandle_is_valid(self.ptr) }
    }

    pub fn set_attribute_string(&self, attr_string: &str) -> Result<(), String> {
        let c_attr = std::ffi::CString::new(attr_string).unwrap();
        let status = unsafe { alfunctionhandle_set_attribute_string(self.ptr, c_attr.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_tool_title(&self, tool_title: &str) -> Result<(), String> {
        let c_title = std::ffi::CString::new(tool_title).unwrap();
        let status = unsafe { alfunctionhandle_set_tool_title(self.ptr, c_title.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_option_box(&self, title: &str, function_name: &str) -> Result<(), String> {
        let c_title = std::ffi::CString::new(title).unwrap();
        let c_function = std::ffi::CString::new(function_name).unwrap();
        let status = unsafe {
            alfunctionhandle_set_option_box(self.ptr, c_title.as_ptr(), c_function.as_ptr())
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_custom_icon(&self, abs_path: &str) -> Result<(), String> {
        let c_path = std::ffi::CString::new(abs_path).unwrap();
        let status = unsafe { alfunctionhandle_set_custom_icon(self.ptr, c_path.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_to_menu(&self, menu_path: &str) -> Result<(), String> {
        let c_path = std::ffi::CString::new(menu_path).unwrap();
        let status = unsafe { alfunctionhandle_add_to_menu(self.ptr, c_path.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn append_to_menu(&self, menu_path: &str) -> Result<(), String> {
        let c_path = std::ffi::CString::new(menu_path).unwrap();
        let status = unsafe { alfunctionhandle_append_to_menu(self.ptr, c_path.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn install_on_menu(&self, menu_path: &str, before: bool) -> Result<(), String> {
        let c_path = std::ffi::CString::new(menu_path).unwrap();
        let status = unsafe { alfunctionhandle_install_on_menu(self.ptr, c_path.as_ptr(), before) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn install_on_submenu(&self, submenu_name: &str) -> Result<(), String> {
        let c_name = std::ffi::CString::new(submenu_name).unwrap();
        let status = unsafe { alfunctionhandle_install_on_submenu(self.ptr, c_name.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_separator(&self, menu_path: &str) -> Result<(), String> {
        let c_path = std::ffi::CString::new(menu_path).unwrap();
        let status = unsafe { alfunctionhandle_add_separator(self.ptr, c_path.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn remove_separator(&self, menu_path: &str) -> Result<(), String> {
        let c_path = std::ffi::CString::new(menu_path).unwrap();
        let status = unsafe { alfunctionhandle_remove_separator(self.ptr, c_path.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn remove_from_menu(&self) -> Result<(), String> {
        let status = unsafe { alfunctionhandle_remove_from_menu(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn delete_object(&self) -> Result<(), String> {
        let status = unsafe { alfunctionhandle_delete_object(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }
}

impl Drop for AlFunctionHandle {
    fn drop(&mut self) {
        unsafe {
            alfunctionhandle_destroy(self.ptr);
            self.ptr = std::ptr::null_mut();
        }
    }
}

unsafe extern "C" {
    fn alfunctionhandle_create() -> *mut AlFunctionHandle_ptr;
    fn alfunctionhandle_destroy(handle: *mut AlFunctionHandle_ptr);
    fn alfunctionhandle_create_with_names(
        handle: *mut AlFunctionHandle_ptr,
        menu_path: *const i8,
        function_name: *const i8,
    ) -> statusCode;
    fn alfunctionhandle_create_with_function(
        handle: *mut AlFunctionHandle_ptr,
        menu_path: *const i8,
        func: *mut AlFunction_ptr,
    ) -> statusCode;
    fn alfunctionhandle_assign(
        handle: *mut AlFunctionHandle_ptr,
        other: *const AlFunctionHandle_ptr,
    );
    fn alfunctionhandle_is_valid(handle: *const AlFunctionHandle_ptr) -> bool;
    fn alfunctionhandle_set_attribute_string(
        handle: *mut AlFunctionHandle_ptr,
        attr_string: *const i8,
    ) -> statusCode;
    fn alfunctionhandle_set_tool_title(
        handle: *mut AlFunctionHandle_ptr,
        tool_title: *const i8,
    ) -> statusCode;
    fn alfunctionhandle_set_option_box(
        handle: *mut AlFunctionHandle_ptr,
        title: *const i8,
        function_name: *const i8,
    ) -> statusCode;
    fn alfunctionhandle_set_custom_icon(
        handle: *mut AlFunctionHandle_ptr,
        abs_path: *const i8,
    ) -> statusCode;
    fn alfunctionhandle_add_to_menu(
        handle: *mut AlFunctionHandle_ptr,
        menu_path: *const i8,
    ) -> statusCode;
    fn alfunctionhandle_append_to_menu(
        handle: *mut AlFunctionHandle_ptr,
        menu_path: *const i8,
    ) -> statusCode;
    fn alfunctionhandle_install_on_menu(
        handle: *mut AlFunctionHandle_ptr,
        menu_path: *const i8,
        before: bool,
    ) -> statusCode;
    fn alfunctionhandle_install_on_submenu(
        handle: *mut AlFunctionHandle_ptr,
        submenu_name: *const i8,
    ) -> statusCode;
    fn alfunctionhandle_add_separator(
        handle: *mut AlFunctionHandle_ptr,
        menu_path: *const i8,
    ) -> statusCode;
    fn alfunctionhandle_remove_separator(
        handle: *mut AlFunctionHandle_ptr,
        menu_path: *const i8,
    ) -> statusCode;
    fn alfunctionhandle_remove_from_menu(handle: *mut AlFunctionHandle_ptr) -> statusCode;
    fn alfunctionhandle_delete_object(handle: *mut AlFunctionHandle_ptr) -> statusCode;

}
