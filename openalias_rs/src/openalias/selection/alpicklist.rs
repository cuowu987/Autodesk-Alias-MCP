#![allow(non_camel_case_types)]
use crate::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AlUserPickList {
    _private: [u8; 0],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlPickMaskType {
    kMaskUnchanged = 0x0,
    kMaskTemplate = 0x1,
    kMaskRoot = 0x2,
    kMaskInterior = 0x4,
    kMaskLeaf = 0x8,
    kMaskLight = 0x10,
    kMaskCurveOnSurface = 0x20,
    kMaskCamera = 0x40,
    kMaskLine = 0x80,
    kMaskPoint = 0x100,
    kMaskEditPoint = 0x200,
    kMaskMeshCurve = 0x400,
    kMaskImagePlane = 0x800,
    kMaskJoint = 0x1000,
    kMaskCluster = 0x2000,
    kMaskSelectionHandle = 0x4000,
    kMaskIKHandle = 0x8000,
    kMaskSurfaceCurve = 0x10000,
    kMaskCurve = 0x20000,
}

impl AlPickMaskType {
    pub fn bits(self) -> i32 {
        self as i32
    }

    pub fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            0x0 => Some(Self::kMaskUnchanged),
            0x1 => Some(Self::kMaskTemplate),
            0x2 => Some(Self::kMaskRoot),
            0x4 => Some(Self::kMaskInterior),
            0x8 => Some(Self::kMaskLeaf),
            0x10 => Some(Self::kMaskLight),
            0x20 => Some(Self::kMaskCurveOnSurface),
            0x40 => Some(Self::kMaskCamera),
            0x80 => Some(Self::kMaskLine),
            0x100 => Some(Self::kMaskPoint),
            0x200 => Some(Self::kMaskEditPoint),
            0x400 => Some(Self::kMaskMeshCurve),
            0x800 => Some(Self::kMaskImagePlane),
            0x1000 => Some(Self::kMaskJoint),
            0x2000 => Some(Self::kMaskCluster),
            0x4000 => Some(Self::kMaskSelectionHandle),
            0x8000 => Some(Self::kMaskIKHandle),
            0x10000 => Some(Self::kMaskSurfaceCurve),
            0x20000 => Some(Self::kMaskCurve),
            _ => None,
        }
    }

    pub fn contains(self, other: Self) -> bool {
        (self.bits() & other.bits()) != 0
    }

    pub fn insert(&mut self, other: Self) {
        *self = unsafe { std::mem::transmute(self.bits() | other.bits()) };
    }

    pub fn remove(&mut self, other: Self) {
        *self = unsafe { std::mem::transmute(self.bits() & !other.bits()) };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlPickMask {
    bits: i32,
}

impl AlPickMask {
    pub fn empty() -> Self {
        Self { bits: 0 }
    }

    pub fn all() -> Self {
        Self { bits: 0x3FFFF }
    }

    pub fn from_bits(bits: i32) -> Self {
        Self { bits }
    }

    pub fn bits(self) -> i32 {
        self.bits
    }

    pub fn insert(&mut self, mask_type: AlPickMaskType) {
        self.bits |= mask_type.bits();
    }

    pub fn remove(&mut self, mask_type: AlPickMaskType) {
        self.bits &= !mask_type.bits();
    }

    pub fn contains(&self, mask_type: AlPickMaskType) -> bool {
        (self.bits & mask_type.bits()) != 0
    }

    pub fn set(&mut self, mask_type: AlPickMaskType, value: bool) {
        if value {
            self.insert(mask_type);
        } else {
            self.remove(mask_type);
        }
    }
}

pub struct AlPickList;

impl AlPickList {
    pub fn is_valid() -> bool {
        unsafe { alpicklist_is_valid() }
    }

    pub fn get_object() -> Option<AlObject> {
        let ptr = unsafe { alpicklist_get_object() };
        if ptr.is_null() {
            None
        } else {
            Some(AlObject { ptr })
        }
    }

    pub fn get_parent_of_object() -> Option<AlDagNode> {
        let ptr = unsafe { alpicklist_get_parent_of_object() };
        if ptr.is_null() {
            None
        } else {
            Some(AlDagNode { ptr })
        }
    }

    pub fn first_pick_item() -> bool {
        let status = unsafe { alpicklist_first_pick_item() };
        if status == statusCode::Success {
            true
        } else {
            false
        }
    }

    pub fn next_pick_item() -> bool {
        let status = unsafe { alpicklist_next_pick_item() };
        if status == statusCode::Success {
            true
        } else {
            false
        }
    }

    pub fn prev_pick_item() -> Result<(), String> {
        let status = unsafe { alpicklist_prev_pick_item() };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn apply_iterator_to_items(iterator: &mut AlIterator) -> Result<i32, String> {
        let mut out_count: i32 = 0;
        let status = unsafe { alpicklist_apply_iterator_to_items(iterator.ptr, &mut out_count) };
        if status == statusCode::Success {
            Ok(out_count)
        } else {
            Err(status.to_string())
        }
    }

    pub fn clear_pick_list() -> bool {
        let status = unsafe { alpicklist_clear_pick_list() };
        if status == statusCode::Success {
            true
        } else {
            false
        }
    }

    pub fn pick_by_name(name: &str) -> Result<(), String> {
        let c_name = std::ffi::CString::new(name).map_err(|_| "Invalid string".to_string())?;
        let status = unsafe { alpicklist_pick_by_name(c_name.as_ptr() as *mut i8) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn pick_from_screen(x: i32, y: i32) -> Result<(), String> {
        let status = unsafe { alpicklist_pick_from_screen(x, y) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn pick_chain_from_screen(x: i32, y: i32) -> Result<(), String> {
        let status = unsafe { alpicklist_pick_chain_from_screen(x, y) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn pick_area_from_screen(x1: i32, y1: i32, x2: i32, y2: i32) -> Result<(), String> {
        let status = unsafe { alpicklist_pick_area_from_screen(x1, y1, x2, y2) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn pick_chain_area_from_screen(x1: i32, y1: i32, x2: i32, y2: i32) -> Result<(), String> {
        let status = unsafe { alpicklist_pick_chain_area_from_screen(x1, y1, x2, y2) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn push_pick_list(save_selection: bool) -> Result<(), String> {
        let status = unsafe { alpicklist_push_pick_list(save_selection) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn pop_pick_list() -> Result<(), String> {
        let status = unsafe { alpicklist_pop_pick_list() };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_pick_mask(mask: AlPickMaskType) -> Result<(), String> {
        let status = unsafe { alpicklist_set_pick_mask(mask.bits()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn get_pick_mask() -> Result<AlPickMask, String> {
        let mut out_mask: i32 = 0;
        let status = unsafe { alpicklist_get_pick_mask(&mut out_mask) };
        if status == statusCode::Success {
            Ok(AlPickMask::from_bits(out_mask))
        } else {
            Err(status.to_string())
        }
    }

    pub fn asynchronous_pick(pick_list: &mut AlUserPickList) -> Result<(), String> {
        let status = unsafe { alpicklist_asynchronous_pick(*pick_list) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn pick_name(name: &str) -> Option<AlObject> {
        Self::clear_pick_list();
        let s = match Self::pick_by_name(name) {
            Ok(_) => match Self::first_pick_item() {
                true => Self::get_object(),
                false => None,
            },
            Err(_) => {
                match AlUniverse::construction_entities().find(|c| c.name() == name) {
                    Some(c) => c.as_object().ok(),
                    None => None,
                }
            }
        };
        Self::clear_pick_list();
        s
    }
    pub fn pick_first() -> Option<AlObject> {
        match Self::first_pick_item() {
            true => Self::get_object(),
            false => None,
        }
    }

    pub fn pick_all() -> impl Iterator<Item = AlObject> {
        Self::first_pick_item();
        let s = std::iter::successors(Self::get_object(), |_prev| {
            if false == Self::next_pick_item() {
                return None;
            };
            Self::get_object()
        });
        s
    }
    pub fn picklist_id() -> String {
        Self::pick_all().map(|p| p.name_ex()).collect::<Vec<_>>().join(" ")
    }
}

unsafe extern "C" {
    fn alpicklist_is_valid() -> bool;

    fn alpicklist_get_object() -> *mut AlObject_ptr;
    fn alpicklist_get_parent_of_object() -> *mut AlDagNode_ptr;

    fn alpicklist_first_pick_item() -> statusCode;
    fn alpicklist_next_pick_item() -> statusCode;
    fn alpicklist_prev_pick_item() -> statusCode;
    fn alpicklist_apply_iterator_to_items(
        iterator: *mut AlIterator_ptr,
        out_count: *mut i32,
    ) -> statusCode;

    fn alpicklist_clear_pick_list() -> statusCode;
    fn alpicklist_pick_by_name(name: *mut i8) -> statusCode;
    fn alpicklist_pick_from_screen(x: i32, y: i32) -> statusCode;
    fn alpicklist_pick_chain_from_screen(x: i32, y: i32) -> statusCode;
    fn alpicklist_pick_area_from_screen(x1: i32, y1: i32, x2: i32, y2: i32) -> statusCode;
    fn alpicklist_pick_chain_area_from_screen(x1: i32, y1: i32, x2: i32, y2: i32) -> statusCode;

    fn alpicklist_push_pick_list(save_selection: bool) -> statusCode;
    fn alpicklist_pop_pick_list() -> statusCode;

    fn alpicklist_set_pick_mask(mask: i32) -> statusCode;
    fn alpicklist_get_pick_mask(out_mask: *mut i32) -> statusCode;

    fn alpicklist_asynchronous_pick(pick_list: AlUserPickList) -> statusCode;
}
