#[repr(C)]
pub struct AlCurveAttributes_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlCurveAttributes {
    pub ptr: *mut AlCurveAttributes_ptr
}
