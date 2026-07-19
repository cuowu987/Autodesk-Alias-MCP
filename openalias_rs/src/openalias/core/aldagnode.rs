use crate::*;

#[repr(C)]
pub struct AlDagNode_ptr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct AlCopyOptions {
    _private: [u8; 0],
}

impl Default for AlCopyOptions {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[derive(Debug)]
pub struct AlDagNode {
    pub ptr: *mut AlDagNode_ptr,
}
unsafe impl Sync for AlDagNode {}
unsafe impl Send for AlDagNode {}

impl AlPickableMethods for AlDagNode {
    fn as_pickable_ptr(&self) -> *mut AlPickable_ptr {
        self.ptr as *mut AlPickable_ptr
    }
}
pub trait AlDagNodeMethods: AlObjectMethods + AlPickableMethods {
    fn as_dag_node_ptr(&self) -> *mut AlDagNode_ptr;

    fn copy_object(&self, options: Option<*const AlCopyOptions>) -> Option<AlDagNode> {
        let options_ptr = options.unwrap_or(std::ptr::null());
        let ptr = unsafe { aldagnode_copy_object(self.as_dag_node_ptr(), options_ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlDagNode { ptr })
        }
    }

    fn copy_object_ex(
        &self,
        options: Option<*const AlCopyOptions>,
        param2: i32,
        param3: &mut i32,
    ) -> Option<AlDagNode> {
        let options_ptr = options.unwrap_or(std::ptr::null());
        let ptr = unsafe {
            aldagnode_copy_object_ex(self.as_dag_node_ptr(), options_ptr, param2, param3)
        };
        if ptr.is_null() {
            None
        } else {
            Some(AlDagNode { ptr })
        }
    }

    fn parent_node(&self) -> Option<AlGroupNode> {
        let ptr = unsafe { aldagnode_parent_node(self.as_dag_node_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(AlGroupNode { ptr })
        }
    }

    fn next_node(&self) -> Option<AlDagNode> {
        let ptr = unsafe { aldagnode_next_node(self.as_dag_node_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(AlDagNode { ptr })
        }
    }

    fn prev_node(&self) -> Option<AlDagNode> {
        let ptr = unsafe { aldagnode_prev_node(self.as_dag_node_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(AlDagNode { ptr })
        }
    }

    fn is_ancestor_an_instance(&self) -> bool {
        unsafe { aldagnode_is_ancestor_an_instance(self.as_dag_node_ptr()) }
    }

    fn add_sibling_node(&self, sibling: &impl AlDagNodeMethods) -> Result<(), String> {
        let status = unsafe {
            aldagnode_add_sibling_node(self.as_dag_node_ptr(), sibling.as_dag_node_ptr())
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn comment(&self) -> Result<(i64, String), String> {
        let mut id: i64 = 0;
        let mut text: *const i8 = std::ptr::null();
        let status = unsafe { aldagnode_comment(self.as_dag_node_ptr(), &mut id, &mut text) };
        if status == statusCode::Success {
            let text_str = if text.is_null() {
                "".to_string()
            } else {
                unsafe { std::ffi::CStr::from_ptr(text) }
                    .to_string_lossy()
                    .to_string()
            };
            Ok((id, text_str))
        } else {
            Err(status.to_string())
        }
    }

    fn set_comment(&self, id: i64, text: &str) -> Result<(), String> {
        let c_text = std::ffi::CString::new(text).map_err(|_| "Invalid string".to_string())?;
        let status = unsafe { aldagnode_set_comment(self.as_dag_node_ptr(), id, c_text.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn remove_comment(&self) -> Result<(), String> {
        let status = unsafe { aldagnode_remove_comment(self.as_dag_node_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn blind_data(&self, index: i32) -> Result<(i64, String), String> {
        let mut id: i64 = 0;
        let mut data: *const i8 = std::ptr::null();
        let status =
            unsafe { aldagnode_blind_data(self.as_dag_node_ptr(), index, &mut id, &mut data) };
        if status == statusCode::Success {
            let data_str = if data.is_null() {
                "".to_string()
            } else {
                unsafe { std::ffi::CStr::from_ptr(data) }
                    .to_string_lossy()
                    .to_string()
            };
            Ok((id, data_str))
        } else {
            Err(status.to_string())
        }
    }

    fn set_blind_data(&self, index: i32, id: i64, data: &str) -> Result<(), String> {
        let c_data = std::ffi::CString::new(data).map_err(|_| "Invalid string".to_string())?;
        let status =
            unsafe { aldagnode_set_blind_data(self.as_dag_node_ptr(), index, id, c_data.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn remove_blind_data(&self, index: i32) -> Result<(), String> {
        let status = unsafe { aldagnode_remove_blind_data(self.as_dag_node_ptr(), index) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn persistent_id(&self, ut: i32) -> Result<AlPersistentID, String> {
        let mut id = AlPersistentID::default();
        let status = unsafe { aldagnode_persistent_id(self.as_dag_node_ptr(), &mut id, ut) };
        if status == statusCode::Success {
            Ok(id)
        } else {
            Err(status.to_string())
        }
    }

    fn has_persistent_id(&self, ut: i32) -> bool {
        let status = unsafe { aldagnode_has_persistent_id(self.as_dag_node_ptr(), ut) };
        if status == statusCode::Success {
            true
        } else {
            false
        }
    }

    fn set_persistent_id(&self, id: AlPersistentID, ut: i32) -> Result<(), String> {
        let status = unsafe { aldagnode_set_persistent_id(self.as_dag_node_ptr(), id, ut) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn local_transformation_matrix(&self) -> Result<[[f64; 4]; 4], String> {
        let mut matrix: [[f64; 4]; 4] = [[0.0; 4]; 4];
        let status = unsafe {
            aldagnode_local_transformation_matrix(
                self.as_dag_node_ptr(),
                matrix.as_mut_ptr() as *mut _,
            )
        };
        if status == statusCode::Success {
            Ok(matrix)
        } else {
            Err(status.to_string())
        }
    }

    fn global_transformation_matrix(&self) -> Result<[[f64; 4]; 4], String> {
        let mut matrix: [[f64; 4]; 4] = [[0.0; 4]; 4];
        let status = unsafe {
            aldagnode_global_transformation_matrix(
                self.as_dag_node_ptr(),
                matrix.as_mut_ptr() as *mut _,
            )
        };
        if status == statusCode::Success {
            Ok(matrix)
        } else {
            Err(status.to_string())
        }
    }

    fn inverse_global_transformation_matrix(&self) -> Result<[[f64; 4]; 4], String> {
        let mut matrix: [[f64; 4]; 4] = [[0.0; 4]; 4];
        let status = unsafe {
            aldagnode_inverse_global_transformation_matrix(
                self.as_dag_node_ptr(),
                matrix.as_mut_ptr() as *mut _,
            )
        };
        if status == statusCode::Success {
            Ok(matrix)
        } else {
            Err(status.to_string())
        }
    }

    fn affected_transformation_matrix(&self, tm: &AlTM) -> Result<[[f64; 4]; 4], String> {
        let mut matrix: [[f64; 4]; 4] = [[0.0; 4]; 4];
        let status = unsafe {
            aldagnode_affected_transformation_matrix(
                self.as_dag_node_ptr(),
                tm,
                matrix.as_mut_ptr() as *mut _,
            )
        };
        if status == statusCode::Success {
            Ok(matrix)
        } else {
            Err(status.to_string())
        }
    }

    fn local_transformation_matrix_tm(&self, tm: AlTM) -> Result<(), String> {
        let status =
            unsafe { aldagnode_local_transformation_matrix_tm(self.as_dag_node_ptr(), tm) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn global_transformation_matrix_tm(&self, tm: AlTM) -> Result<(), String> {
        let status =
            unsafe { aldagnode_global_transformation_matrix_tm(self.as_dag_node_ptr(), tm) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn inverse_global_transformation_matrix_tm(&self, tm: AlTM) -> Result<(), String> {
        let status = unsafe {
            aldagnode_inverse_global_transformation_matrix_tm(self.as_dag_node_ptr(), tm)
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn affected_transformation_matrix_tm(
        &self,
        input_tm: AlTM,
        output_tm: &mut AlTM,
    ) -> Result<(), String> {
        let status = unsafe {
            aldagnode_affected_transformation_matrix_tm(self.as_dag_node_ptr(), input_tm, output_tm)
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn translation(&self) -> Result<[f64; 3], String> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;
        let status =
            unsafe { aldagnode_translation(self.as_dag_node_ptr(), &mut x, &mut y, &mut z) };
        if status == statusCode::Success {
            Ok([x, y, z])
        } else {
            Err(status.to_string())
        }
    }

    fn rotation(&self) -> Result<[f64; 3], String> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;
        let status = unsafe { aldagnode_rotation(self.as_dag_node_ptr(), &mut x, &mut y, &mut z) };
        if status == statusCode::Success {
            Ok([x, y, z])
        } else {
            Err(status.to_string())
        }
    }

    fn scale(&self) -> Result<(f64, f64, f64), String> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;
        let status = unsafe { aldagnode_scale(self.as_dag_node_ptr(), &mut x, &mut y, &mut z) };
        if status == statusCode::Success {
            Ok((x, y, z))
        } else {
            Err(status.to_string())
        }
    }

    fn rotate_pivot(&self) -> Result<[f64; 3], String> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;
        let status =
            unsafe { aldagnode_rotate_pivot(self.as_dag_node_ptr(), &mut x, &mut y, &mut z) };
        if status == statusCode::Success {
            Ok([x, y, z])
        } else {
            Err(status.to_string())
        }
    }

    fn scale_pivot(&self) -> Result<[f64; 3], String> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;
        let status =
            unsafe { aldagnode_scale_pivot(self.as_dag_node_ptr(), &mut x, &mut y, &mut z) };
        if status == statusCode::Success {
            Ok([x, y, z])
        } else {
            Err(status.to_string())
        }
    }

    fn rotate_pivot_in(&self) -> Result<[f64; 3], String> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;
        let status =
            unsafe { aldagnode_rotate_pivot_in(self.as_dag_node_ptr(), &mut x, &mut y, &mut z) };
        if status == statusCode::Success {
            Ok([x, y, z])
        } else {
            Err(status.to_string())
        }
    }

    fn rotate_pivot_out(&self) -> Result<[f64; 3], String> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;
        let status =
            unsafe { aldagnode_rotate_pivot_out(self.as_dag_node_ptr(), &mut x, &mut y, &mut z) };
        if status == statusCode::Success {
            Ok([x, y, z])
        } else {
            Err(status.to_string())
        }
    }

    fn scale_pivot_in(&self) -> Result<[f64; 3], String> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;
        let status =
            unsafe { aldagnode_scale_pivot_in(self.as_dag_node_ptr(), &mut x, &mut y, &mut z) };
        if status == statusCode::Success {
            Ok([x, y, z])
        } else {
            Err(status.to_string())
        }
    }

    fn scale_pivot_out(&self) -> Result<[f64; 3], String> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;
        let status =
            unsafe { aldagnode_scale_pivot_out(self.as_dag_node_ptr(), &mut x, &mut y, &mut z) };
        if status == statusCode::Success {
            Ok([x, y, z])
        } else {
            Err(status.to_string())
        }
    }

    fn set_rotate_pivot(&self, x: f64, y: f64, z: f64) -> Result<(), String> {
        let status = unsafe { aldagnode_set_rotate_pivot(self.as_dag_node_ptr(), x, y, z) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn set_scale_pivot(&self, x: f64, y: f64, z: f64) -> Result<(), String> {
        let status = unsafe { aldagnode_set_scale_pivot(self.as_dag_node_ptr(), x, y, z) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn set_translation(&self, x: f64, y: f64, z: f64) -> Result<(), String> {
        let status = unsafe { aldagnode_set_translation(self.as_dag_node_ptr(), x, y, z) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn set_world_translation(&self, x: f64, y: f64, z: f64) -> Result<(), String> {
        let status = unsafe { aldagnode_set_world_translation(self.as_dag_node_ptr(), x, y, z) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn set_rotation(&self, x: f64, y: f64, z: f64) -> Result<(), String> {
        let status = unsafe { aldagnode_set_rotation(self.as_dag_node_ptr(), x, y, z) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn set_scale(&self, x: f64, y: f64, z: f64) -> Result<(), String> {
        let status = unsafe { aldagnode_set_scale(self.as_dag_node_ptr(), x, y, z) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn local_rotation_axes(&self) -> Result<([f64; 3], [f64; 3], [f64; 3]), String> {
        let mut axis1: [f64; 3] = [0.0; 3];
        let mut axis2: [f64; 3] = [0.0; 3];
        let mut axis3: [f64; 3] = [0.0; 3];
        let status = unsafe {
            aldagnode_local_rotation_axes(
                self.as_dag_node_ptr(),
                axis1.as_mut_ptr(),
                axis2.as_mut_ptr(),
                axis3.as_mut_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok((axis1, axis2, axis3))
        } else {
            Err(status.to_string())
        }
    }

    fn set_local_rotation_axes(
        &self,
        axis1: [f64; 3],
        axis2: [f64; 3],
        axis3: [f64; 3],
    ) -> Result<(), String> {
        let status = unsafe {
            aldagnode_set_local_rotation_axes(
                self.as_dag_node_ptr(),
                axis1.as_ptr(),
                axis2.as_ptr(),
                axis3.as_ptr(),
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn local_rotation_angles(&self) -> Result<(f64, f64, f64), String> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;
        let status = unsafe {
            aldagnode_local_rotation_angles(self.as_dag_node_ptr(), &mut x, &mut y, &mut z)
        };
        if status == statusCode::Success {
            Ok((x, y, z))
        } else {
            Err(status.to_string())
        }
    }

    fn set_local_rotation_angles(&self, x: f64, y: f64, z: f64) -> Result<(), String> {
        let status =
            unsafe { aldagnode_set_local_rotation_angles(self.as_dag_node_ptr(), x, y, z) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn local_rotate_by(&self, x: f64, y: f64, z: f64) -> Result<(), String> {
        let status = unsafe { aldagnode_local_rotate_by(self.as_dag_node_ptr(), x, y, z) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn local_translate_by(&self, x: f64, y: f64, z: f64) -> Result<(), String> {
        let status = unsafe { aldagnode_local_translate_by(self.as_dag_node_ptr(), x, y, z) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn copy_transform(&self, source: &impl AlDagNodeMethods) -> Result<(), String> {
        let status =
            unsafe { aldagnode_copy_transform(self.as_dag_node_ptr(), source.as_dag_node_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn bounding_box(&self) -> Result<[[f64; 4]; 8], String> {
        let mut box_: [[f64; 4]; 8] = [[0.0; 4]; 8];
        let status =
            unsafe { aldagnode_bounding_box(self.as_dag_node_ptr(), box_.as_mut_ptr() as *mut _) };
        if status == statusCode::Success {
            Ok(box_)
        } else {
            Err(status.to_string())
        }
    }
    fn dag_center(&self) -> Result<[f64; 3], String> {
        self.set_display_mode(AlDisplayModeType::kDisplayModeBoundingBox, true)?;
        let box_ = self.bounding_box()?;
        self.set_display_mode(AlDisplayModeType::kDisplayModeBoundingBox, false)?;
        let start = box_[0];
        let end = box_[7];
        let center = [
            (start[0] + end[0]) / 2.0,
            (start[1] + end[1]) / 2.0,
            (start[2] + end[2]) / 2.0,
        ];
        Ok(center)
    }

    fn is_display_mode_set(&self, display_mode: AlDisplayModeType) -> bool {
        unsafe { aldagnode_is_display_mode_set(self.as_dag_node_ptr(), display_mode as i32) }
    }

    fn set_display_mode(
        &self,
        display_mode: AlDisplayModeType,
        enable: bool,
    ) -> Result<(), String> {
        let status = unsafe {
            aldagnode_set_display_mode(self.as_dag_node_ptr(), display_mode as i32, enable)
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn joint(&self) -> Option<AlJoint> {
        let ptr = unsafe { aldagnode_joint(self.as_dag_node_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(AlJoint { ptr })
        }
    }

    fn add_joint(&self) -> Result<(), String> {
        let status = unsafe { aldagnode_add_joint(self.as_dag_node_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn remove_joint(&self) -> Result<(), String> {
        let status = unsafe { aldagnode_remove_joint(self.as_dag_node_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn search_below(&self, name: &str) -> Option<AlDagNode> {
        let c_name = match std::ffi::CString::new(name) {
            Ok(s) => s,
            Err(_) => return None,
        };
        let ptr = unsafe { aldagnode_search_below(self.as_dag_node_ptr(), c_name.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(AlDagNode { ptr })
        }
    }

    fn search_across(&self, name: &str) -> Option<AlDagNode> {
        let c_name = match std::ffi::CString::new(name) {
            Ok(s) => s,
            Err(_) => return None,
        };
        let ptr = unsafe { aldagnode_search_across(self.as_dag_node_ptr(), c_name.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(AlDagNode { ptr })
        }
    }

    fn update_draw_info(&self) -> Result<(), String> {
        let status = unsafe { aldagnode_update_draw_info(self.as_dag_node_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn send_geometry_modified_message(&self) -> Result<(), String> {
        let status = unsafe { aldagnode_send_geometry_modified_message(self.as_dag_node_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn do_updates(&self, new_state: bool) -> Result<(), String> {
        let status = unsafe { aldagnode_do_updates(self.as_dag_node_ptr(), new_state) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn first_constraint(&self) -> Option<AlConstraint> {
        let ptr = unsafe { aldagnode_first_constraint(self.as_dag_node_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(AlConstraint { ptr })
        }
    }

    fn layer(&self) -> Option<AlLayer> {
        let ptr = unsafe { aldagnode_layer(self.as_dag_node_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(AlLayer { ptr })
        }
    }

    fn set_layer(&self, layer: &AlLayer) -> Result<(), String> {
        let status = unsafe { aldagnode_set_layer(self.as_dag_node_ptr(), layer.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn set_layer_int(&self, layer_number: i32) -> Result<(), String> {
        let status = unsafe { aldagnode_set_layer_int(self.as_dag_node_ptr(), layer_number) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn has_tool_meta_data(&self) -> bool {
        unsafe { aldagnode_has_tool_meta_data(self.as_dag_node_ptr()) }
    }

    fn get_tool_meta_data(&self, function_title: &str) -> Option<AlToolMetaData> {
        let c_title = match std::ffi::CString::new(function_title) {
            Ok(s) => s,
            Err(_) => return None,
        };
        let ptr = unsafe { aldagnode_get_tool_meta_data(self.as_dag_node_ptr(), c_title.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(AlToolMetaData { ptr })
        }
    }

    fn create_tool_meta_data(&self, function_title: &str) -> Option<AlToolMetaData> {
        let c_title = match std::ffi::CString::new(function_title) {
            Ok(s) => s,
            Err(_) => return None,
        };
        let ptr =
            unsafe { aldagnode_create_tool_meta_data(self.as_dag_node_ptr(), c_title.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(AlToolMetaData { ptr })
        }
    }

    fn is_a_construction_plane(&self) -> bool {
        unsafe { aldagnode_is_a_construction_plane(self.as_dag_node_ptr()) }
    }

    fn is_instanced(&self) -> bool {
        unsafe { aldagnode_is_instanced(self.as_dag_node_ptr()) }
    }

    fn create_symmetric_geometry(&self) -> Result<(), String> {
        let status = unsafe { aldagnode_create_symmetric_geometry(self.as_dag_node_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn create_symmetric_geometry_ex(&self) -> Result<AlDagNode, String> {
        let mut out_node: *mut AlDagNode_ptr = std::ptr::null_mut();
        let status = unsafe {
            aldagnode_create_symmetric_geometry_ex(self.as_dag_node_ptr(), &mut out_node)
        };
        if status == statusCode::Success {
            Ok(AlDagNode { ptr: out_node })
        } else {
            Err(status.to_string())
        }
    }

    fn get_surface_orientation(&self) -> Result<bool, String> {
        let mut orientation: bool = false;
        let status =
            unsafe { aldagnode_get_surface_orientation(self.as_dag_node_ptr(), &mut orientation) };
        if status == statusCode::Success {
            Ok(orientation)
        } else {
            Err(status.to_string())
        }
    }

    fn set_surface_orientation(&self, orientation: bool) -> Result<(), String> {
        let status =
            unsafe { aldagnode_set_surface_orientation(self.as_dag_node_ptr(), orientation) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn version(&self) -> u32 {
        unsafe { aldagnode_version(self.as_dag_node_ptr()) }
    }

    fn cluster_pre_transformation_matrices(&self) -> Option<AlList> {
        let ptr = unsafe { aldagnode_cluster_pre_transformation_matrices(self.as_dag_node_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(AlList { ptr })
        }
    }
       fn translate(&mut self, x: f64, y: f64, z: f64) -> Result<(), String> {
        let p = self.translation()?;
        self.set_translation(p[0] + x, p[1] + y, p[2] + z)?;
        Ok(())
    }
}

impl AlDagNodeMethods for AlDagNode {
    fn as_dag_node_ptr(&self) -> *mut AlDagNode_ptr {
        self.ptr
    }
}

impl AlObjectMethods for AlDagNode {
    fn as_object_ptr(&self) -> *mut AlObject_ptr {
        self.ptr as *mut AlObject_ptr
    }
    fn copy_wrapper_ex(&self) -> Result<Self, String>
    where
        Self: Sized,
    {
        let copy = self.copy_wrapper();
        copy.as_dag_node()
    }
}

impl Drop for AlDagNode {
    fn drop(&mut self) {
        self.destroy();
        self.ptr = std::ptr::null_mut();
    }
}

unsafe extern "C" {
    fn aldagnode_copy_object(
        dagNode: *mut AlDagNode_ptr,
        options: *const AlCopyOptions,
    ) -> *mut AlDagNode_ptr;
    fn aldagnode_copy_object_ex(
        dagNode: *mut AlDagNode_ptr,
        options: *const AlCopyOptions,
        param2: i32,
        param3: *mut i32,
    ) -> *mut AlDagNode_ptr;

    fn aldagnode_parent_node(dagNode: *mut AlDagNode_ptr) -> *mut AlGroupNode_ptr;
    fn aldagnode_next_node(dagNode: *mut AlDagNode_ptr) -> *mut AlDagNode_ptr;
    fn aldagnode_prev_node(dagNode: *mut AlDagNode_ptr) -> *mut AlDagNode_ptr;
    fn aldagnode_is_ancestor_an_instance(dagNode: *mut AlDagNode_ptr) -> bool;
    fn aldagnode_add_sibling_node(
        dagNode: *mut AlDagNode_ptr,
        sibling: *mut AlDagNode_ptr,
    ) -> statusCode;
    fn aldagnode_comment(
        dagNode: *mut AlDagNode_ptr,
        outId: &mut i64,
        outText: &mut *const i8,
    ) -> statusCode;
    fn aldagnode_set_comment(dagNode: *mut AlDagNode_ptr, id: i64, text: *const i8) -> statusCode;
    fn aldagnode_remove_comment(dagNode: *mut AlDagNode_ptr) -> statusCode;
    fn aldagnode_blind_data(
        dagNode: *mut AlDagNode_ptr,
        index: i32,
        outId: &mut i64,
        outData: &mut *const i8,
    ) -> statusCode;
    fn aldagnode_set_blind_data(
        dagNode: *mut AlDagNode_ptr,
        index: i32,
        id: i64,
        data: *const i8,
    ) -> statusCode;
    fn aldagnode_remove_blind_data(dagNode: *mut AlDagNode_ptr, index: i32) -> statusCode;
    fn aldagnode_persistent_id(
        dagNode: *mut AlDagNode_ptr,
        outId: *mut AlPersistentID,
        ut: i32,
    ) -> statusCode;
    fn aldagnode_has_persistent_id(dagNode: *mut AlDagNode_ptr, ut: i32) -> statusCode;
    fn aldagnode_set_persistent_id(
        dagNode: *mut AlDagNode_ptr,
        id: AlPersistentID,
        ut: i32,
    ) -> statusCode;

    fn aldagnode_local_transformation_matrix(
        dagNode: *mut AlDagNode_ptr,
        matrix: *mut f64,
    ) -> statusCode;
    fn aldagnode_global_transformation_matrix(
        dagNode: *mut AlDagNode_ptr,
        matrix: *mut f64,
    ) -> statusCode;
    fn aldagnode_inverse_global_transformation_matrix(
        dagNode: *mut AlDagNode_ptr,
        matrix: *mut f64,
    ) -> statusCode;
    fn aldagnode_affected_transformation_matrix(
        dagNode: *mut AlDagNode_ptr,
        tm: *const AlTM,
        matrix: *mut f64,
    ) -> statusCode;
    fn aldagnode_local_transformation_matrix_tm(
        dagNode: *mut AlDagNode_ptr,
        tm: AlTM,
    ) -> statusCode;
    fn aldagnode_global_transformation_matrix_tm(
        dagNode: *mut AlDagNode_ptr,
        tm: AlTM,
    ) -> statusCode;
    fn aldagnode_inverse_global_transformation_matrix_tm(
        dagNode: *mut AlDagNode_ptr,
        tm: AlTM,
    ) -> statusCode;
    fn aldagnode_affected_transformation_matrix_tm(
        dagNode: *mut AlDagNode_ptr,
        inputTm: AlTM,
        outputTm: *mut AlTM,
    ) -> statusCode;

    fn aldagnode_translation(
        dagNode: *mut AlDagNode_ptr,
        x: &mut f64,
        y: &mut f64,
        z: &mut f64,
    ) -> statusCode;
    fn aldagnode_rotation(
        dagNode: *mut AlDagNode_ptr,
        x: &mut f64,
        y: &mut f64,
        z: &mut f64,
    ) -> statusCode;
    fn aldagnode_scale(
        dagNode: *mut AlDagNode_ptr,
        x: &mut f64,
        y: &mut f64,
        z: &mut f64,
    ) -> statusCode;
    fn aldagnode_rotate_pivot(
        dagNode: *mut AlDagNode_ptr,
        x: &mut f64,
        y: &mut f64,
        z: &mut f64,
    ) -> statusCode;
    fn aldagnode_scale_pivot(
        dagNode: *mut AlDagNode_ptr,
        x: &mut f64,
        y: &mut f64,
        z: &mut f64,
    ) -> statusCode;
    fn aldagnode_rotate_pivot_in(
        dagNode: *mut AlDagNode_ptr,
        x: &mut f64,
        y: &mut f64,
        z: &mut f64,
    ) -> statusCode;
    fn aldagnode_rotate_pivot_out(
        dagNode: *mut AlDagNode_ptr,
        x: &mut f64,
        y: &mut f64,
        z: &mut f64,
    ) -> statusCode;
    fn aldagnode_scale_pivot_in(
        dagNode: *mut AlDagNode_ptr,
        x: &mut f64,
        y: &mut f64,
        z: &mut f64,
    ) -> statusCode;
    fn aldagnode_scale_pivot_out(
        dagNode: *mut AlDagNode_ptr,
        x: &mut f64,
        y: &mut f64,
        z: &mut f64,
    ) -> statusCode;
    fn aldagnode_set_rotate_pivot(
        dagNode: *mut AlDagNode_ptr,
        x: f64,
        y: f64,
        z: f64,
    ) -> statusCode;
    fn aldagnode_set_scale_pivot(dagNode: *mut AlDagNode_ptr, x: f64, y: f64, z: f64)
    -> statusCode;

    fn aldagnode_set_translation(dagNode: *mut AlDagNode_ptr, x: f64, y: f64, z: f64)
    -> statusCode;
    fn aldagnode_set_world_translation(
        dagNode: *mut AlDagNode_ptr,
        x: f64,
        y: f64,
        z: f64,
    ) -> statusCode;
    fn aldagnode_set_rotation(dagNode: *mut AlDagNode_ptr, x: f64, y: f64, z: f64) -> statusCode;
    fn aldagnode_set_scale(dagNode: *mut AlDagNode_ptr, x: f64, y: f64, z: f64) -> statusCode;
    fn aldagnode_local_rotation_axes(
        dagNode: *mut AlDagNode_ptr,
        axis1: *mut f64,
        axis2: *mut f64,
        axis3: *mut f64,
    ) -> statusCode;
    fn aldagnode_set_local_rotation_axes(
        dagNode: *mut AlDagNode_ptr,
        axis1: *const f64,
        axis2: *const f64,
        axis3: *const f64,
    ) -> statusCode;
    fn aldagnode_local_rotation_angles(
        dagNode: *mut AlDagNode_ptr,
        x: &mut f64,
        y: &mut f64,
        z: &mut f64,
    ) -> statusCode;
    fn aldagnode_set_local_rotation_angles(
        dagNode: *mut AlDagNode_ptr,
        x: f64,
        y: f64,
        z: f64,
    ) -> statusCode;

    fn aldagnode_local_rotate_by(dagNode: *mut AlDagNode_ptr, x: f64, y: f64, z: f64)
    -> statusCode;
    fn aldagnode_local_translate_by(
        dagNode: *mut AlDagNode_ptr,
        x: f64,
        y: f64,
        z: f64,
    ) -> statusCode;
    fn aldagnode_copy_transform(
        dagNode: *mut AlDagNode_ptr,
        source: *mut AlDagNode_ptr,
    ) -> statusCode;
    fn aldagnode_bounding_box(dagNode: *mut AlDagNode_ptr, box_: *mut f64) -> statusCode;
    fn aldagnode_is_display_mode_set(dagNode: *mut AlDagNode_ptr, displayMode: i32) -> bool;
    fn aldagnode_set_display_mode(
        dagNode: *mut AlDagNode_ptr,
        displayMode: i32,
        enable: bool,
    ) -> statusCode;

    fn aldagnode_joint(dagNode: *mut AlDagNode_ptr) -> *mut AlJoint_ptr;
    fn aldagnode_add_joint(dagNode: *mut AlDagNode_ptr) -> statusCode;
    fn aldagnode_remove_joint(dagNode: *mut AlDagNode_ptr) -> statusCode;
    fn aldagnode_search_below(dagNode: *mut AlDagNode_ptr, name: *const i8) -> *mut AlDagNode_ptr;
    fn aldagnode_search_across(dagNode: *mut AlDagNode_ptr, name: *const i8) -> *mut AlDagNode_ptr;
    fn aldagnode_update_draw_info(dagNode: *mut AlDagNode_ptr) -> statusCode;
    fn aldagnode_send_geometry_modified_message(dagNode: *mut AlDagNode_ptr) -> statusCode;
    fn aldagnode_do_updates(dagNode: *mut AlDagNode_ptr, newState: bool) -> statusCode;
    fn aldagnode_first_constraint(dagNode: *mut AlDagNode_ptr) -> *mut AlConstraint_ptr;
    fn aldagnode_layer(dagNode: *mut AlDagNode_ptr) -> *mut AlLayer_ptr;
    fn aldagnode_set_layer(dagNode: *mut AlDagNode_ptr, layer: *const AlLayer_ptr) -> statusCode;
    fn aldagnode_set_layer_int(dagNode: *mut AlDagNode_ptr, layerNumber: i32) -> statusCode;
    fn aldagnode_has_tool_meta_data(dagNode: *mut AlDagNode_ptr) -> bool;
    fn aldagnode_get_tool_meta_data(
        dagNode: *mut AlDagNode_ptr,
        functionTitle: *const i8,
    ) -> *mut AlToolMetaData_ptr;
    fn aldagnode_create_tool_meta_data(
        dagNode: *mut AlDagNode_ptr,
        functionTitle: *const i8,
    ) -> *mut AlToolMetaData_ptr;
    fn aldagnode_is_a_construction_plane(dagNode: *mut AlDagNode_ptr) -> bool;
    fn aldagnode_is_instanced(dagNode: *mut AlDagNode_ptr) -> bool;
    fn aldagnode_create_symmetric_geometry(dagNode: *mut AlDagNode_ptr) -> statusCode;
    fn aldagnode_create_symmetric_geometry_ex(
        dagNode: *mut AlDagNode_ptr,
        outNode: *mut *mut AlDagNode_ptr,
    ) -> statusCode;
    fn aldagnode_get_surface_orientation(
        dagNode: *mut AlDagNode_ptr,
        outOrientation: &mut bool,
    ) -> statusCode;
    fn aldagnode_set_surface_orientation(
        dagNode: *mut AlDagNode_ptr,
        orientation: bool,
    ) -> statusCode;
    fn aldagnode_version(dagNode: *mut AlDagNode_ptr) -> u32;
    fn aldagnode_cluster_pre_transformation_matrices(
        dagNode: *mut AlDagNode_ptr,
    ) -> *mut AlList_ptr;
}
