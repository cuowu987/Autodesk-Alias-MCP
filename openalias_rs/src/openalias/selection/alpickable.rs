use crate::*;

#[repr(C)]
pub struct AlPickable_ptr {
    _private: [u8; 0],
}

#[derive(Debug)]
pub struct AlPickable {
    pub ptr: *mut AlPickable_ptr,
}

pub trait AlPickableMethods {
    fn as_pickable_ptr(&self) -> *mut AlPickable_ptr;

    fn pick(&self) -> Result<(), String> {
        let status = unsafe { alpickable_pick(self.as_pickable_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn unpick(&self) -> Result<(), String> {
        let status = unsafe { alpickable_unpick(self.as_pickable_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn is_picked(&self) -> bool {
        unsafe { alpickable_is_picked(self.as_pickable_ptr()) }
    }
}

impl AlPickableMethods for AlPickable {
    fn as_pickable_ptr(&self) -> *mut AlPickable_ptr {
        self.ptr
    }
}



pub fn pick_multi(pickables: &[&impl AlPickableMethods]) -> Result<(), String> {
    let mut ptrs: Vec<*mut AlPickable_ptr> = pickables.iter().map(|p| p.as_pickable_ptr()).collect();
    let status = unsafe { alpickable_pick_multi(ptrs.as_mut_ptr(), ptrs.len()) };
    if status == statusCode::Success {
        Ok(())
    } else {
        Err(status.to_string())
    }
}

pub fn unpick_multi(pickables: &[&impl AlPickableMethods]) -> Result<(), String> {
    let mut ptrs: Vec<*mut AlPickable_ptr> = pickables.iter().map(|p| p.as_pickable_ptr()).collect();
    let status = unsafe { alpickable_unpick_multi(ptrs.as_mut_ptr(), ptrs.len()) };
    if status == statusCode::Success {
        Ok(())
    } else {
        Err(status.to_string())
    }
}

unsafe extern "C" {
    fn alpickable_pick(pickable: *mut AlPickable_ptr) -> statusCode;
    fn alpickable_pick_multi(pickables: *mut *mut AlPickable_ptr, numPickables: usize) -> statusCode;
    fn alpickable_unpick(pickable: *mut AlPickable_ptr) -> statusCode;
    fn alpickable_unpick_multi(pickables: *mut *mut AlPickable_ptr, numPickables: usize) -> statusCode;
    fn alpickable_is_picked(pickable: *mut AlPickable_ptr) -> bool;
}
