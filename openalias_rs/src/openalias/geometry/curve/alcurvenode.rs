use crate::*;
#[allow(non_camel_case_types)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlCurveNodeJoinErrors {
    kJoinSuccess = 0,
    kJoinFailure = 1,
    kJoinBadCluster = 2,
    kJoinInvalidKeyPoints = 3,
    kJoinNoAttributes = 4,
    kJoinBadData = 5,
    kJoinDuplicateCurve = 6,
    kJoinCurveClosed = 7,
}

#[repr(C)]
pub struct AlCurveNode_ptr {
    _private: [u8; 0],
}

#[derive(Debug)]
pub struct AlCurveNode {
    pub ptr: *mut AlCurveNode_ptr,
}

impl AlCurveNode {
    pub fn new() -> AlCurveNode {
        let ptr = unsafe { alcurvenode_new() };
        AlCurveNode { ptr }
    }

    pub fn create(&mut self, curve: &AlCurve) -> Result<(), String> {
        let status = unsafe { alcurvenode_create(self.ptr, curve.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }
    pub fn new_create(curve: &AlCurve) -> Result<AlCurveNode, String> {
        let mut node = AlCurveNode::new();
        node.create(curve)?;
        Ok(node)
    }

    pub fn curve(&self) -> Option<AlCurve> {
        let ptr = unsafe { alcurvenode_curve(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlCurve { ptr })
        }
    }

    pub fn curve_tm(&self, tm: &mut AlTM) -> Option<AlCurve> {
        let ptr = unsafe { alcurvenode_curve_tm(self.ptr, tm) };
        if ptr.is_null() {
            None
        } else {
            Some(AlCurve { ptr })
        }
    }

    pub fn join(&self, other: &AlCurveNode) -> AlCurveNodeJoinErrors {
        unsafe { alcurvenode_join(self.ptr, other.ptr) }
    }
}

impl AlObjectMethods for AlCurveNode {
    fn as_object_ptr(&self) -> *mut AlObject_ptr {
        self.ptr as *mut AlObject_ptr
    }
}

impl AlPickableMethods for AlCurveNode {
    fn as_pickable_ptr(&self) -> *mut AlPickable_ptr {
        self.ptr as *mut AlPickable_ptr
    }
}

impl AlDagNodeMethods for AlCurveNode {
    fn as_dag_node_ptr(&self) -> *mut AlDagNode_ptr {
        self.ptr as *mut AlDagNode_ptr
    }
}

impl Drop for AlCurveNode {
    fn drop(&mut self) {
        unsafe {
            alcurvenode_delete(self.ptr);
            self.ptr = std::ptr::null_mut();
        }
    }
}

unsafe extern "C" {
    fn alcurvenode_new() -> *mut AlCurveNode_ptr;
    fn alcurvenode_delete(curve_node: *mut AlCurveNode_ptr);
    fn alcurvenode_create(curve_node: *mut AlCurveNode_ptr, curve: *mut AlCurve_ptr) -> statusCode;
    fn alcurvenode_curve(curve_node: *mut AlCurveNode_ptr) -> *mut AlCurve_ptr;
    fn alcurvenode_curve_tm(curve_node: *mut AlCurveNode_ptr, tm: *mut AlTM) -> *mut AlCurve_ptr;
    fn alcurvenode_join(curve_node: *mut AlCurveNode_ptr, other: *mut AlCurveNode_ptr) -> AlCurveNodeJoinErrors;
}
