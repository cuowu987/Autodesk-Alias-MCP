#[repr(C)]
pub struct AlList_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlList {
    pub ptr: *mut AlList_ptr
}
