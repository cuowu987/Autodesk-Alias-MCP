#[repr(C)]
pub struct AlPlaneAttributes_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlPlaneAttributes {
    pub ptr: *mut AlPlaneAttributes_ptr
}
