#[repr(C)]
pub struct AlReferenceFile_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlReferenceFile {
    pub ptr: *mut AlReferenceFile_ptr
}
