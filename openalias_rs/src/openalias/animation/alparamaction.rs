#[repr(C)]
pub struct AlParamAction_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlParamAction {
    pub ptr: *mut AlParamAction_ptr
}
