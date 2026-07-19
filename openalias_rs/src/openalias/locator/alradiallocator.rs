#[repr(C)]
pub struct AlRadialLocator_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlRadialLocator {
    pub ptr: *mut AlRadialLocator_ptr
}
