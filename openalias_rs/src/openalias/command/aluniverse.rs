#![allow(non_camel_case_types)]
use crate::*;
use enigo::{Enigo, Key, KeyboardControllable};

//#[repr(C)]
//struct AlUniverse_ptr {
//    _private: [u8; 0],
//}

#[derive(Debug)]
pub struct AlUniverse {
    //ptr: *mut AlUniverse_ptr,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum AlCoordinateSystem {
    kRightHanded = 0,
    kLeftHanded = 1,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum AlDirectoryType {
    kWireDir,
    kTextureDir,
    kShaderDir,
    kStartupDir,
    kHelpDir,
    kTmpDir,
    kFontDir,
    kPluginDir,
    kImageDir,
    kPaletteDir,
    kProjectDir,
    kWireFileDir,
    kRenderDir,
    kConstructionDir,
    kExternalDataDir,
    kScriptDir,
    kClipDir,
    kSoundDir,
    kExchangeDir,
    kFormExplorerDir,
    kNavDesignDir,
    kPluginConductorDir,
    kMaxDirType,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum AlFileType {
    kUnknownFile = 0,
    kWFile = 1,
    kSdFile = 2,
    kObjFile = 3,
    kStlFile = 4,
    kDxfFile = 5,
    kIgesFile = 6,
    kStepFile = 7,
    kIgmFile = 8,
    kSmfFile = 9,
    kCloudFile = 10,
    kSubdivFile = 11,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum AlFrameRangeType {
    kFrameRangePlayback,
    kFrameRangeRender,
    kFrameRangeAnimation,
    kFrameRangeNone,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum AlApplicationMode {
    kNormalMode,
    kScriptMode,
    kBatchMode,
}

impl AlUniverse {
    pub fn initialize(
        coord_system: AlCoordinateSystem,
        init_project_env: bool,
    ) -> Result<(), String> {
        let status = unsafe { aluniverse_initialize(coord_system, init_project_env) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn is_initialized() -> bool {
        unsafe { aluniverse_is_initialized() }
    }

    pub fn coordinate_system() -> AlCoordinateSystem {
        unsafe { aluniverse_coordinate_system() }
    }

    pub fn expand_file_name(file_name: &str, directory_type: AlDirectoryType) -> Option<String> {
        let mut out_file_name = [0i8; 1024];
        let c_file_name = std::ffi::CString::new(file_name).ok()?;
        let status = unsafe {
            aluniverse_expand_file_name(
                out_file_name.as_mut_ptr(),
                c_file_name.as_ptr(),
                directory_type,
            )
        };
        if status == statusCode::Success {
            Some(unsafe {
                std::ffi::CStr::from_ptr(out_file_name.as_ptr())
                    .to_string_lossy()
                    .to_string()
            })
        } else {
            None
        }
    }

    pub fn is_wire_file(file_name: &str) -> (bool, String) {
        let mut file_version = [0i8; 8];
        let c_file_name = std::ffi::CString::new(file_name).unwrap_or_default();
        let result =
            unsafe { aluniverse_is_wire_file(c_file_name.as_ptr(), file_version.as_mut_ptr()) };
        let version = unsafe {
            std::ffi::CStr::from_ptr(file_version.as_ptr())
                .to_string_lossy()
                .to_string()
        };
        (result, version)
    }

    pub fn wire_file_window_size() -> Result<(i32, i32), String> {
        let mut size_x: i32 = 0;
        let mut size_y: i32 = 0;
        let status = unsafe { aluniverse_wire_file_window_size(&mut size_x, &mut size_y) };
        if status == statusCode::Success {
            Ok((size_x, size_y))
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_wire_file_window_size(size_x: i32, size_y: i32) -> Result<(), String> {
        let status = unsafe { aluniverse_set_wire_file_window_size(size_x, size_y) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn file_type(file_name: &str) -> AlFileType {
        let c_file_name = std::ffi::CString::new(file_name).unwrap_or_default();
        unsafe { aluniverse_file_type(c_file_name.as_ptr()) }
    }

    pub fn file_type_w(file_name: &str) -> AlFileType {
        let wide: Vec<u16> = file_name.encode_utf16().chain(std::iter::once(0)).collect();
        unsafe { aluniverse_file_type_w(wide.as_ptr()) }
    }

    pub fn retrieve(file_name: &str) -> Result<(), String> {
        let c_file_name =
            std::ffi::CString::new(file_name).map_err(|_| "Invalid file name".to_string())?;
        let status = unsafe { aluniverse_retrieve(c_file_name.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn retrieve_with_progress(
        file_name: &str,
        progress_callback: Option<extern "C" fn(i32)>,
    ) -> Result<(), String> {
        let c_file_name =
            std::ffi::CString::new(file_name).map_err(|_| "Invalid file name".to_string())?;
        let callback = match progress_callback {
            Some(cb) => cb,
            None => unsafe { std::mem::transmute(std::ptr::null::<extern "C" fn(i32)>()) },
        };
        let status = unsafe { aluniverse_retrieve_with_progress(c_file_name.as_ptr(), callback) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn last_wire_file_retrieved() -> Option<String> {
        let c_str = unsafe { aluniverse_last_wire_file_retrieved() };
        if c_str.is_null() {
            None
        } else {
            Some(unsafe {
                std::ffi::CStr::from_ptr(c_str)
                    .to_string_lossy()
                    .to_string()
            })
        }
    }

    pub fn last_file_retrieved() -> Option<String> {
        let c_str = unsafe { aluniverse_last_file_retrieved() };
        if c_str.is_null() {
            None
        } else {
            Some(unsafe {
                std::ffi::CStr::from_ptr(c_str)
                    .to_string_lossy()
                    .to_string()
            })
        }
    }

    pub fn last_file_saved() -> Option<String> {
        let c_str = unsafe { aluniverse_last_file_saved() };
        if c_str.is_null() {
            None
        } else {
            Some(unsafe {
                std::ffi::CStr::from_ptr(c_str)
                    .to_string_lossy()
                    .to_string()
            })
        }
    }

    pub fn store(file_name: &str, dag_node: Option<&AlDagNode>) -> Result<(), String> {
        let c_file_name =
            std::ffi::CString::new(file_name).map_err(|_| "Invalid file name".to_string())?;
        let dag_node_ptr = dag_node.map_or(std::ptr::null_mut(), |node| node.ptr);
        let status = unsafe { aluniverse_store(c_file_name.as_ptr(), dag_node_ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn store_ex(
        file_name: &str,
        dag_node: &AlDagNode,
        embed_image_references: bool,
        include_installed_images: bool,
        file_type: AlFileType,
    ) -> Result<(), String> {
        let c_file_name =
            std::ffi::CString::new(file_name).map_err(|_| "Invalid file name".to_string())?;
        let status = unsafe {
            aluniverse_store_ex(
                c_file_name.as_ptr(),
                dag_node.ptr,
                embed_image_references,
                include_installed_images,
                file_type,
            )
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn store_active(file_name: &str, file_type: AlFileType) -> Result<(), String> {
        let c_file_name =
            std::ffi::CString::new(file_name).map_err(|_| "Invalid file name".to_string())?;
        let status = unsafe { aluniverse_store_active(c_file_name.as_ptr(), file_type) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn first_dag_node() -> Option<AlDagNode> {
        let ptr = unsafe { aluniverse_first_dag_node() };
        if ptr.is_null() {
            None
        } else {
            Some(AlDagNode { ptr })
        }
    }
    pub fn dag_nodes() -> impl Iterator<Item = AlDagNode> {
        std::iter::successors(AlUniverse::first_dag_node(), |prev| prev.next_node())
    }

    pub fn first_set() -> Option<AlSet> {
        let ptr = unsafe { aluniverse_first_set() };
        if ptr.is_null() {
            None
        } else {
            Some(AlSet { ptr })
        }
    }

    pub fn first_cluster() -> Option<AlCluster> {
        let ptr = unsafe { aluniverse_first_cluster() };
        if ptr.is_null() {
            None
        } else {
            Some(AlCluster { ptr })
        }
    }

    pub fn current_stage() -> Option<String> {
        let c_str = unsafe { aluniverse_current_stage() };
        if c_str.is_null() {
            None
        } else {
            Some(unsafe {
                std::ffi::CStr::from_ptr(c_str)
                    .to_string_lossy()
                    .to_string()
            })
        }
    }

    pub fn set_current_stage(stage_name: &str) -> Result<(), String> {
        let c_stage_name =
            std::ffi::CString::new(stage_name).map_err(|_| "Invalid stage name".to_string())?;
        let status = unsafe { aluniverse_set_current_stage(c_stage_name.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn merge_stage(stage_name: &str) -> Result<(), String> {
        let c_stage_name =
            std::ffi::CString::new(stage_name).map_err(|_| "Invalid stage name".to_string())?;
        let status = unsafe { aluniverse_merge_stage(c_stage_name.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn merge_all_stages() -> Result<(), String> {
        let status = unsafe { aluniverse_merge_all_stages() };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn delete_stage(stage_name: &str) -> Result<(), String> {
        let c_stage_name =
            std::ffi::CString::new(stage_name).map_err(|_| "Invalid stage name".to_string())?;
        let status = unsafe { aluniverse_delete_stage(c_stage_name.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn delete_all_stages() -> Result<(), String> {
        let status = unsafe { aluniverse_delete_all_stages() };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn stage_visibility(stage_name: &str) -> Result<bool, String> {
        let mut visibility: bool = false;
        let c_stage_name =
            std::ffi::CString::new(stage_name).map_err(|_| "Invalid stage name".to_string())?;
        let status = unsafe { aluniverse_stage_visibility(c_stage_name.as_ptr(), &mut visibility) };
        if status == statusCode::Success {
            Ok(visibility)
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_stage_visibility(stage_name: &str, visibility: bool) -> Result<(), String> {
        let c_stage_name =
            std::ffi::CString::new(stage_name).map_err(|_| "Invalid stage name".to_string())?;
        let status = unsafe { aluniverse_set_stage_visibility(c_stage_name.as_ptr(), visibility) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn stage_window_source() -> Option<String> {
        let c_str = unsafe { aluniverse_stage_window_source() };
        if c_str.is_null() {
            None
        } else {
            Some(unsafe {
                std::ffi::CStr::from_ptr(c_str)
                    .to_string_lossy()
                    .to_string()
            })
        }
    }

    pub fn set_stage_window_source(source: &str) -> Result<(), String> {
        let c_source = std::ffi::CString::new(source).map_err(|_| "Invalid source".to_string())?;
        let status = unsafe { aluniverse_set_stage_window_source(c_source.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn stage_background_source() -> Option<String> {
        let c_str = unsafe { aluniverse_stage_background_source() };
        if c_str.is_null() {
            None
        } else {
            Some(unsafe {
                std::ffi::CStr::from_ptr(c_str)
                    .to_string_lossy()
                    .to_string()
            })
        }
    }

    pub fn set_stage_background_source(source: &str) -> Result<(), String> {
        let c_source = std::ffi::CString::new(source).map_err(|_| "Invalid source".to_string())?;
        let status = unsafe { aluniverse_set_stage_background_source(c_source.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn rename_stage(old_name: &str, new_name: &str) -> Result<(), String> {
        let c_old_name =
            std::ffi::CString::new(old_name).map_err(|_| "Invalid old name".to_string())?;
        let c_new_name =
            std::ffi::CString::new(new_name).map_err(|_| "Invalid new name".to_string())?;
        let status = unsafe { aluniverse_rename_stage(c_old_name.as_ptr(), c_new_name.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn stage_names() -> Option<AlList> {
        let ptr = unsafe { aluniverse_stage_names() };
        if ptr.is_null() {
            None
        } else {
            Some(AlList { ptr })
        }
    }

    pub fn create_new_stage(stage_name: &str) -> Result<String, String> {
        let mut out_new_name: *mut i8 = std::ptr::null_mut();
        let c_stage_name =
            std::ffi::CString::new(stage_name).map_err(|_| "Invalid stage name".to_string())?;
        let status =
            unsafe { aluniverse_create_new_stage(c_stage_name.as_ptr(), &mut out_new_name) };
        if status == statusCode::Success {
            let name = unsafe {
                std::ffi::CStr::from_ptr(out_new_name)
                    .to_string_lossy()
                    .to_string()
            };
            Ok(name)
        } else {
            Err(status.to_string())
        }
    }

    pub fn retrieve_stage_set(file_name: &str, flag: bool) -> Result<(), String> {
        let c_file_name =
            std::ffi::CString::new(file_name).map_err(|_| "Invalid file name".to_string())?;
        let status = unsafe { aluniverse_retrieve_stage_set(c_file_name.as_ptr(), flag) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn save_stage_set(file_name: &str) -> Result<(), String> {
        let c_file_name =
            std::ffi::CString::new(file_name).map_err(|_| "Invalid file name".to_string())?;
        let status = unsafe { aluniverse_save_stage_set(c_file_name.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_stage_wire_file_name(stage_name: &str, file_name: &str) -> Result<(), String> {
        let c_stage_name =
            std::ffi::CString::new(stage_name).map_err(|_| "Invalid stage name".to_string())?;
        let c_file_name =
            std::ffi::CString::new(file_name).map_err(|_| "Invalid file name".to_string())?;
        let status = unsafe {
            aluniverse_set_stage_wire_file_name(c_stage_name.as_ptr(), c_file_name.as_ptr())
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn stage_wire_file_name(stage_name: &str) -> Option<String> {
        let c_stage_name = std::ffi::CString::new(stage_name).unwrap_or_default();
        let c_str = unsafe { aluniverse_stage_wire_file_name(c_stage_name.as_ptr()) };
        if c_str.is_null() {
            None
        } else {
            Some(unsafe {
                std::ffi::CStr::from_ptr(c_str)
                    .to_string_lossy()
                    .to_string()
            })
        }
    }

    pub fn first_window() -> Option<AlWindow> {
        let ptr = unsafe { aluniverse_first_window() };
        if ptr.is_null() {
            None
        } else {
            Some(AlWindow { ptr })
        }
    }

    pub fn current_window() -> Option<AlWindow> {
        let ptr = unsafe { aluniverse_current_window() };
        if ptr.is_null() {
            None
        } else {
            Some(AlWindow { ptr })
        }
    }

    pub fn sbd_window() -> Option<AlWindow> {
        let ptr = unsafe { aluniverse_sbd_window() };
        if ptr.is_null() {
            None
        } else {
            Some(AlWindow { ptr })
        }
    }

    pub fn blind_data(data_id: i32) -> Result<(i64, String), String> {
        let mut out_value: i64 = 0;
        let mut out_string: *const i8 = std::ptr::null();
        let status = unsafe { aluniverse_blind_data(data_id, &mut out_value, &mut out_string) };
        if status == statusCode::Success {
            let s = if out_string.is_null() {
                String::new()
            } else {
                unsafe {
                    std::ffi::CStr::from_ptr(out_string)
                        .to_string_lossy()
                        .to_string()
                }
            };
            Ok((out_value, s))
        } else {
            Err(status.to_string())
        }
    }

    pub fn blind_data_ex(data_id: i32, sub_id: i32) -> Result<(i64, String), String> {
        let mut out_value: i64 = 0;
        let mut out_string: *const i8 = std::ptr::null();
        let status =
            unsafe { aluniverse_blind_data_ex(data_id, sub_id, &mut out_value, &mut out_string) };
        if status == statusCode::Success {
            let s = if out_string.is_null() {
                String::new()
            } else {
                unsafe {
                    std::ffi::CStr::from_ptr(out_string)
                        .to_string_lossy()
                        .to_string()
                }
            };
            Ok((out_value, s))
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_blind_data(data_id: i32, value: i64, string: &str) -> Result<(), String> {
        let c_string = std::ffi::CString::new(string).map_err(|_| "Invalid string".to_string())?;
        let status = unsafe { aluniverse_set_blind_data(data_id, value, c_string.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn remove_blind_data(data_id: i32) -> Result<(), String> {
        let status = unsafe { aluniverse_remove_blind_data(data_id) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn remove_blind_data_ex(data_id: i32, sub_id: i32) -> Result<(), String> {
        let status = unsafe { aluniverse_remove_blind_data_ex(data_id, sub_id) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn first_channel() -> Option<AlChannel> {
        let ptr = unsafe { aluniverse_first_channel() };
        if ptr.is_null() {
            None
        } else {
            Some(AlChannel { ptr })
        }
    }

    pub fn next_channel(channel: &AlChannel) -> Option<AlChannel> {
        let ptr = unsafe { aluniverse_next_channel(channel.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlChannel { ptr })
        }
    }

    pub fn next_channel_d(channel: &mut AlChannel) -> Result<(), String> {
        let status = unsafe { aluniverse_next_channel_d(channel.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn first_action() -> Option<AlAction> {
        let ptr = unsafe { aluniverse_first_action() };
        if ptr.is_null() {
            None
        } else {
            Some(AlAction { ptr })
        }
    }

    pub fn next_action(action: &AlAction) -> Option<AlAction> {
        let ptr = unsafe { aluniverse_next_action(action.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlAction { ptr })
        }
    }

    pub fn first_shader() -> Option<AlShader> {
        let ptr = unsafe { aluniverse_first_shader() };
        if ptr.is_null() {
            None
        } else {
            Some(AlShader { ptr })
        }
    }

    pub fn next_shader(shader: &AlShader) -> Option<AlShader> {
        let ptr = unsafe { aluniverse_next_shader(shader.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlShader { ptr })
        }
    }

    pub fn next_shader_d(shader: &mut AlShader) -> Result<(), String> {
        let status = unsafe { aluniverse_next_shader_d(shader.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn first_switch_shader() -> Option<AlSwitchShader> {
        let ptr = unsafe { aluniverse_first_switch_shader() };
        if ptr.is_null() {
            None
        } else {
            Some(AlSwitchShader { ptr })
        }
    }

    pub fn next_switch_shader(shader: &AlSwitchShader) -> Option<AlSwitchShader> {
        let ptr = unsafe { aluniverse_next_switch_shader(shader.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlSwitchShader { ptr })
        }
    }

    pub fn first_layered_shader() -> Option<AlLayeredShader> {
        let ptr = unsafe { aluniverse_first_layered_shader() };
        if ptr.is_null() {
            None
        } else {
            Some(AlLayeredShader { ptr })
        }
    }

    pub fn next_layered_shader(shader: &AlLayeredShader) -> Option<AlLayeredShader> {
        let ptr = unsafe { aluniverse_next_layered_shader(shader.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlLayeredShader { ptr })
        }
    }

    pub fn first_environment() -> Option<AlEnvironment> {
        let ptr = unsafe { aluniverse_first_environment() };
        if ptr.is_null() {
            None
        } else {
            Some(AlEnvironment { ptr })
        }
    }

    pub fn first_inactive_environment() -> Option<AlEnvironment> {
        let ptr = unsafe { aluniverse_first_inactive_environment() };
        if ptr.is_null() {
            None
        } else {
            Some(AlEnvironment { ptr })
        }
    }

    pub fn next_inactive_environment(env: &AlEnvironment) -> Option<AlEnvironment> {
        let ptr = unsafe { aluniverse_next_inactive_environment(env.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlEnvironment { ptr })
        }
    }

    pub fn first_layer() -> Option<AlLayer> {
        let ptr = unsafe { aluniverse_first_layer() };
        if ptr.is_null() {
            None
        } else {
            Some(AlLayer { ptr })
        }
    }

    pub fn next_layer(layer: &AlLayer) -> Option<AlLayer> {
        let ptr = unsafe { aluniverse_next_layer(layer.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlLayer { ptr })
        }
    }

    pub fn next_layer_d(layer: &mut AlLayer) -> Result<(), String> {
        let status = unsafe { aluniverse_next_layer_d(layer.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn layer_by_number(layer_num: i32) -> Option<AlLayer> {
        let ptr = unsafe { aluniverse_layer_by_number(layer_num) };
        if ptr.is_null() {
            None
        } else {
            Some(AlLayer { ptr })
        }
    }

    pub fn layer_by_name(layer_name: &str) -> Result<AlLayer,String> {
        let c_layer_name = std::ffi::CString::new(layer_name).unwrap_or_default();
        let ptr = unsafe { aluniverse_layer_by_name(c_layer_name.as_ptr() as *mut i8) };
        if ptr.is_null() {
            Err("Layer not found".to_string())
        } else {
            Ok(AlLayer { ptr })
        }
    }

    pub fn creation_layer() -> Option<AlLayer> {
        let ptr = unsafe { aluniverse_creation_layer() };
        if ptr.is_null() {
            None
        } else {
            Some(AlLayer { ptr })
        }
    }

    pub fn set_creation_layer(layer: &AlLayer) -> Result<(), String> {
        let status = unsafe { aluniverse_set_creation_layer(layer.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn layers_enabled() -> bool {
        unsafe { aluniverse_layers_enabled() }
    }

    pub fn set_layers_enabled(enabled: bool) -> Result<(), String> {
        let status = unsafe { aluniverse_set_layers_enabled(enabled) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn first_camera() -> Option<AlPerspectiveCamera> {
        let ptr = unsafe { aluniverse_first_camera() };
        if ptr.is_null() {
            None
        } else {
            Some(AlPerspectiveCamera { ptr })
        }
    }

    pub fn next_camera(camera: &AlPerspectiveCamera) -> Option<AlPerspectiveCamera> {
        let ptr = unsafe { aluniverse_next_camera(camera.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlPerspectiveCamera { ptr })
        }
    }

    pub fn next_camera_d(camera: &mut AlPerspectiveCamera) -> Result<(), String> {
        let status = unsafe { aluniverse_next_camera_d(camera.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn first_locator() -> Option<AlLocator> {
        let ptr = unsafe { aluniverse_first_locator() };
        if ptr.is_null() {
            None
        } else {
            Some(AlLocator { ptr })
        }
    }

    pub fn next_locator(locator: &AlLocator) -> Option<AlLocator> {
        let ptr = unsafe { aluniverse_next_locator(locator.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlLocator { ptr })
        }
    }

    pub fn delete_all_locators() -> Result<(), String> {
        let status = unsafe { aluniverse_delete_all_locators() };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn first_construction_entity() -> Option<AlConstructionEntity> {
        let ptr = unsafe { aluniverse_first_construction_entity() };
        if ptr.is_null() {
            None
        } else {
            Some(AlConstructionEntity { ptr })
        }
    }

    pub fn next_construction_entity(entity: &AlConstructionEntity) -> Option<AlConstructionEntity> {
        let ptr = unsafe { aluniverse_next_construction_entity(entity.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlConstructionEntity { ptr })
        }
    }
        pub fn construction_entities() -> impl Iterator<Item = AlConstructionEntity> {
        std::iter::successors(AlUniverse::first_construction_entity(), |prev| {
            AlUniverse::next_construction_entity(prev)
        })
    }


    pub fn delete_all_construction_entities() -> Result<(), String> {
        let status = unsafe { aluniverse_delete_all_construction_entities() };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn import_cloud_file(file_name: &str, flags: i32) -> Result<(), String> {
        let c_file_name =
            std::ffi::CString::new(file_name).map_err(|_| "Invalid file name".to_string())?;
        let status = unsafe { aluniverse_import_cloud_file(c_file_name.as_ptr(), flags) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn first_cloud() -> Option<AlCloud> {
        let ptr = unsafe { aluniverse_first_cloud() };
        if ptr.is_null() {
            None
        } else {
            Some(AlCloud { ptr })
        }
    }

    pub fn number_of_clouds() -> i32 {
        unsafe { aluniverse_number_of_clouds() }
    }

    pub fn import_subdiv(file_name: &str) -> Result<(), String> {
        let c_file_name =
            std::ffi::CString::new(file_name).map_err(|_| "Invalid file name".to_string())?;
        let status = unsafe { aluniverse_import_subdiv(c_file_name.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn first_blend_curve() -> Option<AlBlendCurve> {
        let ptr = unsafe { aluniverse_first_blend_curve() };
        if ptr.is_null() {
            None
        } else {
            Some(AlBlendCurve { ptr })
        }
    }

    pub fn number_of_blend_curves() -> i32 {
        unsafe { aluniverse_number_of_blend_curves() }
    }

    pub fn first_canvas() -> Option<AlCanvas> {
        let ptr = unsafe { aluniverse_first_canvas() };
        if ptr.is_null() {
            None
        } else {
            Some(AlCanvas { ptr })
        }
    }

    pub fn next_canvas(canvas: &AlCanvas) -> Option<AlCanvas> {
        let ptr = unsafe { aluniverse_next_canvas(canvas.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlCanvas { ptr })
        }
    }

    pub fn next_canvas_d(canvas: &mut AlCanvas) -> Result<(), String> {
        let status = unsafe { aluniverse_next_canvas_d(canvas.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn first_category() -> Option<AlCategory> {
        let ptr = unsafe { aluniverse_first_category() };
        if ptr.is_null() {
            None
        } else {
            Some(AlCategory { ptr })
        }
    }

    pub fn next_category(category: &AlCategory) -> Option<AlCategory> {
        let ptr = unsafe { aluniverse_next_category(category.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlCategory { ptr })
        }
    }

    pub fn import_reference_file(file_name: &str) -> Option<AlReferenceFile> {
        let c_file_name = std::ffi::CString::new(file_name).ok()?;
        let ptr = unsafe { aluniverse_import_reference_file(c_file_name.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(AlReferenceFile { ptr })
        }
    }

    pub fn remove_reference_file(ref_file: &AlReferenceFile) -> Result<(), String> {
        let status = unsafe { aluniverse_remove_reference_file(ref_file.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn create_reference_alternative(name: &str) -> Option<AlReferenceFileSet> {
        let c_name = std::ffi::CString::new(name).ok()?;
        let ptr = unsafe { aluniverse_create_reference_alternative(c_name.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(AlReferenceFileSet { ptr })
        }
    }

    pub fn first_reference_file_set() -> Option<AlReferenceFileSet> {
        let ptr = unsafe { aluniverse_first_reference_file_set() };
        if ptr.is_null() {
            None
        } else {
            Some(AlReferenceFileSet { ptr })
        }
    }

    pub fn next_reference_file_set(ref_set: &AlReferenceFileSet) -> Option<AlReferenceFileSet> {
        let ptr = unsafe { aluniverse_next_reference_file_set(ref_set.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlReferenceFileSet { ptr })
        }
    }

    pub fn next_reference_file_set_d(ref_set: &mut AlReferenceFileSet) -> Result<(), String> {
        let status = unsafe { aluniverse_next_reference_file_set_d(ref_set.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn pack_reference(pack_dir: &str) -> Result<(), String> {
        let c_pack_dir =
            std::ffi::CString::new(pack_dir).map_err(|_| "Invalid pack dir".to_string())?;
        let status = unsafe { aluniverse_pack_reference(c_pack_dir.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn redraw_screen() -> Result<(), String> {
        let status = unsafe { aluniverse_redraw_screen() };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn apply_iterator_to_windows(iterator: &mut AlIterator) -> Result<i32, String> {
        let mut count: i32 = 0;
        let status = unsafe { aluniverse_apply_iterator_to_windows(iterator.ptr, &mut count) };
        if status == statusCode::Success {
            Ok(count)
        } else {
            Err(status.to_string())
        }
    }

    pub fn apply_iterator_to_dag_nodes(iterator: &mut AlIterator) -> Result<i32, String> {
        let mut count: i32 = 0;
        let status = unsafe { aluniverse_apply_iterator_to_dag_nodes(iterator.ptr, &mut count) };
        if status == statusCode::Success {
            Ok(count)
        } else {
            Err(status.to_string())
        }
    }

    pub fn apply_iterator_to_actions(iterator: &mut AlIterator) -> Result<i32, String> {
        let mut count: i32 = 0;
        let status = unsafe { aluniverse_apply_iterator_to_actions(iterator.ptr, &mut count) };
        if status == statusCode::Success {
            Ok(count)
        } else {
            Err(status.to_string())
        }
    }

    pub fn apply_iterator_to_channels(iterator: &mut AlIterator) -> Result<i32, String> {
        let mut count: i32 = 0;
        let status = unsafe { aluniverse_apply_iterator_to_channels(iterator.ptr, &mut count) };
        if status == statusCode::Success {
            Ok(count)
        } else {
            Err(status.to_string())
        }
    }

    pub fn apply_iterator_to_sets(iterator: &mut AlIterator) -> Result<i32, String> {
        let mut count: i32 = 0;
        let status = unsafe { aluniverse_apply_iterator_to_sets(iterator.ptr, &mut count) };
        if status == statusCode::Success {
            Ok(count)
        } else {
            Err(status.to_string())
        }
    }

    pub fn apply_iterator_to_clusters(iterator: &mut AlIterator) -> Result<i32, String> {
        let mut count: i32 = 0;
        let status = unsafe { aluniverse_apply_iterator_to_clusters(iterator.ptr, &mut count) };
        if status == statusCode::Success {
            Ok(count)
        } else {
            Err(status.to_string())
        }
    }

    pub fn apply_iterator_to_shaders(iterator: &mut AlIterator) -> Result<i32, String> {
        let mut count: i32 = 0;
        let status = unsafe { aluniverse_apply_iterator_to_shaders(iterator.ptr, &mut count) };
        if status == statusCode::Success {
            Ok(count)
        } else {
            Err(status.to_string())
        }
    }

    pub fn apply_iterator_to_locators(iterator: &mut AlIterator) -> Result<i32, String> {
        let mut count: i32 = 0;
        let status = unsafe { aluniverse_apply_iterator_to_locators(iterator.ptr, &mut count) };
        if status == statusCode::Success {
            Ok(count)
        } else {
            Err(status.to_string())
        }
    }

    pub fn apply_iterator_to_layers(iterator: &mut AlIterator) -> Result<i32, String> {
        let mut count: i32 = 0;
        let status = unsafe { aluniverse_apply_iterator_to_layers(iterator.ptr, &mut count) };
        if status == statusCode::Success {
            Ok(count)
        } else {
            Err(status.to_string())
        }
    }

    pub fn apply_iterator_to_clouds(iterator: &mut AlIterator) -> Result<i32, String> {
        let mut count: i32 = 0;
        let status = unsafe { aluniverse_apply_iterator_to_clouds(iterator.ptr, &mut count) };
        if status == statusCode::Success {
            Ok(count)
        } else {
            Err(status.to_string())
        }
    }

    pub fn apply_iterator_to_blend_curves(iterator: &mut AlIterator) -> Result<i32, String> {
        let mut count: i32 = 0;
        let status = unsafe { aluniverse_apply_iterator_to_blend_curves(iterator.ptr, &mut count) };
        if status == statusCode::Success {
            Ok(count)
        } else {
            Err(status.to_string())
        }
    }

    pub fn write_sdl_no_animation(file_name: &str, flag: bool) -> Result<(), String> {
        let c_file_name =
            std::ffi::CString::new(file_name).map_err(|_| "Invalid file name".to_string())?;
        let status = unsafe { aluniverse_write_sdl_no_animation(c_file_name.as_ptr(), flag) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn write_sdl(
        file_name: &str,
        flag: bool,
        start_time: f64,
        end_time: f64,
        frame_rate: f64,
    ) -> Result<(), String> {
        let c_file_name =
            std::ffi::CString::new(file_name).map_err(|_| "Invalid file name".to_string())?;
        let status = unsafe {
            aluniverse_write_sdl(c_file_name.as_ptr(), flag, start_time, end_time, frame_rate)
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn frame_range(range_type: AlFrameRangeType) -> Result<(f64, f64, f64), String> {
        let mut start: f64 = 0.0;
        let mut end: f64 = 0.0;
        let mut frame_rate: f64 = 0.0;
        let status =
            unsafe { aluniverse_frame_range(range_type, &mut start, &mut end, &mut frame_rate) };
        if status == statusCode::Success {
            Ok((start, end, frame_rate))
        } else {
            Err(status.to_string())
        }
    }

    pub fn current_time() -> f64 {
        unsafe { aluniverse_current_time() }
    }

    pub fn do_updates(enable: bool) -> Result<(), String> {
        let status = unsafe { aluniverse_do_updates(enable) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn updates_on() -> bool {
        unsafe { aluniverse_updates_on() }
    }

    pub fn application_mode() -> AlApplicationMode {
        unsafe { aluniverse_application_mode() }
    }

    pub fn user_pref_color(color_id: i32) -> Result<(i32, i32, i32), String> {
        let mut r: i32 = 0;
        let mut g: i32 = 0;
        let mut b: i32 = 0;
        let status = unsafe { aluniverse_user_pref_color(color_id, &mut r, &mut g, &mut b) };
        if status == statusCode::Success {
            Ok((r, g, b))
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_user_pref_color(color_id: i32, r: i32, g: i32, b: i32) -> Result<(), String> {
        let status = unsafe { aluniverse_set_user_pref_color(color_id, r, g, b) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn construction_plane_in_effect() -> bool {
        unsafe { aluniverse_construction_plane_in_effect() }
    }

    pub fn construction_plane_transform() -> Result<AlTM, String> {
        let tm = AlTM::zero();
        let status = unsafe { aluniverse_construction_plane_transform(tm) };
        if status == statusCode::Success {
            Ok(tm)
        } else {
            Err(status.to_string())
        }
    }

    pub fn delete_all() -> Result<(), String> {
        let status = unsafe { aluniverse_delete_all() };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn is_old_wire_file(file_name: &str) -> bool {
        let c_file_name = std::ffi::CString::new(file_name).unwrap_or_default();
        unsafe { aluniverse_is_old_wire_file(c_file_name.as_ptr()) }
    }

    pub fn is_old_wire_file_w(file_name: &str) -> bool {
        let wide: Vec<u16> = file_name.encode_utf16().chain(std::iter::once(0)).collect();
        unsafe { aluniverse_is_old_wire_file_w(wide.as_ptr()) }
    }

    pub fn store_current_window(
        file_name: &str,
        width: i32,
        height: i32,
        want_anti_alias: bool,
    ) -> Result<(), String> {
        let c_file_name =
            std::ffi::CString::new(file_name).map_err(|_| "Invalid file name".to_string())?;
        let status = unsafe {
            aluniverse_store_current_window(c_file_name.as_ptr(), width, height, want_anti_alias)
        };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn reverse_flipped_objects() {
        unsafe { aluniverse_reverse_flipped_objects() }
    }
    pub fn hardware_shader_on() {
        //通过模拟按键切换硬件着色器
        let mut enigo = Enigo::new();
        enigo.key_down(Key::Control);
        enigo.key_down(Key::Layout('o'));
        enigo.key_up(Key::Layout('o'));
        enigo.key_up(Key::Control);
    }

    pub fn hardware_shader_toggle() {
        //通过模拟按键切换硬件着色器
        let mut enigo = Enigo::new();
        enigo.key_down(Key::Control);
        enigo.key_down(Key::Layout('p'));
        enigo.key_up(Key::Layout('p'));
        enigo.key_up(Key::Control);
    }
    //物体被删除时候，需要系统快捷键辅助更新状态
    pub fn delete_state_update(){
        let mut enigo = Enigo::new();
        enigo.key_down(Key::Alt);
        enigo.key_down(Key::Layout('o'));
        enigo.key_up(Key::Layout('o'));
        enigo.key_up(Key::Alt);
    }
}

unsafe extern "C" {
    fn aluniverse_initialize(coordSystem: AlCoordinateSystem, initProjectEnv: bool) -> statusCode;
    fn aluniverse_is_initialized() -> bool;

    fn aluniverse_coordinate_system() -> AlCoordinateSystem;

    fn aluniverse_expand_file_name(
        outFileName: *mut i8,
        fileName: *const i8,
        directoryType: AlDirectoryType,
    ) -> statusCode;
    fn aluniverse_is_wire_file(fileName: *const i8, fileVersion: *mut i8) -> bool;

    fn aluniverse_wire_file_window_size(outSizeX: *mut i32, outSizeY: *mut i32) -> statusCode;
    fn aluniverse_set_wire_file_window_size(sizeX: i32, sizeY: i32) -> statusCode;

    fn aluniverse_file_type(fileName: *const i8) -> AlFileType;
    fn aluniverse_file_type_w(fileName: *const u16) -> AlFileType;

    fn aluniverse_retrieve(fileName: *const i8) -> statusCode;
    fn aluniverse_retrieve_with_progress(
        fileName: *const i8,
        progressCallback: extern "C" fn(i32),
    ) -> statusCode;

    fn aluniverse_last_wire_file_retrieved() -> *const i8;
    fn aluniverse_last_file_retrieved() -> *const i8;
    fn aluniverse_last_file_saved() -> *const i8;

    fn aluniverse_store(fileName: *const i8, dagNode: *mut AlDagNode_ptr) -> statusCode;
    fn aluniverse_store_ex(
        fileName: *const i8,
        dagNode: *mut AlDagNode_ptr,
        embedImageReferences: bool,
        includeInstalledImages: bool,
        fileType: AlFileType,
    ) -> statusCode;
    fn aluniverse_store_active(fileName: *const i8, fileType: AlFileType) -> statusCode;

    fn aluniverse_first_dag_node() -> *mut AlDagNode_ptr;
    fn aluniverse_first_set() -> *mut AlSet_ptr;
    fn aluniverse_first_cluster() -> *mut AlCluster_ptr;

    fn aluniverse_current_stage() -> *const i8;
    fn aluniverse_set_current_stage(stageName: *const i8) -> statusCode;

    fn aluniverse_merge_stage(stageName: *const i8) -> statusCode;
    fn aluniverse_merge_all_stages() -> statusCode;

    fn aluniverse_delete_stage(stageName: *const i8) -> statusCode;
    fn aluniverse_delete_all_stages() -> statusCode;

    fn aluniverse_stage_visibility(stageName: *const i8, outVisibility: *mut bool) -> statusCode;
    fn aluniverse_set_stage_visibility(stageName: *const i8, visibility: bool) -> statusCode;

    fn aluniverse_stage_window_source() -> *const i8;
    fn aluniverse_set_stage_window_source(source: *const i8) -> statusCode;

    fn aluniverse_stage_background_source() -> *const i8;
    fn aluniverse_set_stage_background_source(source: *const i8) -> statusCode;

    fn aluniverse_rename_stage(oldName: *const i8, newName: *const i8) -> statusCode;
    fn aluniverse_stage_names() -> *mut AlList_ptr;

    fn aluniverse_create_new_stage(stageName: *const i8, outNewName: *mut *mut i8) -> statusCode;

    fn aluniverse_retrieve_stage_set(fileName: *const i8, flag: bool) -> statusCode;
    fn aluniverse_save_stage_set(fileName: *const i8) -> statusCode;
    fn aluniverse_set_stage_wire_file_name(stageName: *const i8, fileName: *const i8)
    -> statusCode;
    fn aluniverse_stage_wire_file_name(stageName: *const i8) -> *const i8;

    fn aluniverse_first_window() -> *mut AlWindow_ptr;
    fn aluniverse_current_window() -> *mut AlWindow_ptr;
    fn aluniverse_sbd_window() -> *mut AlWindow_ptr;

    fn aluniverse_blind_data(
        dataId: i32,
        outValue: *mut i64,
        outString: *mut *const i8,
    ) -> statusCode;
    fn aluniverse_blind_data_ex(
        dataId: i32,
        subId: i32,
        outValue: *mut i64,
        outString: *mut *const i8,
    ) -> statusCode;
    fn aluniverse_set_blind_data(dataId: i32, value: i64, string: *const i8) -> statusCode;
    fn aluniverse_remove_blind_data(dataId: i32) -> statusCode;
    fn aluniverse_remove_blind_data_ex(dataId: i32, subId: i32) -> statusCode;

    fn aluniverse_first_channel() -> *mut AlChannel_ptr;
    fn aluniverse_next_channel(channel: *mut AlChannel_ptr) -> *mut AlChannel_ptr;
    fn aluniverse_next_channel_d(channel: *mut AlChannel_ptr) -> statusCode;

    fn aluniverse_first_action() -> *mut AlAction_ptr;
    fn aluniverse_next_action(action: *mut AlAction_ptr) -> *mut AlAction_ptr;

    fn aluniverse_first_shader() -> *mut AlShader_ptr;
    fn aluniverse_next_shader(shader: *mut AlShader_ptr) -> *mut AlShader_ptr;
    fn aluniverse_next_shader_d(shader: *mut AlShader_ptr) -> statusCode;

    fn aluniverse_first_switch_shader() -> *mut AlSwitchShader_ptr;
    fn aluniverse_next_switch_shader(shader: *mut AlSwitchShader_ptr) -> *mut AlSwitchShader_ptr;

    fn aluniverse_first_layered_shader() -> *mut AlLayeredShader_ptr;
    fn aluniverse_next_layered_shader(shader: *mut AlLayeredShader_ptr)
    -> *mut AlLayeredShader_ptr;

    fn aluniverse_first_environment() -> *mut AlEnvironment_ptr;
    fn aluniverse_first_inactive_environment() -> *mut AlEnvironment_ptr;
    fn aluniverse_next_inactive_environment(
        env: *const AlEnvironment_ptr,
    ) -> *mut AlEnvironment_ptr;

    fn aluniverse_first_layer() -> *mut AlLayer_ptr;
    fn aluniverse_next_layer(layer: *mut AlLayer_ptr) -> *mut AlLayer_ptr;
    fn aluniverse_next_layer_d(layer: *mut AlLayer_ptr) -> statusCode;
    fn aluniverse_layer_by_number(layerNum: i32) -> *mut AlLayer_ptr;
    fn aluniverse_layer_by_name(layerName: *mut i8) -> *mut AlLayer_ptr;

    fn aluniverse_creation_layer() -> *mut AlLayer_ptr;
    fn aluniverse_set_creation_layer(layer: *mut AlLayer_ptr) -> statusCode;

    fn aluniverse_layers_enabled() -> bool;
    fn aluniverse_set_layers_enabled(enabled: bool) -> statusCode;

    fn aluniverse_first_camera() -> *mut AlPerspectiveCamera_ptr;
    fn aluniverse_next_camera(camera: *mut AlPerspectiveCamera_ptr)
    -> *mut AlPerspectiveCamera_ptr;
    fn aluniverse_next_camera_d(camera: *mut AlPerspectiveCamera_ptr) -> statusCode;

    fn aluniverse_first_locator() -> *mut AlLocator_ptr;
    fn aluniverse_next_locator(locator: *mut AlLocator_ptr) -> *mut AlLocator_ptr;
    fn aluniverse_delete_all_locators() -> statusCode;

    fn aluniverse_first_construction_entity() -> *mut AlConstructionEntity_ptr;
    fn aluniverse_next_construction_entity(
        entity: *mut AlConstructionEntity_ptr,
    ) -> *mut AlConstructionEntity_ptr;
    fn aluniverse_delete_all_construction_entities() -> statusCode;

    fn aluniverse_import_cloud_file(fileName: *const i8, flags: i32) -> statusCode;
    fn aluniverse_first_cloud() -> *mut AlCloud_ptr;
    fn aluniverse_number_of_clouds() -> i32;

    fn aluniverse_import_subdiv(fileName: *const i8) -> statusCode;

    fn aluniverse_first_blend_curve() -> *mut AlBlendCurve_ptr;
    fn aluniverse_number_of_blend_curves() -> i32;

    fn aluniverse_first_canvas() -> *mut AlCanvas_ptr;
    fn aluniverse_next_canvas(canvas: *mut AlCanvas_ptr) -> *mut AlCanvas_ptr;
    fn aluniverse_next_canvas_d(canvas: *mut AlCanvas_ptr) -> statusCode;

    fn aluniverse_first_category() -> *mut AlCategory_ptr;
    fn aluniverse_next_category(category: *mut AlCategory_ptr) -> *mut AlCategory_ptr;

    fn aluniverse_import_reference_file(fileName: *const i8) -> *mut AlReferenceFile_ptr;
    fn aluniverse_remove_reference_file(refFile: *mut AlReferenceFile_ptr) -> statusCode;
    fn aluniverse_create_reference_alternative(name: *const i8) -> *mut AlReferenceFileSet_ptr;
    fn aluniverse_first_reference_file_set() -> *mut AlReferenceFileSet_ptr;
    fn aluniverse_next_reference_file_set(
        refSet: *mut AlReferenceFileSet_ptr,
    ) -> *mut AlReferenceFileSet_ptr;
    fn aluniverse_next_reference_file_set_d(refSet: *mut AlReferenceFileSet_ptr) -> statusCode;
    fn aluniverse_pack_reference(packDir: *const i8) -> statusCode;

    fn aluniverse_redraw_screen() -> statusCode;

    fn aluniverse_apply_iterator_to_windows(
        iter: *mut AlIterator_ptr,
        outCount: *mut i32,
    ) -> statusCode;
    fn aluniverse_apply_iterator_to_dag_nodes(
        iter: *mut AlIterator_ptr,
        outCount: *mut i32,
    ) -> statusCode;
    fn aluniverse_apply_iterator_to_actions(
        iter: *mut AlIterator_ptr,
        outCount: *mut i32,
    ) -> statusCode;
    fn aluniverse_apply_iterator_to_channels(
        iter: *mut AlIterator_ptr,
        outCount: *mut i32,
    ) -> statusCode;
    fn aluniverse_apply_iterator_to_sets(
        iter: *mut AlIterator_ptr,
        outCount: *mut i32,
    ) -> statusCode;
    fn aluniverse_apply_iterator_to_clusters(
        iter: *mut AlIterator_ptr,
        outCount: *mut i32,
    ) -> statusCode;
    fn aluniverse_apply_iterator_to_shaders(
        iter: *mut AlIterator_ptr,
        outCount: *mut i32,
    ) -> statusCode;
    fn aluniverse_apply_iterator_to_locators(
        iter: *mut AlIterator_ptr,
        outCount: *mut i32,
    ) -> statusCode;
    fn aluniverse_apply_iterator_to_layers(
        iter: *mut AlIterator_ptr,
        outCount: *mut i32,
    ) -> statusCode;
    fn aluniverse_apply_iterator_to_clouds(
        iter: *mut AlIterator_ptr,
        outCount: *mut i32,
    ) -> statusCode;
    fn aluniverse_apply_iterator_to_blend_curves(
        iter: *mut AlIterator_ptr,
        outCount: *mut i32,
    ) -> statusCode;

    fn aluniverse_write_sdl_no_animation(fileName: *const i8, flag: bool) -> statusCode;
    fn aluniverse_write_sdl(
        fileName: *const i8,
        flag: bool,
        startTime: f64,
        endTime: f64,
        frameRate: f64,
    ) -> statusCode;

    fn aluniverse_frame_range(
        rangeType: AlFrameRangeType,
        outStart: *mut f64,
        outEnd: *mut f64,
        outFrameRate: *mut f64,
    ) -> statusCode;
    fn aluniverse_current_time() -> f64;

    fn aluniverse_do_updates(enable: bool) -> statusCode;
    fn aluniverse_updates_on() -> bool;
    fn aluniverse_application_mode() -> AlApplicationMode;

    fn aluniverse_user_pref_color(
        colorId: i32,
        outR: *mut i32,
        outG: *mut i32,
        outB: *mut i32,
    ) -> statusCode;
    fn aluniverse_set_user_pref_color(colorId: i32, r: i32, g: i32, b: i32) -> statusCode;

    fn aluniverse_construction_plane_in_effect() -> bool;
    fn aluniverse_construction_plane_transform(outTransform: AlTM) -> statusCode;

    fn aluniverse_delete_all() -> statusCode;

    fn aluniverse_is_old_wire_file(fileName: *const i8) -> bool;
    fn aluniverse_is_old_wire_file_w(fileName: *const u16) -> bool;

    fn aluniverse_store_current_window(
        filename: *const i8,
        width: i32,
        height: i32,
        wantAntiAlias: bool,
    ) -> statusCode;

    fn aluniverse_reverse_flipped_objects();
}
