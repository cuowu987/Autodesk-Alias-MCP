#[repr(C)]
pub struct AlCylinderLight_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlCylinderLight {
    pub ptr: *mut AlCylinderLight_ptr
}
