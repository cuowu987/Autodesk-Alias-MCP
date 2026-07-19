#[repr(C)]
pub struct AlFace_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlFace {
    pub ptr: *mut AlFace_ptr
}
