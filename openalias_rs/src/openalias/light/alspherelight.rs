#[repr(C)]
pub struct AlSphereLight_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlSphereLight {
    pub ptr: *mut AlSphereLight_ptr
}
