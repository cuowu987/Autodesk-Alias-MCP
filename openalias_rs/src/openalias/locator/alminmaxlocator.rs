#[repr(C)]
pub struct AlMinmaxLocator_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlMinmaxLocator {
    pub ptr: *mut AlMinmaxLocator_ptr
}
