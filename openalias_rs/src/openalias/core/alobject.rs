use crate::*;

#[repr(C)]
pub struct AlObject_ptr {
    _private: [u8; 0],
}

#[derive(Debug)]
pub struct AlObject {
    pub ptr: *mut AlObject_ptr,
}

pub trait AlObjectMethods {
    fn as_object_ptr(&self) -> *mut AlObject_ptr;

    fn is_valid(&self) -> bool {
        unsafe { alobject_is_valid(self.as_object_ptr()) }
    }

    fn are_equal(&self, other: &impl AlObjectMethods) -> bool {
        unsafe { alobject_are_equal(self.as_object_ptr(), other.as_object_ptr()) }
    }

    fn type_(&self) -> AlObjectType {
        let s = unsafe { alobject_type(self.as_object_ptr()) };
        AlObjectType::from_i32(s).unwrap()
    }

    fn name_ex(&self) -> String {
        match self.type_() {
            AlObjectType::kSurfaceCVType => self.copy_wrapper().as_surface_cv().unwrap().name_ex(),
            AlObjectType::kSurfaceType => self.copy_wrapper().as_surface().unwrap().name_ex(),
            AlObjectType::kCurveType => self.copy_wrapper().as_curve().unwrap().name_ex(),
            AlObjectType::kCurveCVType => self.copy_wrapper().as_curve_cv().unwrap().name_ex(),
            AlObjectType::kSurfaceCurveType => {
                self.copy_wrapper().as_surface_curve().unwrap().name_ex()
            }
            _ => format!("{}_{:?}", self.name(), self.type_()),
        }
    }
    fn name(&self) -> String {
        let c_str = unsafe { alobject_name(self.as_object_ptr()) };
        if c_str.is_null() {
            "".to_string()
        } else {
            unsafe { std::ffi::CStr::from_ptr(c_str) }
                .to_string_lossy()
                .to_string()
        }
    }

    fn set_name(&self, name: &str) -> Result<(), String> {
        let c_name = std::ffi::CString::new(name).map_err(|_| "Invalid string".to_string())?;
        let status = unsafe { alobject_set_name(self.as_object_ptr(), c_name.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn delete_object(&self) -> Result<(), String> {
        let status = unsafe { alobject_delete_object(self.as_object_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    fn destroy(&self) {
        unsafe { alobject_destroy(self.as_object_ptr()) }
    }

    fn copy_wrapper(&self) -> AlObject {
        let ptr = unsafe { alobject_copy_wrapper(self.as_object_ptr()) };
        if ptr.is_null() {
            panic!("copy_wrapper is error");
        } else {
            AlObject { ptr }
        }
    }
    fn copy_wrapper_ex(&self) -> Result<Self, String>
    where
        Self: Sized,
    {
        Err("copy_wrapper_ex not implemented for this type".to_string())
    }

    fn as_object(self) -> Result<AlObject, String>
    where
        Self: Sized,
    {
        Ok(AlObject {
            ptr: self.as_object_ptr(),
        })
    }

    fn as_animatable(self) -> Result<AlAnimatable, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_animatable_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlAnimatable", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlAnimatable { ptr })
        }
    }

    fn as_clusterable(self) -> Result<AlClusterable, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_clusterable_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlClusterable",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlClusterable { ptr })
        }
    }

    fn as_settable(self) -> Result<AlSettable, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_settable_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlSettable", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlSettable { ptr })
        }
    }

    fn as_pickable(self) -> Result<AlPickable, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_pickable_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlPickable", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlPickable { ptr })
        }
    }

    fn as_camera(self) -> Result<AlCamera, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_camera_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlCamera", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlCamera { ptr })
        }
    }

    fn as_camera_node(self) -> Result<AlCameraNode, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_camera_node_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlCameraNode", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlCameraNode { ptr })
        }
    }

    fn as_cluster(self) -> Result<AlCluster, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_cluster_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlCluster", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlCluster { ptr })
        }
    }

    fn as_cluster_node(self) -> Result<AlClusterNode, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_cluster_node_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlClusterNode",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlClusterNode { ptr })
        }
    }

    fn as_cluster_member(self) -> Result<AlClusterMember, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_cluster_member_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlClusterMember",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlClusterMember { ptr })
        }
    }

    fn as_curve_cv(self) -> Result<AlCurveCV, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_curve_cv_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlCurveCV", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlCurveCV { ptr })
        }
    }

    fn as_curve(self) -> Result<AlCurve, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_curve_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlCurve", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlCurve { ptr })
        }
    }

    fn as_curve_node(self) -> Result<AlCurveNode, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_curve_node_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlCurveNode", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlCurveNode { ptr })
        }
    }

    fn as_curve_on_surface(self) -> Result<AlCurveOnSurface, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_curve_on_surface_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlCurveOnSurface",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlCurveOnSurface { ptr })
        }
    }

    fn as_dag_node(self) -> Result<AlDagNode, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_dag_node_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlDagNode", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlDagNode { ptr })
        }
    }

    fn as_face(self) -> Result<AlFace, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_face_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlFace", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlFace { ptr })
        }
    }

    fn as_face_node(self) -> Result<AlFaceNode, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_face_node_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlFaceNode", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlFaceNode { ptr })
        }
    }

    fn as_group_node(self) -> Result<AlGroupNode, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_group_node_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlGroupNode", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlGroupNode { ptr })
        }
    }

    fn as_light(self) -> Result<AlLight, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_light_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlLight", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlLight { ptr })
        }
    }

    fn as_light_node(self) -> Result<AlLightNode, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_light_node_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlLightNode", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlLightNode { ptr })
        }
    }

    fn as_ambient_light(self) -> Result<AlAmbientLight, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_ambient_light_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlAmbientLight",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlAmbientLight { ptr })
        }
    }

    fn as_area_light(self) -> Result<AlAreaLight, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_area_light_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlAreaLight", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlAreaLight { ptr })
        }
    }

    fn as_direction_light(self) -> Result<AlDirectionLight, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_direction_light_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlDirectionLight",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlDirectionLight { ptr })
        }
    }

    fn as_linear_light(self) -> Result<AlLinearLight, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_linear_light_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlLinearLight",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlLinearLight { ptr })
        }
    }

    fn as_non_ambient_light(self) -> Result<AlNonAmbientLight, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_non_ambient_light_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlNonAmbientLight",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlNonAmbientLight { ptr })
        }
    }

    fn as_point_light(self) -> Result<AlPointLight, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_point_light_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlPointLight", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlPointLight { ptr })
        }
    }

    fn as_spot_light(self) -> Result<AlSpotLight, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_spot_light_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlSpotLight", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlSpotLight { ptr })
        }
    }

    fn as_volume_light(self) -> Result<AlVolumeLight, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_volume_light_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlVolumeLight",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlVolumeLight { ptr })
        }
    }

    fn as_sphere_light(self) -> Result<AlSphereLight, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_sphere_light_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlSphereLight",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlSphereLight { ptr })
        }
    }

    fn as_cylinder_light(self) -> Result<AlCylinderLight, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_cylinder_light_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlCylinderLight",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlCylinderLight { ptr })
        }
    }

    fn as_box_light(self) -> Result<AlBoxLight, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_box_light_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlBoxLight", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlBoxLight { ptr })
        }
    }

    fn as_cone_light(self) -> Result<AlConeLight, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_cone_light_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlConeLight", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlConeLight { ptr })
        }
    }

    fn as_torus_light(self) -> Result<AlTorusLight, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_torus_light_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlTorusLight", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlTorusLight { ptr })
        }
    }

    fn as_surface_cv(self) -> Result<AlSurfaceCV, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_surface_cv_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlSurfaceCV", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlSurfaceCV { ptr })
        }
    }

    fn as_surface(self) -> Result<AlSurface, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_surface_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlSurface", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlSurface { ptr })
        }
    }

    fn as_surface_node(self) -> Result<AlSurfaceNode, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_surface_node_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlSurfaceNode",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlSurfaceNode { ptr })
        }
    }

    fn as_surface_curve(self) -> Result<AlSurfaceCurve, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_surface_curve_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlSurfaceCurve",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlSurfaceCurve { ptr })
        }
    }

    fn as_set(self) -> Result<AlSet, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_set_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlSet", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlSet { ptr })
        }
    }

    fn as_set_member(self) -> Result<AlSetMember, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_set_member_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlSetMember", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlSetMember { ptr })
        }
    }

    fn as_shader(self) -> Result<AlShader, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_shader_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlShader", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlShader { ptr })
        }
    }

    fn as_switch_shader(self) -> Result<AlSwitchShader, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_switch_shader_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlSwitchShader",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlSwitchShader { ptr })
        }
    }

    fn as_layered_shader(self) -> Result<AlLayeredShader, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_layered_shader_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlLayeredShader",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlLayeredShader { ptr })
        }
    }

    fn as_texture(self) -> Result<AlTexture, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_texture_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlTexture", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlTexture { ptr })
        }
    }

    fn as_environment(self) -> Result<AlEnvironment, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_environment_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlEnvironment",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlEnvironment { ptr })
        }
    }

    fn as_keyframe(self) -> Result<AlKeyframe, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_keyframe_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlKeyframe", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlKeyframe { ptr })
        }
    }

    fn as_channel(self) -> Result<AlChannel, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_channel_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlChannel", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlChannel { ptr })
        }
    }

    fn as_action(self) -> Result<AlAction, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_action_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlAction", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlAction { ptr })
        }
    }

    fn as_param_action(self) -> Result<AlParamAction, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_param_action_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlParamAction",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlParamAction { ptr })
        }
    }

    fn as_motion_action(self) -> Result<AlMotionAction, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_motion_action_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlMotionAction",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlMotionAction { ptr })
        }
    }

    fn as_polyset_vertex(self) -> Result<AlPolysetVertex, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_polyset_vertex_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlPolysetVertex",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlPolysetVertex { ptr })
        }
    }

    fn as_polyset_node(self) -> Result<AlPolysetNode, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_polyset_node_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlPolysetNode",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlPolysetNode { ptr })
        }
    }

    fn as_polygon(self) -> Result<AlPolygon, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_polygon_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlPolygon", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlPolygon { ptr })
        }
    }

    fn as_polyset(self) -> Result<AlPolyset, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_polyset_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlPolyset", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlPolyset { ptr })
        }
    }

    fn as_mesh_node(self) -> Result<AlMeshNode, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_mesh_node_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlMeshNode", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlMeshNode { ptr })
        }
    }

    fn as_mesh(self) -> Result<AlMesh, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_mesh_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlMesh", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlMesh { ptr })
        }
    }

    fn as_attributes(self) -> Result<AlAttributes, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_attributes_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlAttributes", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlAttributes { ptr })
        }
    }

    fn as_arc_attributes(self) -> Result<AlArcAttributes, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_arc_attributes_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlArcAttributes",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlArcAttributes { ptr })
        }
    }

    fn as_line_attributes(self) -> Result<AlLineAttributes, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_line_attributes_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlLineAttributes",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlLineAttributes { ptr })
        }
    }

    fn as_curve_attributes(self) -> Result<AlCurveAttributes, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_curve_attributes_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlCurveAttributes",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlCurveAttributes { ptr })
        }
    }

    fn as_plane_attributes(self) -> Result<AlPlaneAttributes, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_plane_attributes_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlPlaneAttributes",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlPlaneAttributes { ptr })
        }
    }

    fn as_conic_attributes(self) -> Result<AlConicAttributes, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_conic_attributes_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlConicAttributes",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlConicAttributes { ptr })
        }
    }

    fn as_rev_surf_attributes(self) -> Result<AlRevSurfAttributes, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_rev_surf_attributes_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlRevSurfAttributes",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlRevSurfAttributes { ptr })
        }
    }

    fn as_joint(self) -> Result<AlJoint, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_joint_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlJoint", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlJoint { ptr })
        }
    }

    fn as_constraint(self) -> Result<AlConstraint, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_constraint_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlConstraint", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlConstraint { ptr })
        }
    }

    fn as_point_constraint(self) -> Result<AlPointConstraint, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_point_constraint_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlPointConstraint",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlPointConstraint { ptr })
        }
    }

    fn as_orientation_constraint(self) -> Result<AlOrientationConstraint, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_orientation_constraint_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlOrientationConstraint",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlOrientationConstraint { ptr })
        }
    }

    fn as_aim_constraint(self) -> Result<AlAimConstraint, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_aim_constraint_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlAimConstraint",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlAimConstraint { ptr })
        }
    }

    fn as_texture_node(self) -> Result<AlTextureNode, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_texture_node_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlTextureNode",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlTextureNode { ptr })
        }
    }

    fn as_shell_node(self) -> Result<AlShellNode, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_shell_node_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlShellNode", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlShellNode { ptr })
        }
    }

    fn as_shell(self) -> Result<AlShell, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_shell_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlShell", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlShell { ptr })
        }
    }

    fn as_trim_region(self) -> Result<AlTrimRegion, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_trim_region_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlTrimRegion", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlTrimRegion { ptr })
        }
    }

    fn as_trim_boundary(self) -> Result<AlTrimBoundary, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_trim_boundary_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlTrimBoundary",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlTrimBoundary { ptr })
        }
    }

    fn as_trim_curve(self) -> Result<AlTrimCurve, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_trim_curve_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlTrimCurve", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlTrimCurve { ptr })
        }
    }

    fn as_contact(self) -> Result<AlContact, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_contact_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlContact", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlContact { ptr })
        }
    }

    fn as_command_ref(self) -> Result<AlCommandRef, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_command_ref_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlCommandRef", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlCommandRef { ptr })
        }
    }

    fn as_command(self) -> Result<AlCommand, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_command_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlCommand", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlCommand { ptr })
        }
    }

    fn as_layer(self) -> Result<AlLayer, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_layer_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlLayer", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlLayer { ptr })
        }
    }

    fn as_orthographic_camera(self) -> Result<AlOrthographicCamera, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_orthographic_camera_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlOrthographicCamera",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlOrthographicCamera { ptr })
        }
    }

    fn as_perspective_camera(self) -> Result<AlPerspectiveCamera, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_perspective_camera_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlPerspectiveCamera",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlPerspectiveCamera { ptr })
        }
    }

    fn as_window(self) -> Result<AlWindow, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_window_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlWindow", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlWindow { ptr })
        }
    }

    fn as_ik_handle(self) -> Result<AlIKHandle, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_ik_handle_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlIKHandle", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlIKHandle { ptr })
        }
    }

    fn as_ik_handle_node(self) -> Result<AlIKHandleNode, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_ik_handle_node_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlIKHandleNode",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlIKHandleNode { ptr })
        }
    }

    fn as_locator(self) -> Result<AlLocator, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_locator_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlLocator", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlLocator { ptr })
        }
    }

    fn as_annotation_locator(self) -> Result<AlAnnotationLocator, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_annotation_locator_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlAnnotationLocator",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlAnnotationLocator { ptr })
        }
    }

    fn as_distance_locator(self) -> Result<AlDistanceLocator, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_distance_locator_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlDistanceLocator",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlDistanceLocator { ptr })
        }
    }

    fn as_angle_locator(self) -> Result<AlAngleLocator, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_angle_locator_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlAngleLocator",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlAngleLocator { ptr })
        }
    }

    fn as_radial_locator(self) -> Result<AlRadialLocator, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_radial_locator_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlRadialLocator",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlRadialLocator { ptr })
        }
    }

    fn as_deviation_locator(self) -> Result<AlDeviationLocator, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_deviation_locator_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlDeviationLocator",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlDeviationLocator { ptr })
        }
    }

    fn as_minmax_locator(self) -> Result<AlMinmaxLocator, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_minmax_locator_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlMinmaxLocator",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlMinmaxLocator { ptr })
        }
    }

    fn as_construction_entity(self) -> Result<AlConstructionEntity, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_construction_entity_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlConstructionEntity",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlConstructionEntity { ptr })
        }
    }

    fn as_construction_vector(self) -> Result<AlConstructionVector, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_construction_vector_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlConstructionVector",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlConstructionVector { ptr })
        }
    }

    fn as_construction_plane(self) -> Result<AlConstructionPlane, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_construction_plane_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlConstructionPlane",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlConstructionPlane { ptr })
        }
    }

    fn as_point(self) -> Result<AlPoint, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_point_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlPoint", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlPoint { ptr })
        }
    }

    fn as_space_point(self) -> Result<AlSpacePoint, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_space_point_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlSpacePoint", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlSpacePoint { ptr })
        }
    }

    fn as_curve_point(self) -> Result<AlCurvePoint, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_curve_point_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlCurvePoint", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlCurvePoint { ptr })
        }
    }

    fn as_curve_on_surface_point(self) -> Result<AlCurveOnSurfacePoint, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_curve_on_surface_point_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlCurveOnSurfacePoint",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlCurveOnSurfacePoint { ptr })
        }
    }

    fn as_surface_point(self) -> Result<AlSurfacePoint, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_surface_point_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlSurfacePoint",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlSurfacePoint { ptr })
        }
    }

    fn as_cloud(self) -> Result<AlCloud, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_cloud_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlCloud", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlCloud { ptr })
        }
    }

    fn as_blend_curve(self) -> Result<AlBlendCurve, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_blend_curve_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlBlendCurve", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlBlendCurve { ptr })
        }
    }

    fn as_blend_point(self) -> Result<AlBlendPoint, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_blend_point_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlBlendPoint", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlBlendPoint { ptr })
        }
    }

    fn as_canvas(self) -> Result<AlCanvas, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_canvas_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlCanvas", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlCanvas { ptr })
        }
    }

    fn as_category(self) -> Result<AlCategory, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_category_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlCategory", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlCategory { ptr })
        }
    }

    fn as_subdiv(self) -> Result<AlSubdiv, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_subdiv_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!("Cannot convert {:?} to AlSubdiv", self.type_()))
        } else {
            std::mem::forget(self);
            Ok(AlSubdiv { ptr })
        }
    }

    fn as_tool_meta_data(self) -> Result<AlToolMetaData, String>
    where
        Self: Sized,
    {
        let ptr = unsafe { alobject_as_tool_meta_data_ptr(self.as_object_ptr()) };
        if ptr.is_null() {
            Err(format!(
                "Cannot convert {:?} to AlToolMetaData",
                self.type_()
            ))
        } else {
            std::mem::forget(self);
            Ok(AlToolMetaData { ptr })
        }
    }
}
impl Drop for AlObject {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl AlObjectMethods for AlObject {
    fn as_object_ptr(&self) -> *mut AlObject_ptr {
        self.ptr
    }
}

unsafe extern "C" {
    fn alobject_is_valid(obj: *mut AlObject_ptr) -> bool;
    fn alobject_are_equal(obj1: *mut AlObject_ptr, obj2: *mut AlObject_ptr) -> bool;
    fn alobject_type(obj: *mut AlObject_ptr) -> i32;
    fn alobject_name(obj: *mut AlObject_ptr) -> *const i8;
    fn alobject_set_name(obj: *mut AlObject_ptr, name: *const i8) -> statusCode;
    fn alobject_delete_object(obj: *mut AlObject_ptr) -> statusCode;
    fn alobject_copy_wrapper(obj: *mut AlObject_ptr) -> *mut AlObject_ptr;
    fn alobject_destroy(obj: *mut AlObject_ptr);

    fn alobject_as_animatable_ptr(obj: *mut AlObject_ptr) -> *mut AlAnimatable_ptr;
    fn alobject_as_clusterable_ptr(obj: *mut AlObject_ptr) -> *mut AlClusterable_ptr;
    fn alobject_as_settable_ptr(obj: *mut AlObject_ptr) -> *mut AlSettable_ptr;
    fn alobject_as_pickable_ptr(obj: *mut AlObject_ptr) -> *mut AlPickable_ptr;

    fn alobject_as_camera_ptr(obj: *mut AlObject_ptr) -> *mut AlCamera_ptr;
    fn alobject_as_camera_node_ptr(obj: *mut AlObject_ptr) -> *mut AlCameraNode_ptr;

    fn alobject_as_cluster_ptr(obj: *mut AlObject_ptr) -> *mut AlCluster_ptr;
    fn alobject_as_cluster_node_ptr(obj: *mut AlObject_ptr) -> *mut AlClusterNode_ptr;
    fn alobject_as_cluster_member_ptr(obj: *mut AlObject_ptr) -> *mut AlClusterMember_ptr;

    fn alobject_as_curve_cv_ptr(obj: *mut AlObject_ptr) -> *mut AlCurveCV_ptr;
    fn alobject_as_curve_ptr(obj: *mut AlObject_ptr) -> *mut AlCurve_ptr;
    fn alobject_as_curve_node_ptr(obj: *mut AlObject_ptr) -> *mut AlCurveNode_ptr;
    fn alobject_as_curve_on_surface_ptr(obj: *mut AlObject_ptr) -> *mut AlCurveOnSurface_ptr;

    fn alobject_as_dag_node_ptr(obj: *mut AlObject_ptr) -> *mut AlDagNode_ptr;

    fn alobject_as_face_ptr(obj: *mut AlObject_ptr) -> *mut AlFace_ptr;
    fn alobject_as_face_node_ptr(obj: *mut AlObject_ptr) -> *mut AlFaceNode_ptr;

    fn alobject_as_group_node_ptr(obj: *mut AlObject_ptr) -> *mut AlGroupNode_ptr;

    fn alobject_as_light_ptr(obj: *mut AlObject_ptr) -> *mut AlLight_ptr;
    fn alobject_as_light_node_ptr(obj: *mut AlObject_ptr) -> *mut AlLightNode_ptr;
    fn alobject_as_ambient_light_ptr(obj: *mut AlObject_ptr) -> *mut AlAmbientLight_ptr;
    fn alobject_as_area_light_ptr(obj: *mut AlObject_ptr) -> *mut AlAreaLight_ptr;
    fn alobject_as_direction_light_ptr(obj: *mut AlObject_ptr) -> *mut AlDirectionLight_ptr;
    fn alobject_as_linear_light_ptr(obj: *mut AlObject_ptr) -> *mut AlLinearLight_ptr;
    fn alobject_as_non_ambient_light_ptr(obj: *mut AlObject_ptr) -> *mut AlNonAmbientLight_ptr;
    fn alobject_as_point_light_ptr(obj: *mut AlObject_ptr) -> *mut AlPointLight_ptr;
    fn alobject_as_spot_light_ptr(obj: *mut AlObject_ptr) -> *mut AlSpotLight_ptr;
    fn alobject_as_volume_light_ptr(obj: *mut AlObject_ptr) -> *mut AlVolumeLight_ptr;
    fn alobject_as_sphere_light_ptr(obj: *mut AlObject_ptr) -> *mut AlSphereLight_ptr;
    fn alobject_as_cylinder_light_ptr(obj: *mut AlObject_ptr) -> *mut AlCylinderLight_ptr;
    fn alobject_as_box_light_ptr(obj: *mut AlObject_ptr) -> *mut AlBoxLight_ptr;
    fn alobject_as_cone_light_ptr(obj: *mut AlObject_ptr) -> *mut AlConeLight_ptr;
    fn alobject_as_torus_light_ptr(obj: *mut AlObject_ptr) -> *mut AlTorusLight_ptr;

    fn alobject_as_surface_cv_ptr(obj: *mut AlObject_ptr) -> *mut AlSurfaceCV_ptr;
    fn alobject_as_surface_ptr(obj: *mut AlObject_ptr) -> *mut AlSurface_ptr;
    fn alobject_as_surface_node_ptr(obj: *mut AlObject_ptr) -> *mut AlSurfaceNode_ptr;
    fn alobject_as_surface_curve_ptr(obj: *mut AlObject_ptr) -> *mut AlSurfaceCurve_ptr;

    fn alobject_as_set_ptr(obj: *mut AlObject_ptr) -> *mut AlSet_ptr;
    fn alobject_as_set_member_ptr(obj: *mut AlObject_ptr) -> *mut AlSetMember_ptr;

    fn alobject_as_shader_ptr(obj: *mut AlObject_ptr) -> *mut AlShader_ptr;
    fn alobject_as_switch_shader_ptr(obj: *mut AlObject_ptr) -> *mut AlSwitchShader_ptr;
    fn alobject_as_layered_shader_ptr(obj: *mut AlObject_ptr) -> *mut AlLayeredShader_ptr;
    fn alobject_as_texture_ptr(obj: *mut AlObject_ptr) -> *mut AlTexture_ptr;
    fn alobject_as_environment_ptr(obj: *mut AlObject_ptr) -> *mut AlEnvironment_ptr;

    fn alobject_as_keyframe_ptr(obj: *mut AlObject_ptr) -> *mut AlKeyframe_ptr;
    fn alobject_as_channel_ptr(obj: *mut AlObject_ptr) -> *mut AlChannel_ptr;
    fn alobject_as_action_ptr(obj: *mut AlObject_ptr) -> *mut AlAction_ptr;
    fn alobject_as_param_action_ptr(obj: *mut AlObject_ptr) -> *mut AlParamAction_ptr;
    fn alobject_as_motion_action_ptr(obj: *mut AlObject_ptr) -> *mut AlMotionAction_ptr;

    fn alobject_as_polyset_vertex_ptr(obj: *mut AlObject_ptr) -> *mut AlPolysetVertex_ptr;
    fn alobject_as_polyset_node_ptr(obj: *mut AlObject_ptr) -> *mut AlPolysetNode_ptr;
    fn alobject_as_polygon_ptr(obj: *mut AlObject_ptr) -> *mut AlPolygon_ptr;
    fn alobject_as_polyset_ptr(obj: *mut AlObject_ptr) -> *mut AlPolyset_ptr;

    fn alobject_as_mesh_node_ptr(obj: *mut AlObject_ptr) -> *mut AlMeshNode_ptr;
    fn alobject_as_mesh_ptr(obj: *mut AlObject_ptr) -> *mut AlMesh_ptr;

    fn alobject_as_attributes_ptr(obj: *mut AlObject_ptr) -> *mut AlAttributes_ptr;
    fn alobject_as_arc_attributes_ptr(obj: *mut AlObject_ptr) -> *mut AlArcAttributes_ptr;
    fn alobject_as_line_attributes_ptr(obj: *mut AlObject_ptr) -> *mut AlLineAttributes_ptr;
    fn alobject_as_curve_attributes_ptr(obj: *mut AlObject_ptr) -> *mut AlCurveAttributes_ptr;
    fn alobject_as_plane_attributes_ptr(obj: *mut AlObject_ptr) -> *mut AlPlaneAttributes_ptr;
    fn alobject_as_conic_attributes_ptr(obj: *mut AlObject_ptr) -> *mut AlConicAttributes_ptr;
    fn alobject_as_rev_surf_attributes_ptr(obj: *mut AlObject_ptr) -> *mut AlRevSurfAttributes_ptr;

    fn alobject_as_joint_ptr(obj: *mut AlObject_ptr) -> *mut AlJoint_ptr;
    fn alobject_as_constraint_ptr(obj: *mut AlObject_ptr) -> *mut AlConstraint_ptr;
    fn alobject_as_point_constraint_ptr(obj: *mut AlObject_ptr) -> *mut AlPointConstraint_ptr;
    fn alobject_as_orientation_constraint_ptr(
        obj: *mut AlObject_ptr,
    ) -> *mut AlOrientationConstraint_ptr;
    fn alobject_as_aim_constraint_ptr(obj: *mut AlObject_ptr) -> *mut AlAimConstraint_ptr;

    fn alobject_as_texture_node_ptr(obj: *mut AlObject_ptr) -> *mut AlTextureNode_ptr;

    fn alobject_as_shell_node_ptr(obj: *mut AlObject_ptr) -> *mut AlShellNode_ptr;
    fn alobject_as_shell_ptr(obj: *mut AlObject_ptr) -> *mut AlShell_ptr;

    fn alobject_as_trim_region_ptr(obj: *mut AlObject_ptr) -> *mut AlTrimRegion_ptr;
    fn alobject_as_trim_boundary_ptr(obj: *mut AlObject_ptr) -> *mut AlTrimBoundary_ptr;
    fn alobject_as_trim_curve_ptr(obj: *mut AlObject_ptr) -> *mut AlTrimCurve_ptr;

    fn alobject_as_contact_ptr(obj: *mut AlObject_ptr) -> *mut AlContact_ptr;
    fn alobject_as_command_ref_ptr(obj: *mut AlObject_ptr) -> *mut AlCommandRef_ptr;
    fn alobject_as_command_ptr(obj: *mut AlObject_ptr) -> *mut AlCommand_ptr;
    fn alobject_as_layer_ptr(obj: *mut AlObject_ptr) -> *mut AlLayer_ptr;

    fn alobject_as_orthographic_camera_ptr(obj: *mut AlObject_ptr)
    -> *mut AlOrthographicCamera_ptr;
    fn alobject_as_perspective_camera_ptr(obj: *mut AlObject_ptr) -> *mut AlPerspectiveCamera_ptr;
    fn alobject_as_window_ptr(obj: *mut AlObject_ptr) -> *mut AlWindow_ptr;

    fn alobject_as_ik_handle_ptr(obj: *mut AlObject_ptr) -> *mut AlIKHandle_ptr;
    fn alobject_as_ik_handle_node_ptr(obj: *mut AlObject_ptr) -> *mut AlIKHandleNode_ptr;

    fn alobject_as_locator_ptr(obj: *mut AlObject_ptr) -> *mut AlLocator_ptr;
    fn alobject_as_annotation_locator_ptr(obj: *mut AlObject_ptr) -> *mut AlAnnotationLocator_ptr;
    fn alobject_as_distance_locator_ptr(obj: *mut AlObject_ptr) -> *mut AlDistanceLocator_ptr;
    fn alobject_as_angle_locator_ptr(obj: *mut AlObject_ptr) -> *mut AlAngleLocator_ptr;
    fn alobject_as_radial_locator_ptr(obj: *mut AlObject_ptr) -> *mut AlRadialLocator_ptr;
    fn alobject_as_deviation_locator_ptr(obj: *mut AlObject_ptr) -> *mut AlDeviationLocator_ptr;
    fn alobject_as_minmax_locator_ptr(obj: *mut AlObject_ptr) -> *mut AlMinmaxLocator_ptr;

    fn alobject_as_construction_entity_ptr(obj: *mut AlObject_ptr)
    -> *mut AlConstructionEntity_ptr;
    fn alobject_as_construction_vector_ptr(obj: *mut AlObject_ptr)
    -> *mut AlConstructionVector_ptr;
    fn alobject_as_construction_plane_ptr(obj: *mut AlObject_ptr) -> *mut AlConstructionPlane_ptr;
    fn alobject_as_point_ptr(obj: *mut AlObject_ptr) -> *mut AlPoint_ptr;
    fn alobject_as_space_point_ptr(obj: *mut AlObject_ptr) -> *mut AlSpacePoint_ptr;
    fn alobject_as_curve_point_ptr(obj: *mut AlObject_ptr) -> *mut AlCurvePoint_ptr;
    fn alobject_as_curve_on_surface_point_ptr(
        obj: *mut AlObject_ptr,
    ) -> *mut AlCurveOnSurfacePoint_ptr;
    fn alobject_as_surface_point_ptr(obj: *mut AlObject_ptr) -> *mut AlSurfacePoint_ptr;

    fn alobject_as_cloud_ptr(obj: *mut AlObject_ptr) -> *mut AlCloud_ptr;

    fn alobject_as_blend_curve_ptr(obj: *mut AlObject_ptr) -> *mut AlBlendCurve_ptr;
    fn alobject_as_blend_point_ptr(obj: *mut AlObject_ptr) -> *mut AlBlendPoint_ptr;
    fn alobject_as_canvas_ptr(obj: *mut AlObject_ptr) -> *mut AlCanvas_ptr;
    fn alobject_as_category_ptr(obj: *mut AlObject_ptr) -> *mut AlCategory_ptr;

    fn alobject_as_subdiv_ptr(obj: *mut AlObject_ptr) -> *mut AlSubdiv_ptr;
    fn alobject_as_tool_meta_data_ptr(obj: *mut AlObject_ptr) -> *mut AlToolMetaData_ptr;
}
