#[repr(C)]
pub struct AlAttributes_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlAttributes {
    pub ptr: *mut AlAttributes_ptr
}
