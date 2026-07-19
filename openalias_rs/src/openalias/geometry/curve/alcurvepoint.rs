#[repr(C)]
pub struct AlCurvePoint_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlCurvePoint {
    pub ptr: *mut AlCurvePoint_ptr
}
