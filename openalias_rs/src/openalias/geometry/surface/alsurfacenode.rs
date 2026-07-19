use crate::*;

#[repr(C)]
pub struct AlSurfaceNode_ptr {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug)]
pub struct AlSurfaceNode {
    pub ptr: *mut AlSurfaceNode_ptr,
}

impl AlSurfaceNode {
    pub fn new() -> AlSurfaceNode {
        let ptr = unsafe { alsurfacenode_new() };
        AlSurfaceNode { ptr }
    }

    pub fn create(&mut self, surface: &AlSurface) -> Result<(), String> {
        let status = unsafe { alsurfacenode_create(self.ptr, surface.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }
    pub fn new_create(surface: &AlSurface) -> Result<AlSurfaceNode, String> {
        let mut node: AlSurfaceNode = AlSurfaceNode::new();
        node.create(surface)?;
        Ok(node)
    }

    pub fn surface(&self) -> Option<AlSurface> {
        let ptr = unsafe { alsurfacenode_surface(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlSurface { ptr })
        }
    }

    pub fn surface_tm(&self, tm: &AlTM) -> Option<AlSurface> {
        let ptr = unsafe { alsurfacenode_surface_tm(self.ptr, tm) };
        if ptr.is_null() {
            None
        } else {
            Some(AlSurface { ptr })
        }
    }

    pub fn patch_precision(&self) -> i32 {
        unsafe { alsurfacenode_patch_precision(self.ptr) }
    }

    pub fn set_patch_precision(&self, precision: i32) -> Result<(), String> {
        let status = unsafe { alsurfacenode_set_patch_precision(self.ptr, precision) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }
}

impl AlPickableMethods for AlSurfaceNode {
    fn as_pickable_ptr(&self) -> *mut AlPickable_ptr {
        self.ptr as *mut AlPickable_ptr
    }
}

impl AlDagNodeMethods for AlSurfaceNode {
    fn as_dag_node_ptr(&self) -> *mut AlDagNode_ptr {
        self.ptr as *mut AlDagNode_ptr
    }
}

impl AlObjectMethods for AlSurfaceNode {
    fn as_object_ptr(&self) -> *mut AlObject_ptr {
        self.ptr as *mut AlObject_ptr
    }
}

impl Drop for AlSurfaceNode {
    fn drop(&mut self) {
        unsafe {
            alsurfacenode_delete(self.ptr);
            self.ptr = std::ptr::null_mut();
        }
    }
}

unsafe extern "C" {
    fn alsurfacenode_new() -> *mut AlSurfaceNode_ptr;
    fn alsurfacenode_delete(surfaceNode: *mut AlSurfaceNode_ptr);
    fn alsurfacenode_create(surfaceNode: *mut AlSurfaceNode_ptr, surface: *mut AlSurface_ptr) -> statusCode;
    fn alsurfacenode_surface(surfaceNode: *mut AlSurfaceNode_ptr) -> *mut AlSurface_ptr;
    fn alsurfacenode_surface_tm(surfaceNode: *mut AlSurfaceNode_ptr, tm: *const AlTM) -> *mut AlSurface_ptr;
    fn alsurfacenode_patch_precision(surfaceNode: *mut AlSurfaceNode_ptr) -> i32;
    fn alsurfacenode_set_patch_precision(surfaceNode: *mut AlSurfaceNode_ptr, precision: i32) -> statusCode;
}
