#[repr(C)]
pub struct AlEnvironment_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlEnvironment {
    pub ptr: *mut AlEnvironment_ptr
}
