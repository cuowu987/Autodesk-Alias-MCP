#[repr(C)]
pub struct AlMotionAction_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlMotionAction {
    pub ptr: *mut AlMotionAction_ptr
}
