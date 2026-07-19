use crate::*;
#[repr(C)]
pub struct AlMainMenu_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlMainMenu {
    pub ptr: *mut AlMainMenu_ptr,
}

impl AlMainMenu {
    pub fn create() -> AlMainMenu {
        let ptr = unsafe { almainmenu_create() };
        AlMainMenu { ptr }
    }

    pub fn create_main_menu(&mut self, menu_name: &str) -> Result<(), String> {
        let c_name = std::ffi::CString::new(menu_name).unwrap();
        let status = unsafe { almainmenu_create_main_menu(self.ptr, c_name.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn remove(&self) -> Result<(), String> {
        let status = unsafe { almainmenu_remove(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }
}

impl Drop for AlMainMenu {
    fn drop(&mut self) {
        unsafe {
            almainmenu_destroy(self.ptr);
            self.ptr = std::ptr::null_mut();
        }
    }
}

unsafe extern "C" {

    fn almainmenu_create() -> *mut AlMainMenu_ptr;
    fn almainmenu_destroy(main_menu: *mut AlMainMenu_ptr);
    fn almainmenu_create_main_menu(
        main_menu: *mut AlMainMenu_ptr,
        menu_name: *const i8,
    ) -> statusCode;
    fn almainmenu_remove(main_menu: *mut AlMainMenu_ptr) -> statusCode;
}
