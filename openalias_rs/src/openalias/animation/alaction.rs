#[repr(C)]
pub struct AlAction_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlAction {
    pub ptr: *mut AlAction_ptr
}
