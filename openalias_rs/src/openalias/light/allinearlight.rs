#[repr(C)]
pub struct AlLinearLight_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlLinearLight {
    pub ptr: *mut AlLinearLight_ptr
}
