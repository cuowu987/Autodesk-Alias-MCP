#[repr(C)]
pub struct AlRevSurfAttributes_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlRevSurfAttributes {
    pub ptr: *mut AlRevSurfAttributes_ptr
}
