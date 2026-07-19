#[repr(C)]
pub struct AlAmbientLight_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlAmbientLight {
    pub ptr: *mut AlAmbientLight_ptr
}
