#[repr(C)]
pub struct AlClusterMember_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlClusterMember {
    pub ptr: *mut AlClusterMember_ptr
}
