#[repr(C)]
pub struct AlShellNode_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlShellNode {
    pub ptr: *mut AlShellNode_ptr
}
