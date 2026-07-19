#[repr(C)]
pub struct AlSurfacePoint_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlSurfacePoint {
    pub ptr: *mut AlSurfacePoint_ptr
}
