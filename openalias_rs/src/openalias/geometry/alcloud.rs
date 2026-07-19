#[repr(C)]
pub struct AlCloud_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlCloud {
    pub ptr: *mut AlCloud_ptr
}
