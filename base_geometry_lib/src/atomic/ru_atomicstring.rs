//全局忽视命名警告
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::os::raw::c_char;
use std::sync::Mutex;
use std::sync::MutexGuard;


/// #模拟类似AtomicBool功能
#[derive(Debug)]
pub struct RU_AtomicString {
    pub value: Mutex<String>,
}
impl RU_AtomicString {
    pub const fn new() -> Self {
        Self {
            value: Mutex::new(String::new()),
        }
    }
    pub fn create(name: String) -> Self {
        Self {
            value: Mutex::new(name),
        }
    }
    pub fn lock(&self, err: &str) -> Result<MutexGuard<'_, String>, String> {
        self.value.lock().map_err(|_| err.to_string())
    }


    pub fn load(&self, err: &str) -> Result<String, String> {
        Ok(self.lock(err)?.clone())
    }
    pub fn set(&self, value: String, err: &str) -> Result<(), String> {
        *self.lock(err)? = value;
        Ok(())
    }
    pub fn is_empty(&self, err: &str) -> Result<bool, String> {
        Ok(self.lock(err)?.is_empty())
    }
    pub fn as_ptr(&self, err: &str) -> *const c_char {
        self.lock(err).unwrap().as_ptr() as *const c_char
    }
    pub fn clear(&self, err: &str) -> Result<(), String> {
        *self.lock(err)? = String::new();
        Ok(())
    }
}





