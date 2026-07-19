#[repr(C)]
pub struct AlBlendCurve_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlBlendCurve {
    pub ptr: *mut AlBlendCurve_ptr
}
