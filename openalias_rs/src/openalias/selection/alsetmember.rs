#[repr(C)]
pub struct AlSetMember_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlSetMember {
    pub ptr: *mut AlSetMember_ptr
}
