#[repr(C)]
pub struct AlReferenceFileSet_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlReferenceFileSet {
    pub ptr: *mut AlReferenceFileSet_ptr
}
