#[repr(C)]
pub struct AlTrimCurve_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlTrimCurve {
    pub ptr: *mut AlTrimCurve_ptr
}
