#[repr(C)]
pub struct AlSettable_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlSettable {
    pub ptr: *mut AlSettable_ptr
}
