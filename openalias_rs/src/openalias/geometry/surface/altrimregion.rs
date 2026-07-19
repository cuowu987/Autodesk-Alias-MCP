#[repr(C)]
pub struct AlTrimRegion_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlTrimRegion {
    pub ptr: *mut AlTrimRegion_ptr
}
