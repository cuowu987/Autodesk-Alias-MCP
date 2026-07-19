use crate::{AlObjectMethods, statusCode};

#[repr(C)]
pub struct AlToolMetaData_ptr {
    _private: [u8; 0],
}


pub struct AlToolMetaData {
    pub ptr: *mut AlToolMetaData_ptr,
}

impl AlToolMetaData {
    pub fn new() -> AlToolMetaData {
        let ptr = unsafe { altoolmetadata_new() };
        AlToolMetaData { ptr }
    }

    pub fn reset(&self) -> Result<(), String> {
        let status = unsafe { altoolmetadata_reset(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn num_items(&self) -> i32 {
        unsafe { altoolmetadata_num_items(self.ptr) }
    }

    pub fn item_label(&self, index: i32) -> Option<String> {
        let c_str = unsafe { altoolmetadata_item_label(self.ptr, index) };
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

    pub fn item_value(&self, index: i32) -> Option<String> {
        let c_str = unsafe { altoolmetadata_item_value(self.ptr, index) };
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

    pub fn add(&self, label: &str, value: &str) -> Result<(), String> {
        let c_label = std::ffi::CString::new(label).unwrap();
        let c_value = std::ffi::CString::new(value).unwrap();
        let status = unsafe { altoolmetadata_add(self.ptr, c_label.as_ptr(), c_value.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn replace(&self, label: &str, new_value: &str) -> Result<(), String> {
        let c_label = std::ffi::CString::new(label).unwrap();
        let c_new_value = std::ffi::CString::new(new_value).unwrap();
        let status =
            unsafe { altoolmetadata_replace(self.ptr, c_label.as_ptr(), c_new_value.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }
}

impl Drop for AlToolMetaData {
    fn drop(&mut self) {
        unsafe {
            altoolmetadata_delete(self.ptr);
            self.ptr = std::ptr::null_mut();
        }
    }
}
impl AlObjectMethods for AlToolMetaData {
    fn as_object_ptr(&self) -> *mut crate::AlObject_ptr {
         self.ptr as *mut crate::AlObject_ptr 
    }
}
impl std::fmt::Debug for AlToolMetaData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num_items = self.num_items();
        let mut items = Vec::new();
        for i in 0..num_items {
            let label = self.item_label(i).unwrap();
            let value = self.item_value(i).unwrap();
            items.push((label, value));
        }
        write!(f, "AlToolMetaData {:?}", items)
    }
}

unsafe extern "C" {
    fn altoolmetadata_new() -> *mut AlToolMetaData_ptr;
    fn altoolmetadata_delete(tool_metadata: *mut AlToolMetaData_ptr);

    fn altoolmetadata_reset(tool_metadata: *mut AlToolMetaData_ptr) -> statusCode;

    fn altoolmetadata_num_items(tool_metadata: *mut AlToolMetaData_ptr) -> i32;

    fn altoolmetadata_item_label(tool_metadata: *mut AlToolMetaData_ptr, index: i32) -> *const i8;
    fn altoolmetadata_item_value(tool_metadata: *mut AlToolMetaData_ptr, index: i32) -> *const i8;

    fn altoolmetadata_add(
        tool_metadata: *mut AlToolMetaData_ptr,
        label: *const i8,
        value: *const i8,
    ) -> statusCode;
    fn altoolmetadata_replace(
        tool_metadata: *mut AlToolMetaData_ptr,
        label: *const i8,
        new_value: *const i8,
    ) -> statusCode;
}
