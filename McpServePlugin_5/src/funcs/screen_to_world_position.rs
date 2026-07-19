use crate::*;
use serde_json::{Value, json};

pub struct ScreenToWorldPosition;

impl ScreenToWorldPosition {
    pub fn info() -> Value {
        json!({
            "name": "screen_to_world_position",
            "description": "Convert screen coordinates to world coordinates using a specified construction plane by name.",
            "examples": [
                {
                    "description": "Convert screen point (100, 200) to world coordinates using 'MyPlane'",
                    "command": "alias_lic screen_to_world_position --screen_x 100 --screen_y 200 --plane_name MyPlane"
                },
                {
                    "description": "Convert screen center to world coordinates using 'ConstructionPlane'",
                    "command": "alias_lic screen_to_world_position --screen_x 512 --screen_y 384 --plane_name ConstructionPlane"
                }
            ],
            "inputSchema": {
                "type": "object",
                "properties": {
                    "screen_x": {
                        "type": "integer",
                        "description": "X coordinate on screen"
                    },
                    "screen_y": {
                        "type": "integer",
                        "description": "Y coordinate on screen"
                    },
                    "plane_name": {
                        "type": "string",
                        "description": "Name of the construction plane to use for mapping"
                    }
                },
                "required": ["screen_x", "screen_y", "plane_name"]
            }
        })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let screen_x = args["screen_x"].as_i64().ok_or("screen_x is required")? as i32;
        let screen_y = args["screen_y"].as_i64().ok_or("screen_y is required")? as i32;
        let plane_name = args["plane_name"].as_str().ok_or("plane_name is required")?;

        let plane_obj = AlPickList::pick_name(plane_name)
            .ok_or(format!("Construction plane '{}' not found", plane_name))?;
        let construction_plane = plane_obj
            .as_construction_plane()
            .map_err(|_| format!("Object '{}' is not a construction plane", plane_name))?;

        let origin = construction_plane.origin();
        let (x_axis, y_axis, _) = construction_plane.axes();
        let plane = RU_Plane::new(origin, x_axis, y_axis)?;

        let window = AlUniverse::current_window().ok_or("no alwindow")?;

        let result = window.map_to_world_postion(screen_x, screen_y, plane);

        match result {
            Some((world_point, t)) => Ok(json!({
                "jsonrpc": "2.0",
                "id": id_val,
                "result": {
                    "content": [{
                        "type": "text",
                        "text": format!("Screen ({}, {}) maps to world ({}, {}, {}) using plane '{}'", screen_x, screen_y, world_point.x, world_point.y, world_point.z, plane_name)
                    }],
                    "world_position": [world_point.x, world_point.y, world_point.z],
                    "t": t
                }
            })),
            None => Err(format!("Failed to map screen ({}, {}) to world position using plane '{}'", screen_x, screen_y, plane_name)),
        }
    }
}
