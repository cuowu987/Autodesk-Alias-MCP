#[repr(C)]
pub struct AlTexture_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlTexture {
    pub ptr: *mut AlTexture_ptr
}
