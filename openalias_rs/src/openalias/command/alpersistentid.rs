//use alias_libs::RU_UUID;
#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct AlPersistentID {
    fIdentifier0: i32,
    fIdentifier1: i32,
    fIdentifier2: i32,
    fIdentifier3: i32,
}

impl AlPersistentID {
    pub const fn new() -> Self {
        Self {
            fIdentifier0: 0,
            fIdentifier1: 0,
            fIdentifier2: 0,
            fIdentifier3: 0,
        }
    }
    /* 
    pub fn CreateByUUID(uuid: &RU_UUID) -> Self {
        // 小端组合前 4 字节
        let d4_part =
            u32::from_le_bytes([uuid.Data4[0], uuid.Data4[1], uuid.Data4[2], uuid.Data4[3]]);
        AlPersistentID::create_from_values(
            uuid.Data1 as i32,
            uuid.Data2 as i32,
            uuid.Data3 as i32,
            d4_part as i32,
        )
    }
    */
    pub fn create_from_values(id0: i32, id1: i32, id2: i32, id3: i32) -> Self {
        AlPersistentID { fIdentifier0: id0, fIdentifier1: id1, fIdentifier2: id2, fIdentifier3: id3 }
    }
    pub fn is_empty(&self) -> bool {
        self.fIdentifier0 == 0 && self.fIdentifier1 == 0 && self.fIdentifier2 == 0 && self.fIdentifier3 == 0
    }

}


unsafe extern "C" {

}
