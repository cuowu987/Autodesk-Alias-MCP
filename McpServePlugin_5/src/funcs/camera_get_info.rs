use crate::*;
use serde_json::{Value, json};

pub struct CameraGetInfo;

impl CameraGetInfo {
    pub fn info() -> Value {
        json!({
            "name": "camera_get_info",
            "description": "Get current camera information including position, orientation, and projection parameters.",
            "examples": [
                {
                    "description": "Get information about the current camera",
                    "command": "alias_lic camera_get_info"
                }
            ],
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    pub fn func(_args: &Value, id_val: &Value) -> Result<Value, String> {
        let window = AlUniverse::current_window().ok_or("no alwindow")?;
        let camera_obj = window.camera().ok_or("no camera")?;
        let is_orthographic = camera_obj.type_() == AlObjectType::kOrthographicCameraType;

        let result = if is_orthographic {
            let (origin, right, up, top_right, center, view_dir) =
                window.orthographic_camera_params()?;
            json!({
                "camera_type": "orthographic",
                "origin": [origin.x, origin.y, origin.z],
                "right": [right.x, right.y, right.z],
                "up": [up.x, up.y, up.z],
                "top_right": [top_right.x, top_right.y, top_right.z],
                "center": [center.x, center.y, center.z],
                "view_dir": [view_dir.x, view_dir.y, view_dir.z]
            })
        } else {
            let perspective = camera_obj.as_perspective_camera()?;
            let (
                eye_pos,
                center_pos,
                up_vec,
                fov,
                near,
                far,
                film_back_w,
                film_back_h,
                focal_length,
                fit_code,
            ) = perspective.camera_params()?;
            let dx = center_pos.x - eye_pos.x;
            let dy = center_pos.y - eye_pos.y;
            let dz = center_pos.z - eye_pos.z;
            let len = (dx * dx + dy * dy + dz * dz).sqrt();
            let (vx, vy, vz) = if len < 1e-12 {
                (0.0, 0.0, -1.0)
            } else {
                (dx / len, dy / len, dz / len)
            };
            let fit_code_i32 = fit_code as i32;
            json!({
                "camera_type": "perspective",
                "eye_position": [eye_pos.x, eye_pos.y, eye_pos.z],
                "center_position": [center_pos.x, center_pos.y, center_pos.z],
                "up_vector": [up_vec.x, up_vec.y, up_vec.z],
                "view_dir": [vx, vy, vz],
                "fov": fov,
                "near": near,
                "far": far,
                "film_back_width": film_back_w,
                "film_back_height": film_back_h,
                "focal_length": focal_length,
                "fit_code": fit_code_i32
            })
        };

        Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "result": {
                "content": [{
                    "type": "text",
                    "text": result.to_string()
                }]
            }
        }))
    }
}
