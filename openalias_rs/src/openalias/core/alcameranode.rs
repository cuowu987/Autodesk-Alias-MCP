use crate::*;

#[repr(C)]
pub struct AlCameraNode_ptr {
    _private: [u8; 0],
}

#[derive(Debug)]
pub struct AlCameraNode {
    pub ptr: *mut AlCameraNode_ptr,
}

impl AlCameraNode {
    pub fn is_eye_node(&self) -> bool {
        unsafe { alcameranode_is_eye_node(self.ptr) }
    }

    pub fn is_view_node(&self) -> bool {
        unsafe { alcameranode_is_view_node(self.ptr) }
    }

    pub fn is_up_node(&self) -> bool {
        unsafe { alcameranode_is_up_node(self.ptr) }
    }

    pub fn camera(&self) -> Option<AlCamera> {
        let ptr = unsafe { alcameranode_camera(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlCamera { ptr })
        }
    }

    pub fn camera_tm(&self, out_tm: &mut AlTM) -> Option<AlCamera> {
        let ptr = unsafe { alcameranode_camera_tm(self.ptr, out_tm) };
        if ptr.is_null() {
            None
        } else {
            Some(AlCamera { ptr })
        }
    }
}

impl AlObjectMethods for AlCameraNode {
    fn as_object_ptr(&self) -> *mut AlObject_ptr {
        self.ptr as *mut AlObject_ptr
    }
}

impl AlPickableMethods for AlCameraNode {
    fn as_pickable_ptr(&self) -> *mut AlPickable_ptr {
        self.ptr as *mut AlPickable_ptr
    }
}

impl AlDagNodeMethods for AlCameraNode {
    fn as_dag_node_ptr(&self) -> *mut AlDagNode_ptr {
        self.ptr as *mut AlDagNode_ptr
    }
}

impl Drop for AlCameraNode {
    fn drop(&mut self) {
        self.destroy();
        self.ptr = std::ptr::null_mut();
    }
}

unsafe extern "C" {
    fn alcameranode_is_eye_node(cameraNode: *const AlCameraNode_ptr) -> bool;
    fn alcameranode_is_view_node(cameraNode: *const AlCameraNode_ptr) -> bool;
    fn alcameranode_is_up_node(cameraNode: *const AlCameraNode_ptr) -> bool;
    fn alcameranode_camera(cameraNode: *const AlCameraNode_ptr) -> *mut AlCamera_ptr;
    fn alcameranode_camera_tm(
        cameraNode: *const AlCameraNode_ptr,
        outTm: *mut AlTM,
    ) -> *mut AlCamera_ptr;
}
