#[repr(C)]
pub struct AlRenderInfo_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
#[repr(C)]
pub struct AlRenderInfo {
    pub ptr: *mut AlRenderInfo_ptr
}
