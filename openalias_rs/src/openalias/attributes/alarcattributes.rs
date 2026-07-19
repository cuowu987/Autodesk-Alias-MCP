#[repr(C)]
pub struct AlArcAttributes_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlArcAttributes {
    pub ptr: *mut AlArcAttributes_ptr
}
