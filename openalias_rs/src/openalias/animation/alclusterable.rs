#[repr(C)]
pub struct AlClusterable_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlClusterable {
    pub ptr: *mut AlClusterable_ptr
}
