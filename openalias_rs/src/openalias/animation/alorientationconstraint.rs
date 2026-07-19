#[repr(C)]
pub struct AlOrientationConstraint_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlOrientationConstraint {
    pub ptr: *mut AlOrientationConstraint_ptr
}
