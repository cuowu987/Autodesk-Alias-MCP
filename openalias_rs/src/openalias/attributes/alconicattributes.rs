#[repr(C)]
pub struct AlConicAttributes_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlConicAttributes {
    pub ptr: *mut AlConicAttributes_ptr
}
