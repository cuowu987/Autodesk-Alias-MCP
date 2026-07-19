use crate::*;
#[repr(C)]
pub struct AlMomentaryFunction_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlMomentaryFunction {
    pub ptr: *mut AlMomentaryFunction_ptr,
}

impl AlFunctionMethods for AlMomentaryFunction {
    fn as_AlFunction_ptr(&self) -> *mut AlFunction_ptr {
        self.ptr as *mut AlFunction_ptr
    }
}

impl AlMomentaryFunction {
    pub fn new() -> AlMomentaryFunction {
        let ptr = unsafe { almomentaryfunction_new() };
        AlMomentaryFunction { ptr }
    }

    pub fn create(action: Option<extern "C" fn()>) -> Result<AlMomentaryFunction, String> {
        let func = AlMomentaryFunction {
            ptr: std::ptr::null_mut(),
        };
        let status = unsafe { almomentaryfunction_create(func.ptr, action) };
        if status == statusCode::Success {
            Ok(func)
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_with_command(
        command: &str,
        action: Option<extern "C" fn()>,
    ) -> Result<AlMomentaryFunction, String> {
        let c_command = std::ffi::CString::new(command).unwrap();
        let func = AlMomentaryFunction {
            ptr: std::ptr::null_mut(),
        };
        let status = unsafe {
            almomentaryfunction_create_with_command(func.ptr, c_command.as_ptr(), action)
        };
        if status == statusCode::Success {
            Ok(func)
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_with_data(
        command: &str,
        action: Option<extern "C" fn()>,
        data: *mut *mut std::ffi::c_void,
    ) -> Result<AlMomentaryFunction, String> {
        let c_command = std::ffi::CString::new(command).unwrap();
        let func = AlMomentaryFunction {
            ptr: std::ptr::null_mut(),
        };
        let status = unsafe {
            almomentaryfunction_create_with_data(func.ptr, c_command.as_ptr(), action, data)
        };
        if status == statusCode::Success {
            Ok(func)
        } else {
            Err(status.to_string())
        }
    }
}

unsafe extern "C" {
    fn almomentaryfunction_new() -> *mut AlMomentaryFunction_ptr;
    fn almomentaryfunction_create(
        func: *mut AlMomentaryFunction_ptr,
        action: Option<extern "C" fn()>,
    ) -> statusCode;
    fn almomentaryfunction_create_with_command(
        func: *mut AlMomentaryFunction_ptr,
        command: *const i8,
        action: Option<extern "C" fn()>,
    ) -> statusCode;
    fn almomentaryfunction_create_with_data(
        func: *mut AlMomentaryFunction_ptr,
        command: *const i8,
        action: Option<extern "C" fn()>,
        data: *mut *mut std::ffi::c_void,
    ) -> statusCode;
}
