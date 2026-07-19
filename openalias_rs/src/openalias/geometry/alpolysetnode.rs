#[repr(C)]
pub struct AlPolysetNode_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlPolysetNode {
    pub ptr: *mut AlPolysetNode_ptr
}
