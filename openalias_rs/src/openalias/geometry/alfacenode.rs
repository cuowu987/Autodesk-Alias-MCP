#[repr(C)]
pub struct AlFaceNode_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlFaceNode {
    pub ptr: *mut AlFaceNode_ptr
}
