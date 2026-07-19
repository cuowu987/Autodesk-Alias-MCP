#[repr(C)]
pub struct AlIKHandle_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlIKHandle {
    pub ptr: *mut AlIKHandle_ptr
}
