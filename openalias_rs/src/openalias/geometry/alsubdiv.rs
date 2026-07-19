#[repr(C)]
pub struct AlSubdiv_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlSubdiv {
    pub ptr: *mut AlSubdiv_ptr
}
