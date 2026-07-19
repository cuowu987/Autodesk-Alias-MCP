#[repr(C)]
pub struct AlAnimatable_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlAnimatable {
    pub ptr: *mut AlAnimatable_ptr
}
