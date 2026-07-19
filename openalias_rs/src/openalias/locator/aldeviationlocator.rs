#[repr(C)]
pub struct AlDeviationLocator_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlDeviationLocator {
    pub ptr: *mut AlDeviationLocator_ptr
}
