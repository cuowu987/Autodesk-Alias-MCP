#[repr(C)]
pub struct AlBoxLight_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlBoxLight {
    pub ptr: *mut AlBoxLight_ptr
}
