#[repr(C)]
pub struct AlDistanceLocator_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlDistanceLocator {
    pub ptr: *mut AlDistanceLocator_ptr
}
