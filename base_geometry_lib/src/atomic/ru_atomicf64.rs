//全局忽视命名警告
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use core::f64;
use std::sync::Mutex;
use std::sync::MutexGuard;




/// #模拟类似AtomicBool功能
#[derive(Debug)]
pub struct RU_AtomicF64 {
    pub value: Mutex<f64>,
}
impl RU_AtomicF64 {
    pub const fn new() -> Self {
        Self {
            value: Mutex::new(0.0),
        }
    }
    pub fn create(value:f64) -> Self {
        Self {
            value: Mutex::new(value),
        }
    }
    pub fn lock(&self,err:&str) -> Result<MutexGuard<'_, f64>, String> {
        self.value.lock().map_err(|_|err.to_string())
    }
   
    pub fn load(&self,err:&str) -> Result<f64, String> {
        Ok(*self.lock(err)?)
    }
    pub fn set(&self, value: f64,err:&str) -> Result<(), String> {
        *self.lock(err)? = value;
        Ok(())
    }


    pub fn clear(&self,err:&str) -> Result<(), String> {
        *self.lock(err)? = 0.0;
        Ok(())
    }
   
}













