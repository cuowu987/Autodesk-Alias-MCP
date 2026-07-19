#[repr(C)]
pub struct AlIKHandleNode_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlIKHandleNode {
    pub ptr: *mut AlIKHandleNode_ptr
}
