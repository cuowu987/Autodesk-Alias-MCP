#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
use crate::*;

#[repr(C)]
pub struct AlMessageTypeHandle_ptr {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug)]
pub struct AlMessageTypeHandle {
    pub ptr: *mut AlMessageTypeHandle_ptr,
}

impl Drop for AlMessageTypeHandle {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl AlMessageTypeHandle {
    pub fn create() -> Option<AlMessageTypeHandle> {
        let ptr = unsafe { almessagetypehandle_create() };
        if ptr.is_null() {
            None
        } else {
            Some(AlMessageTypeHandle { ptr })
        }
    }

    pub fn create_copy(other: &AlMessageTypeHandle) -> Option<AlMessageTypeHandle> {
        let ptr = unsafe { almessagetypehandle_create_copy(other.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlMessageTypeHandle { ptr })
        }
    }

    pub fn destroy(&mut self) {
        unsafe { almessagetypehandle_destroy(self.ptr) };
        self.ptr = std::ptr::null_mut();
    }

    pub fn assign(&mut self, other: &AlMessageTypeHandle) {
        unsafe { almessagetypehandle_assign(self.ptr, other.ptr) };
    }

    pub fn is_valid(&self) -> bool {
        unsafe { almessagetypehandle_is_valid(self.ptr) }
    }

    pub fn type_(&self) -> i32 {
        unsafe { almessagetypehandle_type(self.ptr) }
    }

    pub fn set_prologue(
        &mut self,
        prologue: Option<extern "C" fn(i32, *mut std::ffi::c_void) -> i32>,
    ) -> Result<(), String> {
        let func =
            prologue.unwrap_or_else(|| unsafe { std::mem::transmute(std::ptr::null_mut::<u8>()) });
        let status = unsafe { almessagetypehandle_set_prologue(self.ptr, func) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_epilogue(
        &mut self,
        epilogue: Option<extern "C" fn(i32, *mut std::ffi::c_void) -> i32>,
    ) -> Result<(), String> {
        let func =
            epilogue.unwrap_or_else(|| unsafe { std::mem::transmute(std::ptr::null_mut::<u8>()) });
        let status = unsafe { almessagetypehandle_set_epilogue(self.ptr, func) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_lock(&self) -> Result<bool, String> {
        let mut locked = false;
        let status = unsafe { almessagetypehandle_add_lock(self.ptr, &mut locked) };
        if status == statusCode::Success {
            Ok(locked)
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_add_lock(&mut self, locked: bool) -> Result<(), String> {
        let status = unsafe { almessagetypehandle_set_add_lock(self.ptr, locked) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn send_lock(&self) -> Result<bool, String> {
        let mut locked = false;
        let status = unsafe { almessagetypehandle_send_lock(self.ptr, &mut locked) };
        if status == statusCode::Success {
            Ok(locked)
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_send_lock(&mut self, locked: bool) -> Result<(), String> {
        let status = unsafe { almessagetypehandle_set_send_lock(self.ptr, locked) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }
}

pub struct AlMessage;

impl AlMessage {
    pub fn add_message_handler(
        message_type: AlMessageType,
        callback: *mut std::ffi::c_void,
    ) -> Result<(), String> {
        AlMessage::add_message_handler_i32(message_type as i32, callback)
    }
    pub fn add_message_handler_i32(
        message_type: i32,
        callback: *mut std::ffi::c_void,
    ) -> Result<(), String> {
        let status = unsafe { almessage_add_message_handler(message_type, callback) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn remove_message_handler(
        message_type: AlMessageType,
        callback: *mut std::ffi::c_void,
    ) -> Result<(), String> {
        AlMessage::remove_message_handler_i32(message_type as i32, callback)
    }
    pub fn remove_message_handler_i32(
        message_type: i32,
        callback: *mut std::ffi::c_void,
    ) -> Result<(), String> {
        let status = unsafe { almessage_remove_message_handler(message_type, callback) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn add_message_type(name: &str) -> Result<AlMessageTypeHandle, String> {
        let c_name = std::ffi::CString::new(name).map_err(|_| "Invalid string".to_string())?;
        let ptr = unsafe { almessage_add_message_type(c_name.as_ptr()) };
        if ptr.is_null() {
            return Err("Failed to add message type: null pointer returned".to_string());
        }
        Ok(AlMessageTypeHandle { ptr })
    }

    pub fn get_message_type(name: &str) -> Result<i32, String> {
        let c_name = std::ffi::CString::new(name).map_err(|_| "Invalid string".to_string())?;
        let value = unsafe { almessage_get_message_type(c_name.as_ptr()) };
        Ok(value)
    }

    pub fn send_message(
        message_type: AlMessageType,
        data: *mut std::ffi::c_void,
        priority: AlPriorityType,
    ) -> Result<(), String> {
        AlMessage::send_message_i32(message_type as i32, data, priority)
    }
    pub fn send_message_i32(
        message_type: i32,
        data: *mut std::ffi::c_void,
        priority: AlPriorityType,
    ) -> Result<(), String> {
        let status = unsafe { almessage_send_message(message_type, data, priority) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlMessageType {
    FirstMessage = 0,
    DagNodeDeleted = 1,                  // 删除DAG节点
    DagNodeInstanced = 2,                // 实例化DAG节点
    DeleteAll = 3,                       // 删除所有DAG节点
    DagNodeModified = 4,                 // 修改DAG节点
    DagNodeModifiedGeometry = 5,         // 修改DAG节点几何体
    DagNodeModifiedShaderAssignment = 6, // 修改DAG节点着色器赋值
    DagNodeModifiedConstraint = 7,       // 修改DAG节点约束
    DagNodePreReplaceGeometry = 8,       // 替换DAG节点几何体前
    DagNodeReplaceGeometry = 9,          // 替换DAG节点几何体
    DagNodeApplyTransformation = 10,     // 应用DAG节点变换
    DagNodeVisible = 11,                 // DAG节点可见
    DagDispModified = 12,                // 修改DAG节点显示
    CosDeleted = 13,                     // 删除COS
    CosModified = 14,                    // 修改COS
    CosVisible = 15,                     // COS可见
    AttributesDelete = 16,               // 删除属性
    PreUpdate = 17,                      // 更新前
    Update = 18,                         // 更新
    PostUpdate = 19,                     // 更新后
    AnimPlayback = 20,                   // 动画播放
    ListModifiedNodes = 21,              // 列表修改节点
    PreRefresh = 22,                     // 刷新前
    Refresh = 23,                        // 刷新
    CommandInstall = 24,                 // 命令安装
    CommandFree = 25,                    // 命令释放
    PickListModified = 26,               // 选择列表修改
    TrimSurface = 27,                    // 裁剪表面
    UntrimSurface = 28,                  // 取消裁剪表面
    PlotRefresh = 29,                    // 刷新绘制
    UniverseCreated = 30,                // 创建宇宙
    UniverseDeleted = 31,                // 删除宇宙
    UniverseMerged = 32,                 // 合并宇宙
    Quit = 33,                           // 退出
    PreRetrieve = 34,                    // 导入前
    PostRetrieve = 35,                   // 导入后
    PreStore = 36,                       // 保存前
    PostStore = 37,                      // 保存后
    DagInserted = 38,                    // 插入DAG节点
    DagNameModified = 39,                // 修改DAG节点名称
    UniverseActive = 40,                 // 洇宙激活
    ClMemberModified = 41,               // 修改类成员
    ExprModified = 42,                   // 修改表达式
    CommandUnInstall = 43,               // 命令卸载
    JointModified = 44,                  // 修改关节
    HierarchyModified = 45,              // 修改层级
    CnetDeleted = 46,                    // 删除CNET
    CloudDeleted = 47,                   // 删除云
    ShaderModified = 48,                 // 修改着色器
    LightModified = 49,                  // 修改灯光
    CameraModified = 50,                 // 修改相机
    TextureModified = 51,                // 修改纹理
    TextureAdded = 52,                   // 添加纹理
    TextureDeleted = 53,                 // 删除纹理
    ShaderAdded = 54,                    // 添加着色器
    ShaderDeleted = 55,                  // 删除着色器
    ReferenceFileDeleted = 56,           // 删除引用文件
    ReferenceFileSetDeleted = 57,        // 删除引用文件集
    LayerAssign = 58,                    // 分配层
    LayerSymmetryModified = 59,          // 修改层对称
    DagNodeUndeleted = 60,               // 未删除DAG节点
    LayerStateModified = 61,             // 修改层状态
    LayerVisibilityModified = 62,        // 修改层可见性
    LayersEnabled = 63,                  // 启用层
    LayersReordered = 64,                // 层重新排序
    LayerAdded = 65,                     // 添加层
    LayerDeleted = 66,                   // 删除层
    LayerAttributeChanged = 67,          // 层层属性修改
    ReferenceFileModified = 68,          // 修改引用文件
    ReferenceFileAdded = 69,             // 添加引用文件
    ReferenceFileSetModified = 70,       // 修改引用文件集
    ReferenceFileSetAdded = 71,          // 添加引用文件集
    ReferenceLayerModified = 72,         // 修改引用层
    CosCreated = 73,                     // 创建COS
    LocatorAdded = 74,                   // 添加定位器
    LocatorDeleted = 75,                 // 删除定位器
    LocatorModified = 76,                // 修改定位器
    DagNodeColourModified = 77,          // 修改DAG节点颜色
    LastMessage = 78,                    // 最后一条消息
}

pub const kMessageInvalid: AlMessageType = AlMessageType::FirstMessage;
pub const kMessageStageCreated: AlMessageType = AlMessageType::UniverseCreated;
pub const kMessageStageDeleted: AlMessageType = AlMessageType::UniverseDeleted;
pub const kMessageStageMerged: AlMessageType = AlMessageType::UniverseMerged;
pub const kMessageStageActive: AlMessageType = AlMessageType::UniverseActive;

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum AlPriorityType {
    kImmediate = 0,
    kQueue = 1,
    kIdleQueue = 2,
}

#[repr(C)]
pub struct AlReferenceLayer_ptr {
    _private: [u8; 0],
}

pub type AlCallbackVoid = extern "C" fn(AlMessageType);
pub type AlCallbackInt = extern "C" fn(AlMessageType, i32);
pub type AlCallback2Ints = extern "C" fn(AlMessageType, i32, i32);
pub type AlCallbackStringInt = extern "C" fn(AlMessageType, *const i8, i32);
pub type AlCallbackString = extern "C" fn(AlMessageType, *const i8);
pub type AlCallbackCurveOnSurface = extern "C" fn(AlMessageType, *mut AlCurveOnSurface_ptr);
pub type AlCallbackAttributes = extern "C" fn(AlMessageType, *mut AlAttributes_ptr);
pub type AlCallbackUpdate = extern "C" fn(AlMessageType, i32) -> i32;
pub type AlCallbackSurface = extern "C" fn(AlMessageType, *mut AlSurface_ptr);
pub type AlCallbackOneDagNode = extern "C" fn(AlMessageType, *mut AlDagNode_ptr);
pub type AlCallbackTwoDagNodes =
    extern "C" fn(AlMessageType, *mut AlDagNode_ptr, *mut AlDagNode_ptr);
pub type AlCallbackDagNodeAndMatrix = extern "C" fn(AlMessageType, *mut AlDagNode_ptr, *mut AlTM);
pub type AlCallbackOneCloud = extern "C" fn(AlMessageType, *mut AlCloud_ptr);
pub type AlCallbackOneShader = extern "C" fn(AlMessageType, *mut AlShader_ptr);
pub type AlCallbackOneTexture = extern "C" fn(AlMessageType, *mut AlTexture_ptr);
pub type AlCallbackOneLight = extern "C" fn(AlMessageType, *mut AlLight_ptr);
pub type AlCallbackOneCamera = extern "C" fn(AlMessageType, *mut AlCamera_ptr);
pub type AlCallbackOneRefFile = extern "C" fn(AlMessageType, *mut AlReferenceFile_ptr);
pub type AlCallbackOneRefFileMod = extern "C" fn(AlMessageType, *mut AlReferenceFile_ptr, i32);
pub type AlCallbackOneRefFileSet = extern "C" fn(AlMessageType, *mut AlReferenceFileSet_ptr);
pub type AlCallbackOneRefFileSetMod =
    extern "C" fn(AlMessageType, *mut AlReferenceFileSet_ptr, i32);
pub type AlCallbackOneRefLayer = extern "C" fn(AlMessageType, *mut AlReferenceLayer_ptr);
pub type AlCallbackOneRefLayerMod = extern "C" fn(AlMessageType, *mut AlReferenceLayer_ptr, i32);
pub type AlCallbackLocator = extern "C" fn(AlMessageType, *mut AlLocator_ptr);
pub type AlCallbackListModifiedNodes =
    extern "C" fn(AlMessageType, *const AlDagNode_ptr, *mut AlObject_ptr) -> bool;

unsafe extern "C" {
    fn almessagetypehandle_create() -> *mut AlMessageTypeHandle_ptr;
    fn almessagetypehandle_create_copy(
        other: *const AlMessageTypeHandle_ptr,
    ) -> *mut AlMessageTypeHandle_ptr;
    fn almessagetypehandle_destroy(handle: *mut AlMessageTypeHandle_ptr);

    fn almessagetypehandle_assign(
        handle: *mut AlMessageTypeHandle_ptr,
        other: *const AlMessageTypeHandle_ptr,
    );

    fn almessagetypehandle_is_valid(handle: *const AlMessageTypeHandle_ptr) -> bool;
    fn almessagetypehandle_type(handle: *const AlMessageTypeHandle_ptr) -> i32;

    fn almessagetypehandle_set_prologue(
        handle: *mut AlMessageTypeHandle_ptr,
        prologue: extern "C" fn(i32, *mut std::ffi::c_void) -> i32,
    ) -> statusCode;
    fn almessagetypehandle_set_epilogue(
        handle: *mut AlMessageTypeHandle_ptr,
        epilogue: extern "C" fn(i32, *mut std::ffi::c_void) -> i32,
    ) -> statusCode;

    fn almessagetypehandle_add_lock(
        handle: *mut AlMessageTypeHandle_ptr,
        out_locked: *mut bool,
    ) -> statusCode;
    fn almessagetypehandle_set_add_lock(
        handle: *mut AlMessageTypeHandle_ptr,
        locked: bool,
    ) -> statusCode;

    fn almessagetypehandle_send_lock(
        handle: *mut AlMessageTypeHandle_ptr,
        out_locked: *mut bool,
    ) -> statusCode;
    fn almessagetypehandle_set_send_lock(
        handle: *mut AlMessageTypeHandle_ptr,
        locked: bool,
    ) -> statusCode;

    fn almessage_add_message_handler(
        message_type: i32,
        callback: *mut std::ffi::c_void,
    ) -> statusCode;
    fn almessage_remove_message_handler(
        message_type: i32,
        callback: *mut std::ffi::c_void,
    ) -> statusCode;

    fn almessage_add_message_type(name: *const i8) -> *mut AlMessageTypeHandle_ptr;
    fn almessage_get_message_type(name: *const i8) -> i32;

    fn almessage_send_message(
        message_type: i32,
        data: *mut std::ffi::c_void,
        priority: AlPriorityType,
    ) -> statusCode;
}
