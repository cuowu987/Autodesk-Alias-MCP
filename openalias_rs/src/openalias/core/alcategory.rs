#[repr(C)]
pub struct AlCategory_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlCategory {
    pub ptr: *mut AlCategory_ptr
}
