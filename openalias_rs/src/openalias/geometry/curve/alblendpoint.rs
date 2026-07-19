#[repr(C)]
pub struct AlBlendPoint_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlBlendPoint {
    pub ptr: *mut AlBlendPoint_ptr
}
