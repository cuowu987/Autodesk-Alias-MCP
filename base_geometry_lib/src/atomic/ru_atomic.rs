//全局忽视命名警告
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::sync::Mutex;
use std::sync::MutexGuard;


/// # 用于保存ai通过python生成的几何(预览几何,后缀名_preview)
/// 方便快速转换预览几何和最终几何
pub struct RU_Atomic<T> {
    pub values: Mutex<Option<T>>,
}


impl<T> RU_Atomic<T> {
    pub const fn new() -> Self {
        Self {
            values: Mutex::new(None),
        }
    }
    pub fn lock(&self, err: &str) -> Result<MutexGuard<'_, Option<T>>, String> {
        self.values.lock().map_err(|_| err.to_string())
    }
    pub fn clear(&self, err: &str) -> Result<(), String> {
        let _s = self.lock(err)?.take();
        //*self.lock(err)? = None;
        Ok(())
    }
}





