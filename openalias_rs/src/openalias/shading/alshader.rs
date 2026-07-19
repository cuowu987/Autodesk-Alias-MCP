use crate::*;

#[repr(C)]
pub struct AlShader_ptr {
    _private: [u8; 0],
}

#[derive(Debug)]
pub struct AlShader {
    pub ptr: *mut AlShader_ptr,
}

unsafe impl Send for AlShader {}
unsafe impl Sync for AlShader {}

impl Drop for AlShader {
    fn drop(&mut self) {
        self.destroy();
    }
}
impl AlObjectMethods for AlShader {
    fn as_object_ptr(&self) -> *mut AlObject_ptr {
        self.ptr as *mut AlObject_ptr
    }
}

impl AlShader {
    pub fn create() -> Result<Self, String> {
        let ptr = unsafe { alshader_create() };
        let status = unsafe { alshader_create_shader(ptr) };
        if status == statusCode::Success {
            Ok(Self { ptr })
        } else {
            Err(status.to_string())
        }
    }

    pub fn destroy(&mut self) {
        if !self.ptr.is_null() {
            unsafe { alshader_destroy(self.ptr) };
            self.ptr = std::ptr::null_mut();
        }
    }

    pub fn parameter(&self, field: AlShadingFields) -> Result<f64, String> {
        let mut value: f64 = 0.0;
        let status = unsafe { alshader_parameter(self.ptr, field as i32, &mut value) };
        if status == statusCode::Success {
            Ok(value)
        } else {
            Err(status.to_string())
        }
    }
    pub fn color(&self) -> Result<[f64; 4], String> {
        let mut color: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
        color[0] = self.parameter(AlShadingFields::kFLD_SHADING_COMMON_COLOR_R)?;
        color[1] = self.parameter(AlShadingFields::kFLD_SHADING_COMMON_COLOR_G)?;
        color[2] = self.parameter(AlShadingFields::kFLD_SHADING_COMMON_COLOR_B)?;
        Ok(color)
    }

    pub fn set_parameter(&mut self, field: AlShadingFields, value: f64) -> Result<(), String> {
        let status = unsafe { alshader_set_parameter(self.ptr, field as i32, value) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }
    pub fn set_color(&mut self, color: [f32; 4]) -> Result<(), String> {
        self.set_parameter(AlShadingFields::kFLD_SHADING_COMMON_COLOR_R, color[0] as f64 * 100.0)?;
        self.set_parameter(AlShadingFields::kFLD_SHADING_COMMON_COLOR_G, color[1] as f64 * 100.0)?;
        self.set_parameter(AlShadingFields::kFLD_SHADING_COMMON_COLOR_B, color[2] as f64 * 100.0)?;
        Ok(())
    }

    pub fn blind_data(&self, user_type: i32) -> Result<(i64, String), String> {
        let mut out_long: i64 = 0;
        let mut out_string: *const i8 = std::ptr::null();
        let status =
            unsafe { alshader_blind_data(self.ptr, user_type, &mut out_long, &mut out_string) };
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
            Ok((out_long, s))
        } else {
            Err(status.to_string())
        }
    }

    pub fn set_blind_data(
        &mut self,
        user_type: i32,
        value: i64,
        string: &str,
    ) -> Result<(), String> {
        let c_string = std::ffi::CString::new(string).map_err(|_| "Invalid string".to_string())?;
        let status =
            unsafe { alshader_set_blind_data(self.ptr, user_type, value, c_string.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn remove_blind_data(&mut self, user_type: i32) -> Result<(), String> {
        let status = unsafe { alshader_remove_blind_data(self.ptr, user_type) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn shading_model(&self) -> Option<String> {
        let c_str = unsafe { alshader_shading_model(self.ptr) };
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

    pub fn set_shading_model(&mut self, model: &str) -> Result<(), String> {
        let c_model = std::ffi::CString::new(model).map_err(|_| "Invalid string".to_string())?;
        let status = unsafe { alshader_set_shading_model(self.ptr, c_model.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn first_texture(&self) -> Option<AlTexture> {
        let ptr = unsafe { alshader_first_texture(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlTexture { ptr })
        }
    }

    pub fn next_texture(&self, texture: &AlTexture) -> Option<AlTexture> {
        let ptr = unsafe { alshader_next_texture(self.ptr, texture.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlTexture { ptr })
        }
    }

    pub fn next_texture_d(&mut self, texture: &mut AlTexture) -> Result<(), String> {
        let status = unsafe { alshader_next_texture_d(self.ptr, texture.ptr) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn fields(&self) -> Option<AlList> {
        let ptr = unsafe { alshader_fields(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlList { ptr })
        }
    }

    pub fn mapped_fields(&self) -> Option<AlList> {
        let ptr = unsafe { alshader_mapped_fields(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlList { ptr })
        }
    }

    pub fn add_texture(&mut self, param_name: &str, file_name: &str) -> Result<AlTexture, String> {
        let c_param_name =
            std::ffi::CString::new(param_name).map_err(|_| "Invalid param name".to_string())?;
        let c_file_name =
            std::ffi::CString::new(file_name).map_err(|_| "Invalid file name".to_string())?;
        let mut out_texture: *mut AlTexture_ptr = std::ptr::null_mut();
        let status = unsafe {
            alshader_add_texture(
                self.ptr,
                c_param_name.as_ptr(),
                c_file_name.as_ptr(),
                &mut out_texture,
            )
        };
        if status == statusCode::Success {
            Ok(AlTexture { ptr: out_texture })
        } else {
            Err(status.to_string())
        }
    }

    pub fn remove_texture(&mut self, param_name: &str) -> Result<(), String> {
        let c_param_name =
            std::ffi::CString::new(param_name).map_err(|_| "Invalid param name".to_string())?;
        let status = unsafe { alshader_remove_texture(self.ptr, c_param_name.as_ptr()) };
        if status == statusCode::Success {
            Ok(())
        } else {
            Err(status.to_string())
        }
    }

    pub fn apply_iterator_to_textures(&mut self, iterator: &mut AlIterator) -> Result<i32, String> {
        let mut count: i32 = 0;
        let status =
            unsafe { alshader_apply_iterator_to_textures(self.ptr, iterator.ptr, &mut count) };
        if status == statusCode::Success {
            Ok(count)
        } else {
            Err(status.to_string())
        }
    }

    pub fn copy_object(&mut self) -> Option<AlShader> {
        let ptr = unsafe { alshader_copy_object(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(AlShader { ptr })
        }
    }

    pub fn is_used(&self) -> bool {
        unsafe { alshader_is_used(self.ptr) }
    }

    pub fn convert_solid_to_file_texture(
        &mut self,
        dag_node: &AlDagNode,
        param: AlShadingFields,
        use_mip_map: bool,
        file_name: &str,
    ) -> Option<AlShader> {
        let c_file_name = std::ffi::CString::new(file_name)
            .map_err(|_| "Invalid file name".to_string())
            .ok()?;
        let ptr = unsafe {
            alshader_convert_solid_to_file_texture(
                self.ptr,
                dag_node.ptr,
                param as i32,
                use_mip_map,
                c_file_name.as_ptr(),
            )
        };
        if ptr.is_null() {
            None
        } else {
            Some(AlShader { ptr })
        }
    }

    pub fn convert_environment_to_file_texture(
        &mut self,
        param: AlShadingFields,
        file_name: &str,
        format: i32,
        use_mip_map: bool,
    ) -> Option<AlShader> {
        let c_file_name = std::ffi::CString::new(file_name)
            .map_err(|_| "Invalid file name".to_string())
            .ok()?;
        let ptr = unsafe {
            alshader_convert_environment_to_file_texture(
                self.ptr,
                param as i32,
                c_file_name.as_ptr(),
                format,
                use_mip_map,
            )
        };
        if ptr.is_null() {
            None
        } else {
            Some(AlShader { ptr })
        }
    }
}

unsafe extern "C" {
    fn alshader_create() -> *mut AlShader_ptr;
    fn alshader_destroy(shader: *mut AlShader_ptr);

    fn alshader_create_shader(shader: *mut AlShader_ptr) -> statusCode;

    fn alshader_parameter(
        shader: *const AlShader_ptr,
        field: i32,
        out_value: *mut f64,
    ) -> statusCode;
    fn alshader_set_parameter(shader: *mut AlShader_ptr, field: i32, value: f64) -> statusCode;

    fn alshader_blind_data(
        shader: *const AlShader_ptr,
        user_type: i32,
        out_long: *mut i64,
        out_string: *mut *const i8,
    ) -> statusCode;
    fn alshader_set_blind_data(
        shader: *mut AlShader_ptr,
        user_type: i32,
        value: i64,
        string: *const i8,
    ) -> statusCode;
    fn alshader_remove_blind_data(shader: *mut AlShader_ptr, user_type: i32) -> statusCode;

    fn alshader_shading_model(shader: *const AlShader_ptr) -> *const i8;
    fn alshader_set_shading_model(shader: *mut AlShader_ptr, model: *const i8) -> statusCode;

    fn alshader_first_texture(shader: *const AlShader_ptr) -> *mut AlTexture_ptr;
    fn alshader_next_texture(
        shader: *const AlShader_ptr,
        texture: *const AlTexture_ptr,
    ) -> *mut AlTexture_ptr;
    fn alshader_next_texture_d(
        shader: *mut AlShader_ptr,
        texture: *mut AlTexture_ptr,
    ) -> statusCode;

    fn alshader_fields(shader: *const AlShader_ptr) -> *mut AlList_ptr;
    fn alshader_mapped_fields(shader: *const AlShader_ptr) -> *mut AlList_ptr;

    fn alshader_add_texture(
        shader: *mut AlShader_ptr,
        param_name: *const i8,
        file_name: *const i8,
        out_texture: *mut *mut AlTexture_ptr,
    ) -> statusCode;
    fn alshader_remove_texture(shader: *mut AlShader_ptr, param_name: *const i8) -> statusCode;

    fn alshader_apply_iterator_to_textures(
        shader: *mut AlShader_ptr,
        iterator: *mut AlIterator_ptr,
        out_count: *mut i32,
    ) -> statusCode;

    fn alshader_copy_object(shader: *mut AlShader_ptr) -> *mut AlShader_ptr;
    fn alshader_is_used(shader: *mut AlShader_ptr) -> bool;

    fn alshader_convert_solid_to_file_texture(
        shader: *mut AlShader_ptr,
        dag_node: *mut AlDagNode_ptr,
        param: i32,
        use_mip_map: bool,
        file_name: *const i8,
    ) -> *mut AlShader_ptr;
    fn alshader_convert_environment_to_file_texture(
        shader: *mut AlShader_ptr,
        param: i32,
        file_name: *const i8,
        format: i32,
        use_mip_map: bool,
    ) -> *mut AlShader_ptr;
}
