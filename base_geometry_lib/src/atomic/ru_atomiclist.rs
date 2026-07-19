//全局忽视命名警告
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::sync::Mutex;
use std::sync::MutexGuard;


/// # 用于保存ai通过python生成的几何(预览几何,后缀名_preview)
/// 方便快速转换预览几何和最终几何
pub struct RU_AtomicList<T> {
    pub values: Mutex<Vec<T>>,
}


impl<T> RU_AtomicList<T> {
    pub const fn new() -> Self {
        Self {
            values: Mutex::new(Vec::new()),
        }
    }
    pub fn lock(&self, err: &str) -> Result<MutexGuard<'_, Vec<T>>, String> {
        self.values.lock().map_err(|_| err.to_string())
    }
    pub fn clear(&self, err: &str) -> Result<(), String> {
        self.lock(err)?.clear();
        Ok(())
    }
    pub fn map<F, C>(&self, err: &str, mut f: F) -> Result<Vec<C>, String>
    where
        F: FnMut(&mut T) -> Result<C, String>,
    {
        let mut s = self.lock(err)?;
        s.iter_mut().map(|v| f(v)).collect()
    }


    pub fn clear_map<F>(&self, err: &str, mut f: F) -> Result<(), String>
    where
        F: FnMut(T),
    {
        let mut s = self.lock(err)?;
        for v in s.drain(..) {
            f(v);
        }
        Ok(())
    }
}





