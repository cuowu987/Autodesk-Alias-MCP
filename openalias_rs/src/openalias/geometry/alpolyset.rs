#[repr(C)]
pub struct AlPolyset_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlPolyset {
    pub ptr: *mut AlPolyset_ptr
}
