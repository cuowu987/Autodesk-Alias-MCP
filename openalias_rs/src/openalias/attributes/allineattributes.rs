#[repr(C)]
pub struct AlLineAttributes_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlLineAttributes {
    pub ptr: *mut AlLineAttributes_ptr
}
