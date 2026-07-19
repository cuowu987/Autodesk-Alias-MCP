#![allow(non_camel_case_types)]

use crate::*;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolShadeMode {
    kShadeAll = 0,
    kActiveOnly = 1,
}

#[repr(C)]
pub struct AlToolShader_ptr {
    _private: [u8; 0],
}

pub struct AlToolShader {
    ptr: *mut AlToolShader_ptr,
}

unsafe impl Send for AlToolShader {}
unsafe impl Sync for AlToolShader {}

impl AlToolShader {
    pub fn create() -> Self {
        let ptr = unsafe { altoolshader_create() };
        Self { ptr }
    }

    pub fn destroy(&mut self) {
        if !self.ptr.is_null() {
            unsafe { altoolshader_destroy(self.ptr) };
            self.ptr = std::ptr::null_mut();
        }
    }

    pub fn enable_shader(&mut self, mode: ToolShadeMode) -> Result<(), String> {
        let status = unsafe { altoolshader_enable_shader(self.ptr, mode) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn disable_shader(&mut self) -> Result<(), String> {
        let status = unsafe { altoolshader_disable_shader(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn enable_front_color(&mut self, color: [f32; 4]) -> Result<(), String> {
        let status = unsafe { altoolshader_enable_front_color(self.ptr, color.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn disable_front_color(&mut self) -> Result<(), String> {
        let status = unsafe { altoolshader_disable_front_color(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn enable_back_color(&mut self, color: [f32; 4]) -> Result<(), String> {
        let status = unsafe { altoolshader_enable_back_color(self.ptr, color.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn disable_back_color(&mut self) -> Result<(), String> {
        let status = unsafe { altoolshader_disable_back_color(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn enable_texture_from_file(&mut self, filepath: &str) -> Result<(), String> {
        let c_path = std::ffi::CString::new(filepath).map_err(|_| "Invalid filepath".to_string())?;
        let status = unsafe { altoolshader_enable_texture_from_file(self.ptr, c_path.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn enable_texture_from_memory(&mut self, image: &[u8], width: i32, height: i32) -> Result<(), String> {
        let status = unsafe { altoolshader_enable_texture_from_memory(self.ptr, image.as_ptr(), width, height) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn disable_texture(&mut self) -> Result<(), String> {
        let status = unsafe { altoolshader_disable_texture(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }
}

impl Drop for AlToolShader {
    fn drop(&mut self) {
        self.destroy();
        self.ptr = std::ptr::null_mut();
    }
}

unsafe extern "C" {
    fn altoolshader_create() -> *mut AlToolShader_ptr;
    fn altoolshader_destroy(shader: *mut AlToolShader_ptr);
    
    fn altoolshader_enable_shader(shader: *mut AlToolShader_ptr, mode: ToolShadeMode) -> statusCode;
    fn altoolshader_disable_shader(shader: *mut AlToolShader_ptr) -> statusCode;
    
    fn altoolshader_enable_front_color(shader: *mut AlToolShader_ptr, color: *const f32) -> statusCode;
    fn altoolshader_disable_front_color(shader: *mut AlToolShader_ptr) -> statusCode;
    
    fn altoolshader_enable_back_color(shader: *mut AlToolShader_ptr, color: *const f32) -> statusCode;
    fn altoolshader_disable_back_color(shader: *mut AlToolShader_ptr) -> statusCode;
    
    fn altoolshader_enable_texture_from_file(shader: *mut AlToolShader_ptr, filepath: *const i8) -> statusCode;
    fn altoolshader_enable_texture_from_memory(shader: *mut AlToolShader_ptr, image: *const u8, width: i32, height: i32) -> statusCode;
    fn altoolshader_disable_texture(shader: *mut AlToolShader_ptr) -> statusCode;
}
