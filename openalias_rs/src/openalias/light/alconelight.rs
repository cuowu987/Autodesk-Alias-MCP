#[repr(C)]
pub struct AlConeLight_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlConeLight {
    pub ptr: *mut AlConeLight_ptr
}
