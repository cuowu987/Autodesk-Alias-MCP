#[repr(C)]
pub struct AlPointLight_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlPointLight {
    pub ptr: *mut AlPointLight_ptr
}
