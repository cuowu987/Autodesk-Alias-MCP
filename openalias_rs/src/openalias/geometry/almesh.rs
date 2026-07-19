#[repr(C)]
pub struct AlMesh_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlMesh {
    pub ptr: *mut AlMesh_ptr
}
