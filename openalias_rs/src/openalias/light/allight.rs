#[repr(C)]
pub struct AlLight_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlLight {
    pub ptr: *mut AlLight_ptr
}
