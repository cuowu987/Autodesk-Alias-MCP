#[repr(C)]
pub struct AlAimConstraint_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlAimConstraint {
    pub ptr: *mut AlAimConstraint_ptr
}
