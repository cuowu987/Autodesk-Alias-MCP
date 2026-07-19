#![allow(non_camel_case_types)]
use crate::*;

pub type AlMouseButtonFunction = extern "C" fn(input: i32, x: i32, y: i32);
pub type AlUndoCallbackType = extern "C" fn();

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlInputType {
    kInputInvalid = 0,
    kInputAbort = 1,
    kInputKeyboard = 2,
    kInputButton = 3,
    kInputOther = 4,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlCoordinateType {
    kCoordinateInvalid = 0,
    kCoordinateAbsolute = 1,
    kCoordinateRelative = 2,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlBehaviourType {
    kBehaviourInvalid = 0,
    kBehaviourContinuous = 1,
    kBehaviourMomentary = 2,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlFilterType {
    kFilterNone = 0,
    kFilterLinear = 1,
    kFilterAngular = 2,
}

#[repr(C)]
pub struct AlFunction_ptr {
    _private: [u8; 0],
}

#[derive(Debug)]
pub struct AlFunction {
    pub ptr: *mut AlFunction_ptr,
}

pub trait AlFunctionMethods {
    fn as_AlFunction_ptr(&self) -> *mut AlFunction_ptr;
    fn delete_object(&self) -> Result<(), String> {
        let ptr = self.as_AlFunction_ptr();
        let status = unsafe { alfunction_delete_object(ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn name(&self) -> Option<String> {
        let ptr = self.as_AlFunction_ptr();
        let status = unsafe { alfunction_name(ptr) };
        if status.is_null() {
            None
        } else {
            Some(unsafe {
                std::ffi::CStr::from_ptr(status)
                    .to_string_lossy()
                    .to_string()
            })
        }
    }

    fn as_momentary_function(self) -> Option<AlMomentaryFunction>
    where
        Self: Sized,
    {
        let ptr = self.as_AlFunction_ptr();
        let ptr_1 = unsafe { alfunction_as_momentary_function_ptr(ptr) };
        if ptr_1.is_null() {
            None
        } else {
            std::mem::forget(self);
            Some(AlMomentaryFunction { ptr: ptr_1 })
        }
    }

    fn as_continuous_function(self) -> Option<AlContinuousFunction>
    where
        Self: Sized,
    {
        let ptr = self.as_AlFunction_ptr();
        let ptr_1 = unsafe { alfunction_as_continuous_function_ptr(ptr) };
        if ptr_1.is_null() {
            None
        } else {
            std::mem::forget(self);
            Some(AlContinuousFunction { ptr: ptr_1 })
        }
    }
}

impl AlFunctionMethods for AlFunction {
    fn as_AlFunction_ptr(&self) -> *mut AlFunction_ptr {
        self.ptr
    }
}

unsafe extern "C" {
    fn alfunction_delete_object(func: *mut AlFunction_ptr) -> statusCode;
    fn alfunction_name(func: *mut AlFunction_ptr) -> *const i8;
    fn alfunction_as_momentary_function_ptr(
        func: *mut AlFunction_ptr,
    ) -> *mut AlMomentaryFunction_ptr;
    fn alfunction_as_continuous_function_ptr(
        func: *mut AlFunction_ptr,
    ) -> *mut AlContinuousFunction_ptr;
}
