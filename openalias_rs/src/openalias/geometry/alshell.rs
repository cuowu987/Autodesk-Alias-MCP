#[repr(C)]
pub struct AlShell_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlShell {
    pub ptr: *mut AlShell_ptr
}
