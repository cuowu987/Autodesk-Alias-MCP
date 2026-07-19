#[repr(C)]
pub struct AlPointConstraint_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlPointConstraint {
    pub ptr: *mut AlPointConstraint_ptr
}
