#[repr(C)]
pub struct AlCurveOnSurfacePoint_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlCurveOnSurfacePoint {
    pub ptr: *mut AlCurveOnSurfacePoint_ptr
}
