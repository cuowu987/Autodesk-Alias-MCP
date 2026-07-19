#[repr(C)]
pub struct AlConstraint_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlConstraint {
    pub ptr: *mut AlConstraint_ptr
}
