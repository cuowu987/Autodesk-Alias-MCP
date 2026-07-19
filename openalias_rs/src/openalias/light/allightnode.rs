#[repr(C)]
pub struct AlLightNode_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlLightNode {
    pub ptr: *mut AlLightNode_ptr
}
