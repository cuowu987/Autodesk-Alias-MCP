#[repr(C)]
pub struct AlClusterNode_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlClusterNode {
    pub ptr: *mut AlClusterNode_ptr
}
