#[repr(C)]
pub struct AlCanvas_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlCanvas {
    pub ptr: *mut AlCanvas_ptr
}
