#[repr(C)]
pub struct AlTextureNode_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlTextureNode {
    pub ptr: *mut AlTextureNode_ptr
}
