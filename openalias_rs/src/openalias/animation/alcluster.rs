#[repr(C)]
pub struct AlCluster_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlCluster {
    pub ptr: *mut AlCluster_ptr
}
