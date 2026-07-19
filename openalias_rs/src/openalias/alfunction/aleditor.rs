use crate::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct APIColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

pub type APIVoidCallback = extern "C" fn();
pub type APIBoolCallback = extern "C" fn(new_value: bool);
pub type APIIntCallback = extern "C" fn(new_value: i32);
pub type APIDoubleCallback = extern "C" fn(new_value: f64);
pub type APICharCallback = extern "C" fn(new_value: *const i8);
pub type APIColorCallback = extern "C" fn(new_value: APIColor);

#[repr(C)]
pub struct AlEditor_ptr {
    _private: [u8; 0],
}

#[derive(Debug)]
pub struct AlEditor {
    pub ptr: *mut AlEditor_ptr,
}

impl AlEditor {
    pub fn new(title: &str, function_name: &str) -> Result<Self, String> {
        let c_title = std::ffi::CString::new(title).unwrap();
        let c_function_name = std::ffi::CString::new(function_name).unwrap();
        let ptr = unsafe { aleditor_new(c_title.as_ptr(), c_function_name.as_ptr()) };
        if ptr.is_null() {
            return Err("Failed to create editor".into());
        }
        Ok(Self { ptr })
    }

    pub fn create(&self) -> Result<(), String> {
        let status = unsafe { aleditor_create(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn open(&self) -> Result<(), String> {
        let status = unsafe { aleditor_open(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn close(&self) -> Result<(), String> {
        let status = unsafe { aleditor_close(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_separator(&self) -> Result<(), String> {
        let status = unsafe { aleditor_add_separator(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_group(&self, unique_title: &str) -> Result<(), String> {
        let c_title = std::ffi::CString::new(unique_title).unwrap();
        let status = unsafe { aleditor_add_group(self.ptr, c_title.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn end_group(&self) -> Result<(), String> {
        let status = unsafe { aleditor_end_group(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_string(
        &self,
        title: &str,
        default_value: &str,
        func: APICharCallback,
        symbol_name: &str,
    ) -> Result<(), String> {
        let c_title = std::ffi::CString::new(title).unwrap();
        let c_default = std::ffi::CString::new(default_value).unwrap();
        let c_symbol = std::ffi::CString::new(symbol_name).unwrap();
        let status = unsafe {
            aleditor_add_string(
                self.ptr,
                c_title.as_ptr(),
                c_default.as_ptr(),
                func,
                c_symbol.as_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_string_no_symbol(
        &self,
        title: &str,
        default_value: &str,
        func: APICharCallback,
    ) -> Result<(), String> {
        let c_title = std::ffi::CString::new(title).unwrap();
        let c_default = std::ffi::CString::new(default_value).unwrap();
        let status = unsafe {
            aleditor_add_string_no_symbol(self.ptr, c_title.as_ptr(), c_default.as_ptr(), func)
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_read_only_string(
        &self,
        title: &str,
        value: &str,
        symbol_name: &str,
    ) -> Result<(), String> {
        let c_title = std::ffi::CString::new(title).unwrap();
        let c_value = std::ffi::CString::new(value).unwrap();
        let c_symbol = std::ffi::CString::new(symbol_name).unwrap();
        let status = unsafe {
            aleditor_add_read_only_string(
                self.ptr,
                c_title.as_ptr(),
                c_value.as_ptr(),
                c_symbol.as_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_multiline_string(
        &self,
        title: &str,
        default_value: &str,
        func: APICharCallback,
        readonly: bool,
        max_length: usize,
        symbol_name: &str,
    ) -> Result<(), String> {
        let c_title = std::ffi::CString::new(title).unwrap();
        let c_default = std::ffi::CString::new(default_value).unwrap();
        let c_symbol = std::ffi::CString::new(symbol_name).unwrap();
        let status = unsafe {
            aleditor_add_multiline_string(
                self.ptr,
                c_title.as_ptr(),
                c_default.as_ptr(),
                func,
                readonly,
                max_length,
                c_symbol.as_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_button(&self, title: &str, func: APIVoidCallback) -> Result<(), String> {
        let c_title = std::ffi::CString::new(title).unwrap();
        let status = unsafe { aleditor_add_button(self.ptr, c_title.as_ptr(), func) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_button_auto_close(
        &self,
        title: &str,
        func: APIVoidCallback,
        auto_close: bool,
    ) -> Result<(), String> {
        let c_title = std::ffi::CString::new(title).unwrap();
        let status =
            unsafe { aleditor_add_button_auto_close(self.ptr, c_title.as_ptr(), func, auto_close) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_go_button(&self) -> Result<(), String> {
        let status = unsafe { aleditor_add_go_button(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_cancel_button(&self) -> Result<(), String> {
        let status = unsafe { aleditor_add_cancel_button(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_next_button(&self) -> Result<(), String> {
        let status = unsafe { aleditor_add_next_button(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_reset_button(&self) -> Result<(), String> {
        let status = unsafe { aleditor_add_reset_button(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_text_button(&self, title: &str, func: APIVoidCallback) -> Result<(), String> {
        let c_title = std::ffi::CString::new(title).unwrap();
        let status = unsafe { aleditor_add_text_button(self.ptr, c_title.as_ptr(), func) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_checkbox(
        &self,
        title: &str,
        default_value: bool,
        func: APIBoolCallback,
        symbol_name: &str,
    ) -> Result<(), String> {
        let c_title = std::ffi::CString::new(title).unwrap();
        let c_symbol = std::ffi::CString::new(symbol_name).unwrap();
        let status = unsafe {
            aleditor_add_checkbox(
                self.ptr,
                c_title.as_ptr(),
                default_value,
                func,
                c_symbol.as_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_checkbox_no_symbol(
        &self,
        title: &str,
        default_value: bool,
        func: APIBoolCallback,
    ) -> Result<(), String> {
        let c_title = std::ffi::CString::new(title).unwrap();
        let status = unsafe {
            aleditor_add_checkbox_no_symbol(self.ptr, c_title.as_ptr(), default_value, func)
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_popup(
        &self,
        title: &str,
        items: &str,
        default_value: i32,
        func: APIIntCallback,
        symbol_name: &str,
    ) -> Result<(), String> {
        let c_title = std::ffi::CString::new(title).unwrap();
        let c_items = std::ffi::CString::new(items).unwrap();
        let c_symbol = std::ffi::CString::new(symbol_name).unwrap();
        let status = unsafe {
            aleditor_add_popup(
                self.ptr,
                c_title.as_ptr(),
                c_items.as_ptr(),
                default_value,
                func,
                c_symbol.as_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_popup_no_symbol(
        &self,
        title: &str,
        items: &str,
        default_value: i32,
        func: APIIntCallback,
    ) -> Result<(), String> {
        let c_title = std::ffi::CString::new(title).unwrap();
        let c_items = std::ffi::CString::new(items).unwrap();
        let status = unsafe {
            aleditor_add_popup_no_symbol(
                self.ptr,
                c_title.as_ptr(),
                c_items.as_ptr(),
                default_value,
                func,
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_radio(
        &self,
        title: &str,
        items: &str,
        default_value: i32,
        func: APIIntCallback,
        symbol_name: &str,
    ) -> Result<(), String> {
        let c_title = std::ffi::CString::new(title).unwrap();
        let c_items = std::ffi::CString::new(items).unwrap();
        let c_symbol = std::ffi::CString::new(symbol_name).unwrap();
        let status = unsafe {
            aleditor_add_radio(
                self.ptr,
                c_title.as_ptr(),
                c_items.as_ptr(),
                default_value,
                func,
                c_symbol.as_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_radio_no_symbol(
        &self,
        title: &str,
        items: &str,
        default_value: i32,
        func: APIIntCallback,
    ) -> Result<(), String> {
        let c_title = std::ffi::CString::new(title).unwrap();
        let c_items = std::ffi::CString::new(items).unwrap();
        let status = unsafe {
            aleditor_add_radio_no_symbol(
                self.ptr,
                c_title.as_ptr(),
                c_items.as_ptr(),
                default_value,
                func,
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_int_slider(
        &self,
        title: &str,
        default_value: i32,
        slider_min: i32,
        slider_max: i32,
        func: APIIntCallback,
        symbol_name: &str,
    ) -> Result<(), String> {
        let c_title = std::ffi::CString::new(title).unwrap();
        let c_symbol = std::ffi::CString::new(symbol_name).unwrap();
        let status = unsafe {
            aleditor_add_int_slider(
                self.ptr,
                c_title.as_ptr(),
                default_value,
                slider_min,
                slider_max,
                func,
                c_symbol.as_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_int_slider_no_symbol(
        &self,
        title: &str,
        default_value: i32,
        slider_min: i32,
        slider_max: i32,
        func: APIIntCallback,
    ) -> Result<(), String> {
        let c_title = std::ffi::CString::new(title).unwrap();
        let status = unsafe {
            aleditor_add_int_slider_no_symbol(
                self.ptr,
                c_title.as_ptr(),
                default_value,
                slider_min,
                slider_max,
                func,
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_double_slider(
        &self,
        title: &str,
        default_value: f64,
        slider_min: f64,
        slider_max: f64,
        func: APIDoubleCallback,
        symbol_name: &str,
    ) -> Result<(), String> {
        let c_title = std::ffi::CString::new(title).unwrap();
        let c_symbol = std::ffi::CString::new(symbol_name).unwrap();
        let status = unsafe {
            aleditor_add_double_slider(
                self.ptr,
                c_title.as_ptr(),
                default_value,
                slider_min,
                slider_max,
                func,
                c_symbol.as_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_double_slider_no_symbol(
        &self,
        title: &str,
        default_value: f64,
        slider_min: f64,
        slider_max: f64,
        func: APIDoubleCallback,
    ) -> Result<(), String> {
        let c_title = std::ffi::CString::new(title).unwrap();
        let status = unsafe {
            aleditor_add_double_slider_no_symbol(
                self.ptr,
                c_title.as_ptr(),
                default_value,
                slider_min,
                slider_max,
                func,
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_double_slider_precision(
        &self,
        title: &str,
        default_value: f64,
        slider_min: f64,
        slider_max: f64,
        func: APIDoubleCallback,
        precision: i32,
        symbol_name: &str,
    ) -> Result<(), String> {
        let c_title = std::ffi::CString::new(title).unwrap();
        let c_symbol = std::ffi::CString::new(symbol_name).unwrap();
        let status = unsafe {
            aleditor_add_double_slider_precision(
                self.ptr,
                c_title.as_ptr(),
                default_value,
                slider_min,
                slider_max,
                func,
                precision,
                c_symbol.as_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_color(
        &self,
        title: &str,
        default_value: APIColor,
        func: APIColorCallback,
        symbol_name: &str,
    ) -> Result<(), String> {
        let c_title = std::ffi::CString::new(title).unwrap();
        let c_symbol = std::ffi::CString::new(symbol_name).unwrap();
        let status = unsafe {
            aleditor_add_color(
                self.ptr,
                c_title.as_ptr(),
                default_value,
                func,
                c_symbol.as_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_color_no_symbol(
        &self,
        title: &str,
        default_value: APIColor,
        func: APIColorCallback,
    ) -> Result<(), String> {
        let c_title = std::ffi::CString::new(title).unwrap();
        let status = unsafe {
            aleditor_add_color_no_symbol(self.ptr, c_title.as_ptr(), default_value, func)
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_file_browser(
        &self,
        title: &str,
        dialog_title: &str,
        initial_dir: &str,
        func: APICharCallback,
        symbol_name: &str,
    ) -> Result<(), String> {
        let c_title = std::ffi::CString::new(title).unwrap();
        let c_dialog_title = std::ffi::CString::new(dialog_title).unwrap();
        let c_initial_dir = std::ffi::CString::new(initial_dir).unwrap();
        let c_symbol = std::ffi::CString::new(symbol_name).unwrap();
        let status = unsafe {
            aleditor_add_file_browser(
                self.ptr,
                c_title.as_ptr(),
                c_dialog_title.as_ptr(),
                c_initial_dir.as_ptr(),
                func,
                c_symbol.as_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_folder_browser(
        &self,
        title: &str,
        dialog_title: &str,
        initial_dir: &str,
        func: APICharCallback,
        symbol_name: &str,
    ) -> Result<(), String> {
        let c_title = std::ffi::CString::new(title).unwrap();
        let c_dialog_title = std::ffi::CString::new(dialog_title).unwrap();
        let c_initial_dir = std::ffi::CString::new(initial_dir).unwrap();
        let c_symbol = std::ffi::CString::new(symbol_name).unwrap();
        let status = unsafe {
            aleditor_add_folder_browser(
                self.ptr,
                c_title.as_ptr(),
                c_dialog_title.as_ptr(),
                c_initial_dir.as_ptr(),
                func,
                c_symbol.as_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_image_file(&self, file_name: &str, symbol_name: &str) -> Result<(), String> {
        let c_file_name = std::ffi::CString::new(file_name).unwrap();
        let c_symbol = std::ffi::CString::new(symbol_name).unwrap();
        let status =
            unsafe { aleditor_add_image_file(self.ptr, c_file_name.as_ptr(), c_symbol.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_image_data(
        &self,
        img_width: i32,
        img_height: i32,
        image: &[u8],
        symbol_name: &str,
    ) -> Result<(), String> {
        let c_symbol = std::ffi::CString::new(symbol_name).unwrap();
        let status = unsafe {
            aleditor_add_image_data(
                self.ptr,
                img_width,
                img_height,
                image.as_ptr(),
                c_symbol.as_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_enabled(&self, title: &str, enabled: bool) -> Result<(), String> {
        let c_title = std::ffi::CString::new(title).unwrap();
        let status = unsafe { aleditor_set_enabled(self.ptr, c_title.as_ptr(), enabled) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_visible(&self, title: &str, visible: bool) -> Result<(), String> {
        let c_title = std::ffi::CString::new(title).unwrap();
        let status = unsafe { aleditor_set_visible(self.ptr, c_title.as_ptr(), visible) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_read_only_string(&self, title: &str, new_value: &str) -> Result<(), String> {
        let c_title = std::ffi::CString::new(title).unwrap();
        let c_value = std::ffi::CString::new(new_value).unwrap();
        let status =
            unsafe { aleditor_set_read_only_string(self.ptr, c_title.as_ptr(), c_value.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_image_file(&self, symbol_name: &str, file_name: &str) -> Result<(), String> {
        let c_symbol = std::ffi::CString::new(symbol_name).unwrap();
        let c_file_name = std::ffi::CString::new(file_name).unwrap();
        let status =
            unsafe { aleditor_set_image_file(self.ptr, c_symbol.as_ptr(), c_file_name.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_image_data(
        &self,
        symbol_name: &str,
        img_width: i32,
        img_height: i32,
        image: &[u8],
    ) -> Result<(), String> {
        let c_symbol = std::ffi::CString::new(symbol_name).unwrap();
        let status = unsafe {
            aleditor_set_image_data(
                self.ptr,
                c_symbol.as_ptr(),
                img_width,
                img_height,
                image.as_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_group_expanded(&self, unique_title: &str, expanded: bool) -> Result<(), String> {
        let c_title = std::ffi::CString::new(unique_title).unwrap();
        let status = unsafe { aleditor_set_group_expanded(self.ptr, c_title.as_ptr(), expanded) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_title(&self, symbol_name: &str, title: &str) -> Result<(), String> {
        let c_symbol = std::ffi::CString::new(symbol_name).unwrap();
        let c_title = std::ffi::CString::new(title).unwrap();
        let status = unsafe { aleditor_set_title(self.ptr, c_symbol.as_ptr(), c_title.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_popup_items(
        &self,
        symbol_name: &str,
        items: &str,
        value: i32,
    ) -> Result<(), String> {
        let c_symbol = std::ffi::CString::new(symbol_name).unwrap();
        let c_items = std::ffi::CString::new(items).unwrap();
        let status = unsafe {
            aleditor_set_popup_items(self.ptr, c_symbol.as_ptr(), c_items.as_ptr(), value)
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_int_slider_range(
        &self,
        symbol_name: &str,
        slider_min: i32,
        slider_max: i32,
    ) -> Result<(), String> {
        let c_symbol = std::ffi::CString::new(symbol_name).unwrap();
        let status = unsafe {
            aleditor_set_int_slider_range(self.ptr, c_symbol.as_ptr(), slider_min, slider_max)
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_double_slider_range(
        &self,
        symbol_name: &str,
        slider_min: f64,
        slider_max: f64,
    ) -> Result<(), String> {
        let c_symbol = std::ffi::CString::new(symbol_name).unwrap();
        let status = unsafe {
            aleditor_set_double_slider_range(self.ptr, c_symbol.as_ptr(), slider_min, slider_max)
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }
}

impl Drop for AlEditor {
    fn drop(&mut self) {
        unsafe {
            aleditor_destroy(self.ptr);
            self.ptr = std::ptr::null_mut();
        }
    }
}

unsafe extern "C" {
    fn aleditor_new(title: *const i8, function_name: *const i8) -> *mut AlEditor_ptr;
    fn aleditor_destroy(editor: *mut AlEditor_ptr);
    fn aleditor_create(editor: *mut AlEditor_ptr) -> statusCode;
    fn aleditor_open(editor: *mut AlEditor_ptr) -> statusCode;
    fn aleditor_close(editor: *mut AlEditor_ptr) -> statusCode;
    fn aleditor_add_separator(editor: *mut AlEditor_ptr) -> statusCode;
    fn aleditor_add_group(editor: *mut AlEditor_ptr, unique_title: *const i8) -> statusCode;
    fn aleditor_end_group(editor: *mut AlEditor_ptr) -> statusCode;
    fn aleditor_add_string(
        editor: *mut AlEditor_ptr,
        title: *const i8,
        default_value: *const i8,
        func: APICharCallback,
        symbol_name: *const i8,
    ) -> statusCode;
    fn aleditor_add_string_no_symbol(
        editor: *mut AlEditor_ptr,
        title: *const i8,
        default_value: *const i8,
        func: APICharCallback,
    ) -> statusCode;
    fn aleditor_add_read_only_string(
        editor: *mut AlEditor_ptr,
        title: *const i8,
        value: *const i8,
        symbol_name: *const i8,
    ) -> statusCode;
    fn aleditor_add_multiline_string(
        editor: *mut AlEditor_ptr,
        title: *const i8,
        default_value: *const i8,
        func: APICharCallback,
        readonly: bool,
        max_length: usize,
        symbol_name: *const i8,
    ) -> statusCode;
    fn aleditor_add_button(
        editor: *mut AlEditor_ptr,
        title: *const i8,
        func: APIVoidCallback,
    ) -> statusCode;
    fn aleditor_add_button_auto_close(
        editor: *mut AlEditor_ptr,
        title: *const i8,
        func: APIVoidCallback,
        auto_close: bool,
    ) -> statusCode;
    fn aleditor_add_go_button(editor: *mut AlEditor_ptr) -> statusCode;
    fn aleditor_add_cancel_button(editor: *mut AlEditor_ptr) -> statusCode;
    fn aleditor_add_next_button(editor: *mut AlEditor_ptr) -> statusCode;
    fn aleditor_add_reset_button(editor: *mut AlEditor_ptr) -> statusCode;
    fn aleditor_add_text_button(
        editor: *mut AlEditor_ptr,
        title: *const i8,
        func: APIVoidCallback,
    ) -> statusCode;
    fn aleditor_add_checkbox(
        editor: *mut AlEditor_ptr,
        title: *const i8,
        default_value: bool,
        func: APIBoolCallback,
        symbol_name: *const i8,
    ) -> statusCode;
    fn aleditor_add_checkbox_no_symbol(
        editor: *mut AlEditor_ptr,
        title: *const i8,
        default_value: bool,
        func: APIBoolCallback,
    ) -> statusCode;
    fn aleditor_add_popup(
        editor: *mut AlEditor_ptr,
        title: *const i8,
        items: *const i8,
        default_value: i32,
        func: APIIntCallback,
        symbol_name: *const i8,
    ) -> statusCode;
    fn aleditor_add_popup_no_symbol(
        editor: *mut AlEditor_ptr,
        title: *const i8,
        items: *const i8,
        default_value: i32,
        func: APIIntCallback,
    ) -> statusCode;
    fn aleditor_add_radio(
        editor: *mut AlEditor_ptr,
        title: *const i8,
        items: *const i8,
        default_value: i32,
        func: APIIntCallback,
        symbol_name: *const i8,
    ) -> statusCode;
    fn aleditor_add_radio_no_symbol(
        editor: *mut AlEditor_ptr,
        title: *const i8,
        items: *const i8,
        default_value: i32,
        func: APIIntCallback,
    ) -> statusCode;
    fn aleditor_add_int_slider(
        editor: *mut AlEditor_ptr,
        title: *const i8,
        default_value: i32,
        slider_min: i32,
        slider_max: i32,
        func: APIIntCallback,
        symbol_name: *const i8,
    ) -> statusCode;
    fn aleditor_add_int_slider_no_symbol(
        editor: *mut AlEditor_ptr,
        title: *const i8,
        default_value: i32,
        slider_min: i32,
        slider_max: i32,
        func: APIIntCallback,
    ) -> statusCode;
    fn aleditor_add_double_slider(
        editor: *mut AlEditor_ptr,
        title: *const i8,
        default_value: f64,
        slider_min: f64,
        slider_max: f64,
        func: APIDoubleCallback,
        symbol_name: *const i8,
    ) -> statusCode;
    fn aleditor_add_double_slider_no_symbol(
        editor: *mut AlEditor_ptr,
        title: *const i8,
        default_value: f64,
        slider_min: f64,
        slider_max: f64,
        func: APIDoubleCallback,
    ) -> statusCode;
    fn aleditor_add_double_slider_precision(
        editor: *mut AlEditor_ptr,
        title: *const i8,
        default_value: f64,
        slider_min: f64,
        slider_max: f64,
        func: APIDoubleCallback,
        precision: i32,
        symbol_name: *const i8,
    ) -> statusCode;
    fn aleditor_add_color(
        editor: *mut AlEditor_ptr,
        title: *const i8,
        default_value: APIColor,
        func: APIColorCallback,
        symbol_name: *const i8,
    ) -> statusCode;
    fn aleditor_add_color_no_symbol(
        editor: *mut AlEditor_ptr,
        title: *const i8,
        default_value: APIColor,
        func: APIColorCallback,
    ) -> statusCode;
    fn aleditor_add_file_browser(
        editor: *mut AlEditor_ptr,
        title: *const i8,
        dialog_title: *const i8,
        initial_dir: *const i8,
        func: APICharCallback,
        symbol_name: *const i8,
    ) -> statusCode;
    fn aleditor_add_folder_browser(
        editor: *mut AlEditor_ptr,
        title: *const i8,
        dialog_title: *const i8,
        initial_dir: *const i8,
        func: APICharCallback,
        symbol_name: *const i8,
    ) -> statusCode;
    fn aleditor_add_image_file(
        editor: *mut AlEditor_ptr,
        file_name: *const i8,
        symbol_name: *const i8,
    ) -> statusCode;
    fn aleditor_add_image_data(
        editor: *mut AlEditor_ptr,
        img_width: i32,
        img_height: i32,
        image: *const u8,
        symbol_name: *const i8,
    ) -> statusCode;
    fn aleditor_set_enabled(
        editor: *mut AlEditor_ptr,
        title: *const i8,
        enabled: bool,
    ) -> statusCode;
    fn aleditor_set_visible(
        editor: *mut AlEditor_ptr,
        title: *const i8,
        visible: bool,
    ) -> statusCode;
    fn aleditor_set_read_only_string(
        editor: *mut AlEditor_ptr,
        title: *const i8,
        new_value: *const i8,
    ) -> statusCode;
    fn aleditor_set_image_file(
        editor: *mut AlEditor_ptr,
        symbol_name: *const i8,
        file_name: *const i8,
    ) -> statusCode;
    fn aleditor_set_image_data(
        editor: *mut AlEditor_ptr,
        symbol_name: *const i8,
        img_width: i32,
        img_height: i32,
        image: *const u8,
    ) -> statusCode;
    fn aleditor_set_group_expanded(
        editor: *mut AlEditor_ptr,
        unique_title: *const i8,
        expanded: bool,
    ) -> statusCode;
    fn aleditor_set_title(
        editor: *mut AlEditor_ptr,
        symbol_name: *const i8,
        title: *const i8,
    ) -> statusCode;
    fn aleditor_set_popup_items(
        editor: *mut AlEditor_ptr,
        symbol_name: *const i8,
        items: *const i8,
        value: i32,
    ) -> statusCode;
    fn aleditor_set_int_slider_range(
        editor: *mut AlEditor_ptr,
        symbol_name: *const i8,
        slider_min: i32,
        slider_max: i32,
    ) -> statusCode;
    fn aleditor_set_double_slider_range(
        editor: *mut AlEditor_ptr,
        symbol_name: *const i8,
        slider_min: f64,
        slider_max: f64,
    ) -> statusCode;
}
#[allow(non_camel_case_types)]
pub struct AlEditor_X {
    pub aleditor: AlEditor,
    pub alhandle: AlFunctionHandle,
    pub alfunc: AlContinuousFunction,

    pub title: String,
    pub function_name: String,
    pub attr: String,
    pub menu_name: String,
    pub icon_path: String,
}

unsafe impl Send for AlEditor_X {}
unsafe impl Sync for AlEditor_X {}

impl AlEditor_X {
    pub fn Init(
        title: &str,
        function_name: &str,
        attr: &str,
        menu_name: &str,
        icon_path: &str,
        initFunc: Option<APIVoidCallback>,
        downFunc: Option<AlMouseButtonFunction>,
        moveFunc: Option<AlMouseButtonFunction>,
        upFunc: Option<AlMouseButtonFunction>,
        cleanupFunc: Option<APIVoidCallback>,
    ) -> Result<Self, String> {
        let mut alfunc = AlContinuousFunction::new();
        let mut alhandle = AlFunctionHandle::new();

        alfunc.create_with_command(
            function_name,
            initFunc,
            downFunc,
            moveFunc,
            upFunc,
            cleanupFunc,
            false,
        )?;
        alhandle.create_with_function(title, &alfunc)?;
        alhandle.set_attribute_string(attr)?;

        let aleditor = AlEditor::new(title, function_name)?;
        Ok(Self {
            aleditor,
            alhandle,
            alfunc,
            title: title.to_string(),
            function_name: function_name.to_string(),
            attr: attr.to_string(),
            menu_name: menu_name.to_string(),
            icon_path: icon_path.to_string(),
        })
    }

    pub fn End(&self) -> Result<(), String> {
        self.aleditor.create()?;
        self.alhandle
            .set_option_box(&self.title, &self.function_name)?;
        self.alhandle.append_to_menu(&self.menu_name)?;
        self.alhandle.set_custom_icon(self.icon_path.as_str())?;
        Ok(())
    }
}
impl Drop for AlEditor_X {
    fn drop(&mut self) {
        self.aleditor.close().ok();
        self.alhandle.remove_from_menu().ok();
        //drop(self.aleditor);
        self.alhandle.delete_object().ok();
        self.alfunc.delete_object().ok();
    }
}
