#[repr(C)]
pub struct AlPolysetVertex_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlPolysetVertex {
    pub ptr: *mut AlPolysetVertex_ptr
}
