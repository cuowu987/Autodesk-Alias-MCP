use crate::*;
use serde_json::{Value, json};

pub struct ObjectCreatePoint;

impl ObjectCreatePoint {
    pub fn info() -> Value {
        json!({
            "name": "object_create_point",
            "description": "Create a new space point at the specified 3D coordinates (x, y, z). The point will be named 'ai_point' by default.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "x": { "type": "number","description": "X coordinate of the point"},
                    "y": { "type": "number","description": "Y coordinate of the point" },
                    "z": { "type": "number","description": "Z coordinate of the point" }
                },
                "required": ["x", "y", "z"]
            },
            "examples": [
                {
                    "description": "Create a point at the origin",
                    "command": "alias_lic object_create_point --x 0 --y 0 --z 0"
                },
                {
                    "description": "Create a point at specific coordinates",
                    "command": "alias_lic object_create_point --x 10 --y 20 --z 30"
                }
            ]
        })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let x = args["x"].as_f64().unwrap_or(0.0);
        let y = args["y"].as_f64().unwrap_or(0.0);
        let z = args["z"].as_f64().unwrap_or(0.0);
        let alpoint = AlSpacePoint::new_create(x, y, z)?;
        AlUniverse::redraw_screen().ok();
        alpoint.set_name("ai_point").ok();
        Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "result": {
                "content": [{
                    "type": "text",
                    "text": format!(" Point{{name:{}}} created ", alpoint.name())
                }]
            }
        }))
    }
}
