#[repr(C)]
pub struct AlTrimBoundary_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlTrimBoundary {
    pub ptr: *mut AlTrimBoundary_ptr
}
