#[repr(C)]
pub struct AlCommand_ptr {
    _private: [u8; 0], // 不透明
}

#[derive(Debug)]
pub struct AlCommand {
    pub ptr: *mut AlCommand_ptr
}
