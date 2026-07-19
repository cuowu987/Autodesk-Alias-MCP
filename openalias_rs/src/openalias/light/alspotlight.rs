#[repr(C)]
pub struct AlSpotLight_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlSpotLight {
    pub ptr: *mut AlSpotLight_ptr
}
