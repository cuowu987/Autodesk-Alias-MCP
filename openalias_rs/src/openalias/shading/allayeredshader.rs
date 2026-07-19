#[repr(C)]
pub struct AlLayeredShader_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlLayeredShader {
    pub ptr: *mut AlLayeredShader_ptr
}
