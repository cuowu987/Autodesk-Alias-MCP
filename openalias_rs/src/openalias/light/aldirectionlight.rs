#[repr(C)]
pub struct AlDirectionLight_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlDirectionLight {
    pub ptr: *mut AlDirectionLight_ptr
}
