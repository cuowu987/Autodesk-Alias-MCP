#[repr(C)]
pub struct AlSwitchShader_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlSwitchShader {
    pub ptr: *mut AlSwitchShader_ptr
}
