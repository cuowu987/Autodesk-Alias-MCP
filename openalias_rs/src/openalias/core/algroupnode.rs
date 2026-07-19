use crate::*;

#[repr(C)]
pub struct AlGroupNode_ptr {
    _private: [u8; 0],
}

#[derive(Debug)]
pub struct AlGroupNode {
    pub ptr: *mut AlGroupNode_ptr,
}

impl AlGroupNode {
    pub fn new() -> AlGroupNode {
        unsafe {
            let ptr = algroupnode_new();
            AlGroupNode { ptr }
        }
    }

    pub fn create(&mut self) -> Result<(), String> {
        let status = unsafe { algroupnode_create(self.ptr) };
        match status {
            statusCode::Success => Ok(()),
            _ => Err(status.to_string()),
        }
    }
    pub fn new_create() -> Result<AlGroupNode, String> {
        let mut group = Self::new();
        group.create()?;
        Ok(group)
    }

    pub fn child_node(&self) -> Option<AlDagNode> {
        let ptr = unsafe { algroupnode_child_node(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlDagNode { ptr })
        }
    }

    pub fn child_nodes(&self) -> impl Iterator<Item = AlDagNode> {
        std::iter::successors(self.child_node(), |prev| prev.next_node())
    }

    pub fn child_node_tm(&self, tm: AlTM) -> Option<AlDagNode> {
        let ptr = unsafe { algroupnode_child_node_tm(self.ptr, tm) };
        if ptr.is_null() {
            None
        } else {
            Some(AlDagNode { ptr })
        }
    }

    pub fn next_instance(&self) -> Option<AlGroupNode> {
        let ptr = unsafe { algroupnode_next_instance(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlGroupNode { ptr })
        }
    }

    pub fn prev_instance(&self) -> Option<AlGroupNode> {
        let ptr = unsafe { algroupnode_prev_instance(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlGroupNode { ptr })
        }
    }

    pub fn next_instance_d(&self) -> Result<(), String> {
        let status = unsafe { algroupnode_next_instance_d(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn prev_instance_d(&self) -> Result<(), String> {
        let status = unsafe { algroupnode_prev_instance_d(self.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn is_instanceable(&self) -> bool {
        unsafe { algroupnode_is_instanceable(self.ptr) }
    }

    pub fn is_instance_node(&self) -> bool {
        unsafe { algroupnode_is_instance_node(self.ptr) }
    }

    pub fn add_child_node(&mut self, child: &AlDagNode) -> Result<(), String> {
        let status = unsafe { algroupnode_add_child_node(self.ptr, child.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn apply_iterator_to_children(
        &self,
        iterator: &AlIterator,
        result: &mut i32,
    ) -> Result<(), String> {
        let status =
            unsafe { algroupnode_apply_iterator_to_children(self.ptr, iterator.ptr, result) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }
    pub fn get_geometry(&self) -> Result<(Vec<AlSurfaceNode>, Vec<AlCurveNode>), String> {
        let mut surfaces = Vec::new();
        let mut curves = Vec::new();
        self.collect_geometry_recursive(&mut surfaces, &mut curves)?;
        Ok((surfaces, curves))
    }

    /// 递归收集所有 SurfaceNode 和 CurveNode
    fn collect_geometry_recursive(
        &self,
        surfaces: &mut Vec<AlSurfaceNode>,
        curves: &mut Vec<AlCurveNode>,
    ) -> Result<(), String> {
        for child in self.child_nodes() {
            let node_type = child.type_();

            match node_type {
                AlObjectType::kSurfaceNodeType => {
                    if let Ok(surface_node) = child.as_surface_node() {
                        surfaces.push(surface_node);
                    }
                }
                AlObjectType::kCurveNodeType => {
                    if let Ok(curve_node) = child.as_curve_node() {
                        curves.push(curve_node);
                    }
                }

                AlObjectType::kGroupNodeType => {
                    if let Ok(group_node) = child.as_group_node() {
                        group_node.collect_geometry_recursive(surfaces, curves)?;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
    /// 递归收集所有 DagNode
    fn collect_geometry_recursive_dag(&self, dag_nodes: &mut Vec<AlDagNode>) -> Result<(), String> {
        for child in self.child_nodes() {
            let node_type = child.type_();

            match node_type {
                AlObjectType::kSurfaceNodeType
                | AlObjectType::kCurveNodeType
                | AlObjectType::kDagNodeType
                | AlObjectType::kMeshNodeType => {
                    if let Ok(dag_node) = child.as_dag_node() {
                        dag_nodes.push(dag_node);
                    }
                }
                AlObjectType::kGroupNodeType => {
                    if let Ok(group_node) = child.as_group_node() {
                        group_node.collect_geometry_recursive_dag(dag_nodes)?;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
    pub fn collect_geometry(&self) -> Result<(Vec<AlSurfaceNode>, Vec<AlCurveNode>), String> {
        let mut surfaces = Vec::new();
        let mut curves = Vec::new();
        self.collect_geometry_recursive(&mut surfaces, &mut curves)?;
        Ok((surfaces, curves))
    }
    pub fn collect_geometry_dag(&self) -> Result<Vec<AlDagNode>, String> {
        let mut dag_nodes = Vec::new();
        self.collect_geometry_recursive_dag(&mut dag_nodes)?;
        Ok(dag_nodes)
    }
}

impl AlObjectMethods for AlGroupNode {
    fn as_object_ptr(&self) -> *mut AlObject_ptr {
        self.ptr as *mut AlObject_ptr
    }
}

impl AlPickableMethods for AlGroupNode {
    fn as_pickable_ptr(&self) -> *mut AlPickable_ptr {
        self.ptr as *mut AlPickable_ptr
    }
}

impl AlDagNodeMethods for AlGroupNode {
    fn as_dag_node_ptr(&self) -> *mut AlDagNode_ptr {
        self.ptr as *mut AlDagNode_ptr
    }
}

impl Drop for AlGroupNode {
    fn drop(&mut self) {
        unsafe {
            algroupnode_destroy(self.ptr);
            self.ptr = std::ptr::null_mut();
        }
    }
}

unsafe extern "C" {
    fn algroupnode_new() -> *mut AlGroupNode_ptr;
    fn algroupnode_create(group_node: *mut AlGroupNode_ptr) -> statusCode;
    fn algroupnode_destroy(group_node: *mut AlGroupNode_ptr);
    fn algroupnode_child_node(group_node: *mut AlGroupNode_ptr) -> *mut AlDagNode_ptr;
    fn algroupnode_child_node_tm(group_node: *mut AlGroupNode_ptr, tm: AlTM) -> *mut AlDagNode_ptr;
    fn algroupnode_next_instance(group_node: *mut AlGroupNode_ptr) -> *mut AlGroupNode_ptr;
    fn algroupnode_prev_instance(group_node: *mut AlGroupNode_ptr) -> *mut AlGroupNode_ptr;
    fn algroupnode_next_instance_d(group_node: *mut AlGroupNode_ptr) -> statusCode;
    fn algroupnode_prev_instance_d(group_node: *mut AlGroupNode_ptr) -> statusCode;
    fn algroupnode_is_instanceable(group_node: *mut AlGroupNode_ptr) -> bool;
    fn algroupnode_is_instance_node(group_node: *mut AlGroupNode_ptr) -> bool;
    fn algroupnode_add_child_node(
        group_node: *mut AlGroupNode_ptr,
        child: *mut AlDagNode_ptr,
    ) -> statusCode;
    fn algroupnode_apply_iterator_to_children(
        group_node: *mut AlGroupNode_ptr,
        iterator: *mut AlIterator_ptr,
        result: *mut i32,
    ) -> statusCode;
}
