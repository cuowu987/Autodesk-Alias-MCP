#[repr(C)]
pub struct AlContact_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlContact {
    pub ptr: *mut AlContact_ptr
}
