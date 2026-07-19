use crate::*;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlFileBrowseMode {
    FileBrowseRead = 0,
    FileBrowseWrite = 1,
    DirectoryBrowseRead = 2,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlConfirmType {
    OKCancel = 0,
    YesNoCancel = 1,
    OKOnly = 2,
    YesNo = 3,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlAnswerType {
    OK = 0,
    Yes = 1,
    No = 2,
    Cancel = 3,
}

pub fn printf_1(output_type: AlOutputType, format: &str) {
    let c_format = std::ffi::CString::new(format).unwrap_or_default();
    unsafe {
        allivedata_printf(output_type as i32, c_format.as_ptr());
    }
}


pub fn prompt_box(
    confirm_type: AlConfirmType,
    message: &str,
    x: i16,
    y: i16,
) -> Result<AlAnswerType, String> {
    let c_message = std::ffi::CString::new(message).map_err(|_| "Invalid message".to_string())?;
    let mut out_answer: i32 = 0;
    let status = unsafe {
        allivedata_prompt_box(
            confirm_type as i32,
            c_message.as_ptr(),
            &mut out_answer,
            x,
            y,
        )
    };
    if status == statusCode::Success {
        Ok(unsafe { std::mem::transmute(out_answer) })
    } else {
        Err(status.to_string())
    }
}

/// 带 format! 的便捷宏
#[macro_export]
macro_rules! printf {
    ($output_type:expr, $($arg:tt)*) => {
        $crate::printf_1($output_type, &format!($($arg)*))
    };
}
#[macro_export]
macro_rules! prmptBox {
    ($($arg:tt)*) => {
        prompt_box(AlConfirmType::OKCancel, &format!($($arg)*),500,500)
    };
}



pub fn get_alias_preference(preference_name: &str) -> Option<String> {
    let c_name = std::ffi::CString::new(preference_name).ok()?;
    let result = unsafe { allivedata_get_alias_preference(c_name.as_ptr()) };
    if result.is_null() {
        return None;
    }
    unsafe {
        std::ffi::CStr::from_ptr(result)
            .to_str()
            .ok()
            .map(|s| s.to_string())
    }
}

pub fn debug_reset_option_box(editor_name: &str) -> Result<(), String> {
    let c_name =
        std::ffi::CString::new(editor_name).map_err(|_| "Invalid editor name".to_string())?;
    let status = unsafe { allivedata_debug_reset_option_box(c_name.as_ptr()) };
    if status == statusCode::Success {
        Ok(())
    } else {
        Err(status.to_string())
    }
}

pub fn file_browser(
    mode: AlFileBrowseMode,
    title: &str,
    multi_select: bool,
    filter: &str,
) -> Result<String, String> {
    let c_title = std::ffi::CString::new(title).map_err(|_| "Invalid title".to_string())?;
    let c_filter = std::ffi::CString::new(filter).map_err(|_| "Invalid filter".to_string())?;
    let mut out_file_name: *mut i8 = std::ptr::null_mut();
    let status = unsafe {
        allivedata_file_browser(
            mode as i32,
            &mut out_file_name,
            c_title.as_ptr(),
            multi_select,
            c_filter.as_ptr(),
        )
    };
    if status == statusCode::Success && !out_file_name.is_null() {
        let result = unsafe {
            std::ffi::CStr::from_ptr(out_file_name)
                .to_string_lossy()
                .to_string()
        };
        Ok(result)
    } else {
        Err(status.to_string())
    }
}

pub fn file_browser_ex(
    mode: AlFileBrowseMode,
    title: &str,
    multi_select: bool,
    filter: &str,
    default_file_name: &str,
) -> Result<String, String> {
    let c_title = std::ffi::CString::new(title).map_err(|_| "Invalid title".to_string())?;
    let c_filter = std::ffi::CString::new(filter).map_err(|_| "Invalid filter".to_string())?;
    let c_default = std::ffi::CString::new(default_file_name)
        .map_err(|_| "Invalid default file name".to_string())?;
    let mut out_file_name: *mut i8 = std::ptr::null_mut();
    let status = unsafe {
        allivedata_file_browser_ex(
            mode as i32,
            &mut out_file_name,
            c_title.as_ptr(),
            multi_select,
            c_filter.as_ptr(),
            c_default.as_ptr(),
        )
    };
    if status == statusCode::Success && !out_file_name.is_null() {
        let result = unsafe {
            std::ffi::CStr::from_ptr(out_file_name)
                .to_string_lossy()
                .to_string()
        };
        Ok(result)
    } else {
        Err(status.to_string())
    }
}

pub fn escape_key_pressed() -> bool {
    unsafe { allivedata_escape_key_pressed() }
}


pub fn get_integer(key: &str) -> Result<i32, String> {
    let c_key = std::ffi::CString::new(key).map_err(|_| "Invalid key".to_string())?;
    let mut out_value: i32 = 0;
    let status = unsafe { allivedata_get_integer(c_key.as_ptr(), &mut out_value) };
    if status == statusCode::Success {
        Ok(out_value)
    } else {
        Err(status.to_string())
    }
}

pub fn get_double(key: &str) -> Result<f64, String> {
    let c_key = std::ffi::CString::new(key).map_err(|_| "Invalid key".to_string())?;
    let mut out_value: f64 = 0.0;
    let status = unsafe { allivedata_get_double(c_key.as_ptr(), &mut out_value) };
    if status == statusCode::Success {
        Ok(out_value)
    } else {
        Err(status.to_string())
    }
}

pub fn get_string(key: &str) -> Result<String, String> {
    let c_key = std::ffi::CString::new(key).map_err(|_| "Invalid key".to_string())?;
    let mut out_value: *const i8 = std::ptr::null();
    let status = unsafe { allivedata_get_string(c_key.as_ptr(), &mut out_value) };
    if status == statusCode::Success && !out_value.is_null() {
        Ok(unsafe {
            std::ffi::CStr::from_ptr(out_value)
                .to_string_lossy()
                .to_string()
        })
    } else {
        Err(status.to_string())
    }
}

pub fn set_integer(key: &str, value: i32) -> Result<(), String> {
    let c_key = std::ffi::CString::new(key).map_err(|_| "Invalid key".to_string())?;
    let status = unsafe { allivedata_set_integer(c_key.as_ptr(), value) };
    if status == statusCode::Success {
        Ok(())
    } else {
        Err(status.to_string())
    }
}

pub fn set_double(key: &str, value: f64) -> Result<(), String> {
    let c_key = std::ffi::CString::new(key).map_err(|_| "Invalid key".to_string())?;
    let status = unsafe { allivedata_set_double(c_key.as_ptr(), value) };
    if status == statusCode::Success {
        Ok(())
    } else {
        Err(status.to_string())
    }
}

pub fn set_string(key: &str, value: &str) -> Result<(), String> {
    let c_key = std::ffi::CString::new(key).map_err(|_| "Invalid key".to_string())?;
    let c_value = std::ffi::CString::new(value).map_err(|_| "Invalid value".to_string())?;
    let status = unsafe { allivedata_set_string(c_key.as_ptr(), c_value.as_ptr()) };
    if status == statusCode::Success {
        Ok(())
    } else {
        Err(status.to_string())
    }
}

pub fn plugin_dir_path_name() -> Option<AlList> {
    let ptr = unsafe { allivedata_plugin_dir_path_name() };
    if ptr.is_null() {
        None
    } else {
        Some(AlList { ptr })
    }
}

pub fn plugin_conductor_dir_path_name() -> Option<AlList> {
    let ptr = unsafe { allivedata_plugin_conductor_dir_path_name() };
    if ptr.is_null() {
        None
    } else {
        Some(AlList { ptr })
    }
}

pub fn app_exchange_dir_path_name() -> Option<AlList> {
    let ptr = unsafe { allivedata_app_exchange_dir_path_name() };
    if ptr.is_null() {
        None
    } else {
        Some(AlList { ptr })
    }
}

pub fn form_explorer_dir_path_name() -> Option<AlList> {
    let ptr = unsafe { allivedata_form_explorer_dir_path_name() };
    if ptr.is_null() {
        None
    } else {
        Some(AlList { ptr })
    }
}

pub fn nav_design_dir_path_name() -> Option<AlList> {
    let ptr = unsafe { allivedata_nav_design_dir_path_name() };
    if ptr.is_null() {
        None
    } else {
        Some(AlList { ptr })
    }
}

pub fn init_pluginman() -> i32 {
    unsafe { allivedata_init_pluginman() }
}

pub fn store_plugin_auto_load_file() {
    unsafe {
        allivedata_store_plugin_auto_load_file();
    }
}

pub fn vprintf(output_type: AlOutputType, format: &str, ap: *mut std::ffi::c_void) {
    let c_format = std::ffi::CString::new(format).unwrap_or_default();
    unsafe {
        allivedata_vprintf(output_type as i32, c_format.as_ptr(), ap);
    }
}

pub fn confirm_box(
    chars_per_line: i32,
    x: i32,
    y: i32,
    dont_ask_symbol_name: &str,
    notice: &str,
) -> Result<AlAnswerType, String> {
    let c_dont_ask = std::ffi::CString::new(dont_ask_symbol_name)
        .map_err(|_| "Invalid dont_ask_symbol_name".to_string())?;
    let c_notice = std::ffi::CString::new(notice).map_err(|_| "Invalid notice".to_string())?;
    let mut out_answer: i32 = 0;
    let status = unsafe {
        allivedata_confirm_box(
            chars_per_line,
            x,
            y,
            &mut out_answer,
            c_dont_ask.as_ptr(),
            c_notice.as_ptr(),
        )
    };
    if status == statusCode::Success {
        Ok(unsafe { std::mem::transmute(out_answer) })
    } else {
        Err(status.to_string())
    }
}

pub fn scrollable_prompt_box(
    confirm_type: AlConfirmType,
    message: &str,
    input_buffer: &mut [i8],
) -> Result<AlAnswerType, String> {
    let c_message = std::ffi::CString::new(message).map_err(|_| "Invalid message".to_string())?;
    let mut out_answer: i32 = 0;
    let status = unsafe {
        allivedata_scrollable_prompt_box(
            confirm_type as i32,
            c_message.as_ptr(),
            input_buffer.as_mut_ptr(),
            &mut out_answer,
        )
    };
    if status == statusCode::Success {
        Ok(unsafe { std::mem::transmute(out_answer) })
    } else {
        Err(status.to_string())
    }
}

pub fn string_prompt_box(
    confirm_type: AlConfirmType,
    message: &str,
) -> Result<(AlAnswerType, String), String> {
    let c_message = std::ffi::CString::new(message).map_err(|_| "Invalid message".to_string())?;
    let mut out_answer: i32 = 0;
    let mut out_string: *mut i8 = std::ptr::null_mut();
    let status = unsafe {
        allivedata_string_prompt_box(
            confirm_type as i32,
            c_message.as_ptr(),
            &mut out_answer,
            &mut out_string,
        )
    };
    if status == statusCode::Success {
        let string_result = if out_string.is_null() {
            "".to_string()
        } else {
            unsafe {
                std::ffi::CStr::from_ptr(out_string)
                    .to_string_lossy()
                    .to_string()
            }
        };
        Ok((unsafe { std::mem::transmute(out_answer) }, string_result))
    } else {
        Err(status.to_string())
    }
}


unsafe extern "C" {
    fn allivedata_printf(output_type: i32, format: *const i8);
    fn allivedata_vprintf(output_type: i32, format: *const i8, ap: *mut std::ffi::c_void);
    fn allivedata_get_alias_preference(preference_name: *const i8) -> *const i8;

    fn allivedata_debug_reset_option_box(editor_name: *const i8) -> statusCode;
    fn allivedata_file_browser(
        mode: i32,
        out_file_name: *mut *mut i8,
        title: *const i8,
        multi_select: bool,
        filter: *const i8,
    ) -> statusCode;
    fn allivedata_file_browser_ex(
        mode: i32,
        out_file_name: *mut *mut i8,
        title: *const i8,
        multi_select: bool,
        filter: *const i8,
        default_file_name: *const i8,
    ) -> statusCode;

    fn allivedata_escape_key_pressed() -> bool;
    fn allivedata_prompt_box(
        confirm_type: i32,
        message: *const i8,
        out_answer: *mut i32,
        x: i16,
        y: i16,
    ) -> statusCode;
    fn allivedata_confirm_box(
        chars_per_line: i32,
        x: i32,
        y: i32,
        out_answer: *mut i32,
        dont_ask_symbol_name: *const i8,
        notice: *const i8,
        ...
    ) -> statusCode;
    fn allivedata_scrollable_prompt_box(
        confirm_type: i32,
        message: *const i8,
        input_buffer: *mut i8,
        out_answer: *mut i32,
    ) -> statusCode;
    fn allivedata_string_prompt_box(
        confirm_type: i32,
        message: *const i8,
        out_answer: *mut i32,
        out_string: *mut *mut i8,
    ) -> statusCode;

    fn allivedata_get_integer(key: *const i8, out_value: *mut i32) -> statusCode;
    fn allivedata_get_double(key: *const i8, out_value: *mut f64) -> statusCode;
    fn allivedata_get_string(key: *const i8, out_value: *mut *const i8) -> statusCode;
    fn allivedata_set_integer(key: *const i8, value: i32) -> statusCode;
    fn allivedata_set_double(key: *const i8, value: f64) -> statusCode;
    fn allivedata_set_string(key: *const i8, value: *const i8) -> statusCode;

    fn allivedata_plugin_dir_path_name() -> *mut AlList_ptr;
    fn allivedata_plugin_conductor_dir_path_name() -> *mut AlList_ptr;
    fn allivedata_app_exchange_dir_path_name() -> *mut AlList_ptr;
    fn allivedata_form_explorer_dir_path_name() -> *mut AlList_ptr;
    fn allivedata_nav_design_dir_path_name() -> *mut AlList_ptr;

    fn allivedata_init_pluginman() -> i32;
    fn allivedata_store_plugin_auto_load_file();
}
