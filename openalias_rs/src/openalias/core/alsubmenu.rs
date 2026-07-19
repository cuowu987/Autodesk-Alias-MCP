use crate::*;
#[repr(C)]
pub struct AlSubmenu_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlSubmenu {
    pub ptr: *mut AlSubmenu_ptr
}



impl AlSubmenu {
    pub fn create() -> AlSubmenu {
        let ptr = unsafe { alsubmenu_create() };
        AlSubmenu { ptr }
    }

    pub fn create_submenu(&mut self, parent_menu: &str, submenu_name: &str) -> Result<(), String> {
        let c_parent = std::ffi::CString::new(parent_menu).unwrap();
        let c_name = std::ffi::CString::new(submenu_name).unwrap();
        let status = unsafe { alsubmenu_create_submenu(self.ptr, c_parent.as_ptr(), c_name.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn remove_submenu(&self) -> Result<(), String> {
        let status = unsafe { alsubmenu_remove_submenu(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }
}

impl Drop for AlSubmenu {
    fn drop(&mut self) {
        unsafe {
            alsubmenu_destroy(self.ptr);
            self.ptr = std::ptr::null_mut();
        }
    }
}



unsafe extern "C" {
    fn alsubmenu_create() -> *mut AlSubmenu_ptr;
    fn alsubmenu_destroy(submenu: *mut AlSubmenu_ptr);
    fn alsubmenu_create_submenu(submenu: *mut AlSubmenu_ptr, parent_menu: *const i8, submenu_name: *const i8) -> statusCode;
    fn alsubmenu_remove_submenu(submenu: *mut AlSubmenu_ptr) -> statusCode;
}