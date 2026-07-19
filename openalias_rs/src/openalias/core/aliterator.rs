#[repr(C)]
pub struct AlIterator_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlIterator {
    pub ptr: *mut AlIterator_ptr
}
