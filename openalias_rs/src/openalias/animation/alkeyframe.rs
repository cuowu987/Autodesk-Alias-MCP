#[repr(C)]
pub struct AlKeyframe_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlKeyframe {
    pub ptr: *mut AlKeyframe_ptr
}
