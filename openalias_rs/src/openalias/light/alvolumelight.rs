#[repr(C)]
pub struct AlVolumeLight_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlVolumeLight {
    pub ptr: *mut AlVolumeLight_ptr
}
