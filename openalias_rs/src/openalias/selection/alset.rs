#[repr(C)]
pub struct AlSet_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlSet {
    pub ptr: *mut AlSet_ptr
}
