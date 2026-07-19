#[repr(C)]
pub struct AlAreaLight_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlAreaLight {
    pub ptr: *mut AlAreaLight_ptr
}
