#[repr(C)]
pub struct AlCommandRef_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlCommandRef {
    pub ptr: *mut AlCommandRef_ptr
}
