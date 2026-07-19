#[repr(C)]
pub struct AlChannel_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlChannel {
    pub ptr: *mut AlChannel_ptr
}
