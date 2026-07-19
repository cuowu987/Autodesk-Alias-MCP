#[repr(C)]
pub struct AlTorusLight_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlTorusLight {
    pub ptr: *mut AlTorusLight_ptr
}
