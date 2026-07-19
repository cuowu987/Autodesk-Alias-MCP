#[repr(C)]
pub struct AlJoint_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlJoint {
    pub ptr: *mut AlJoint_ptr
}
