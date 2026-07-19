use crate::*;
use serde_json::{Value, json};

pub struct CurveCvGet;

impl CurveCvGet {
    pub fn info() -> Value {
        json!({
            "name": "curve_cv_get",
            "description": "Get the world position of a specific control point (CV) on a curve by name and index. Returns the 3D coordinates of the control point.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Name of the curve to query"
                    },
                    "index": {
                        "type": "integer",
                        "description": "Index of the control point to get (0-based)"
                    }
                },
                "required": ["name", "index"]
            },
            "examples": [
                {
                    "description": "Get the first control point (index 0) of a curve",
                    "command": "alias_lic curve_cv_get --name curve1 --index 0"
                },
                {
                    "description": "Get the 5th control point (index 4) of a curve",
                    "command": "alias_lic curve_cv_get --name curve2 --index 4"
                }
            ]
        })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let name = args["name"].as_str().unwrap_or("");
        let index = args["index"].as_i64().unwrap_or(-1) as i32;
        
        if name.is_empty() {
            return Err("name is required".to_string());
        }
        if index < 0 {
            return Err("index must be >= 0".to_string());
        }
        
        let obj = AlPickList::pick_name(name)
            .ok_or_else(|| format!("Curve '{}' not found", name))?;
        let curve_node = obj
            .as_curve_node()
            .map_err(|_| format!("Object '{}' is not a curve", name))?;
        let curve = curve_node.curve().ok_or("Curve is null")?;
        
        let cv_num = curve.number_of_cvs();
        if index >= cv_num as i32 {
            return Err(format!("Index {} exceeds curve CV count {}", index, cv_num));
        }
        
        let cv = curve
            .get_cv(index)
            .ok_or_else(|| format!("Failed to get CV at index {}", index))?;
        
        let pos = cv.world_position()?;
        
        Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "result": {
                "content": [{
                    "type": "text",
                    "text": format!("Curve '{}' CV {} position: ({:.4}, {:.4}, {:.4})", name, index, pos[0], pos[1], pos[2])
                }]
            }
        }))
    }
}