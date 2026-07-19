#[repr(C)]
pub struct AlPolygon_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlPolygon {
    pub ptr: *mut AlPolygon_ptr
}
