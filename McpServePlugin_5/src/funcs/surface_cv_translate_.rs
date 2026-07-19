use crate::*;
use serde_json::{Value, json};

pub struct SurfaceCvTranslate;

impl SurfaceCvTranslate {
    pub fn info() -> Value {
        json!({
            "name": "surface_cv_translate",
            "description": "Translate (move) a specific control point (CV) on a surface by relative displacement. Use this to modify surface shape by moving individual control points.",
            "examples": [
                {
                    "description": "Move CV at (0, 0) by +1.0 units in X direction",
                    "command": "alias_lic surface_cv_translate --name MySurface --u_index 0 --v_index 0 --vector [1.0, 0, 0]"
                },
                {
                    "description": "Move CV at (1, 1) by -0.5 units in Y direction",
                    "command": "alias_lic surface_cv_translate --name MySurface --u_index 1 --v_index 1 --vector [0, -0.5, 0]"
                }
            ],
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Name of the surface to modify"
                    },
                    "u_index": {
                        "type": "integer",
                        "description": "U index of the control point to move (0-based)"
                    },
                    "v_index": {
                        "type": "integer",
                        "description": "V index of the control point to move (0-based)"
                    },
                    "vector": {
                        "type": "array",
                        "items": {
                            "type": "number"
                        },
                        "description": "Relative displacement vector [dx, dy, dz] for the control point"
                    }
                },
                "required": ["name", "u_index", "v_index", "vector"]
            }
        })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let name = args["name"].as_str().unwrap_or("");
        let u_index = args["u_index"].as_i64().unwrap_or(-1) as i32;
        let v_index = args["v_index"].as_i64().unwrap_or(-1) as i32;
        
        if name.is_empty() {
            return Err("name is required".to_string());
        }
        if u_index < 0 {
            return Err("u_index must be >= 0".to_string());
        }
        if v_index < 0 {
            return Err("v_index must be >= 0".to_string());
        }
        if !args["vector"].is_array() {
            return Err("vector must be an array [dx, dy, dz]".to_string());
        }
        
        let vector = args["vector"].as_array().unwrap();
        if vector.len() != 3 {
            return Err("vector must have exactly 3 elements [dx, dy, dz]".to_string());
        }
        
        let dx = vector[0].as_f64().unwrap_or(0.0);
        let dy = vector[1].as_f64().unwrap_or(0.0);
        let dz = vector[2].as_f64().unwrap_or(0.0);
        
        let obj = AlPickList::pick_name(name)
            .ok_or_else(|| format!("Surface '{}' not found", name))?;
        let surface_node = obj
            .as_surface_node()
            .map_err(|_| format!("Object '{}' is not a surface", name))?;
        let surface = surface_node.surface().ok_or("Surface is null")?;
        
        let u_cv_count = surface.u_num_cvs();
        let v_cv_count = surface.v_num_cvs();
        if u_index >= u_cv_count {
            return Err(format!("u_index {} exceeds surface U CV count {}", u_index, u_cv_count));
        }
        if v_index >= v_cv_count {
            return Err(format!("v_index {} exceeds surface V CV count {}", v_index, v_cv_count));
        }
        
        let mut cv = surface
            .get_cv(u_index, v_index)
            .ok_or_else(|| format!("Failed to get CV at ({}, {})", u_index, v_index))?;
        
        let old_pos = cv.world_position()?;
        let new_x = old_pos[0] + dx;
        let new_y = old_pos[1] + dy;
        let new_z = old_pos[2] + dz;
        
        cv.set_world_position_3d(new_x, new_y, new_z, true)?;
        surface_node.update_draw_info().ok();
        AlUniverse::redraw_screen().ok();
        
        Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "result": {
                "content": [{
                    "type": "text",
                    "text": format!("Surface '{}' CV ({},{}) moved by ({:.4}, {:.4}, {:.4}): from ({:.4}, {:.4}, {:.4}) to ({:.4}, {:.4}, {:.4})", name, u_index, v_index, dx, dy, dz, old_pos[0], old_pos[1], old_pos[2], new_x, new_y, new_z)
                }]
            }
        }))
    }
}