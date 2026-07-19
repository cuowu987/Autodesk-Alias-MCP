#[repr(C)]
pub struct AlAngleLocator_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlAngleLocator {
    pub ptr: *mut AlAngleLocator_ptr
}
