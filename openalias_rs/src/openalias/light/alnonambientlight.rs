#[repr(C)]
pub struct AlNonAmbientLight_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlNonAmbientLight {
    pub ptr: *mut AlNonAmbientLight_ptr
}
