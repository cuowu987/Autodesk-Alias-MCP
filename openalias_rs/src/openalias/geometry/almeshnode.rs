#[repr(C)]
pub struct AlMeshNode_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlMeshNode {
    pub ptr: *mut AlMeshNode_ptr
}
