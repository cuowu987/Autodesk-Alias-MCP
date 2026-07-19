#[repr(C)]
pub struct AlSubdivNode_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlSubdivNode {
    pub ptr: *mut AlSubdivNode_ptr
}
