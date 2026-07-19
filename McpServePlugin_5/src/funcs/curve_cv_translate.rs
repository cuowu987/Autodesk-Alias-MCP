use crate::*;
use serde_json::{Value, json};

pub struct CurveCvTranslate;

impl CurveCvTranslate {
    pub fn info() -> Value {
        json!({
            "name": "curve_cv_translate",
            "description": "Translate (move) a specific control point (CV) on a curve. Supports two modes: relative displacement (vector) or absolute position.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Name of the curve to modify"
                    },
                    "index": {
                        "type": "integer",
                        "description": "Index of the control point to move (0-based)"
                    },
                    "mode": {
                        "type": "string",
                        "enum": ["relative", "absolute"],
                        "description": "Movement mode: 'relative' for displacement vector, 'absolute' for exact position. Default: 'relative'"
                    },
                    "vector": {
                        "type": "array",
                        "items": {
                            "type": "number"
                        },
                        "description": "Relative displacement vector [dx, dy, dz] for the control point (used in 'relative' mode)"
                    },
                    "position": {
                        "type": "array",
                        "items": {
                            "type": "number"
                        },
                        "description": "Absolute position [x, y, z] for the control point (used in 'absolute' mode)"
                    }
                },
                "required": ["name", "index"]
            },
            "examples": [
                {
                    "description": "Move a CV by a relative displacement vector",
                    "command": "alias_lic curve_cv_translate --name curve1 --index 2 --mode relative --vector [1.0, 0.5, 0.0]"
                },
                {
                    "description": "Move a CV to an absolute position",
                    "command": "alias_lic curve_cv_translate --name curve2 --index 0 --mode absolute --position [5.0, 3.0, 2.0]"
                }
            ]
        })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let name = args["name"].as_str().unwrap_or("");
        let index = args["index"].as_i64().unwrap_or(-1) as i32;
        let mode = args["mode"].as_str().unwrap_or("relative");
        
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
        
        let mut cv = curve
            .get_cv(index)
            .ok_or_else(|| format!("Failed to get CV at index {}", index))?;
        
        let old_pos = cv.world_position()?;
        let (new_x, new_y, new_z, dx, dy, dz) = if mode == "absolute" {
            if !args["position"].is_array() {
                return Err("position must be an array [x, y, z] in absolute mode".to_string());
            }
            let pos = args["position"].as_array().unwrap();
            if pos.len() != 3 {
                return Err("position must have exactly 3 elements [x, y, z]".to_string());
            }
            let nx = pos[0].as_f64().unwrap_or(0.0);
            let ny = pos[1].as_f64().unwrap_or(0.0);
            let nz = pos[2].as_f64().unwrap_or(0.0);
            (nx, ny, nz, nx - old_pos[0], ny - old_pos[1], nz - old_pos[2])
        } else {
            if !args["vector"].is_array() {
                return Err("vector must be an array [dx, dy, dz] in relative mode".to_string());
            }
            let vector = args["vector"].as_array().unwrap();
            if vector.len() != 3 {
                return Err("vector must have exactly 3 elements [dx, dy, dz]".to_string());
            }
            let dx_val = vector[0].as_f64().unwrap_or(0.0);
            let dy_val = vector[1].as_f64().unwrap_or(0.0);
            let dz_val = vector[2].as_f64().unwrap_or(0.0);
            (old_pos[0] + dx_val, old_pos[1] + dy_val, old_pos[2] + dz_val, dx_val, dy_val, dz_val)
        };
        
        cv.set_world_position_3d(new_x, new_y, new_z, true)?;
        curve_node.update_draw_info().ok();
        AlUniverse::redraw_screen().ok();
        
        Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "result": {
                "content": [{
                    "type": "text",
                    "text": format!("Curve '{}' CV {} moved {} by ({:.4}, {:.4}, {:.4}): from ({:.4}, {:.4}, {:.4}) to ({:.4}, {:.4}, {:.4})", name, index, if mode == "absolute" { "abs" } else { "rel" }, dx, dy, dz, old_pos[0], old_pos[1], old_pos[2], new_x, new_y, new_z)
                }]
            }
        }))
    }
}