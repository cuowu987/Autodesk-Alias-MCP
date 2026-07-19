use crate::*;
use serde_json::{Value, json};

pub struct SurfaceCvGet;

impl SurfaceCvGet {
    pub fn info() -> Value {
        json!({
            "name": "surface_cv_get",
            "description": "Get the world position of a specific control point (CV) on a surface by name and indices. Returns the 3D coordinates of the control point.",
            "examples": [
                {
                    "description": "Get CV position at U=0, V=0 on surface 'MySurface'",
                    "command": "alias_lic surface_cv_get --name MySurface --u_index 0 --v_index 0"
                },
                {
                    "description": "Get CV position at U=2, V=1 on surface 'TestSurface'",
                    "command": "alias_lic surface_cv_get --name TestSurface --u_index 2 --v_index 1"
                }
            ],
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Name of the surface to query"
                    },
                    "u_index": {
                        "type": "integer",
                        "description": "U index of the control point (0-based)"
                    },
                    "v_index": {
                        "type": "integer",
                        "description": "V index of the control point (0-based)"
                    }
                },
                "required": ["name", "u_index", "v_index"]
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
        
        let cv = surface
            .get_cv(u_index, v_index)
            .ok_or_else(|| format!("Failed to get CV at ({}, {})", u_index, v_index))?;
        
        let pos = cv.world_position()?;
        
        Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "result": {
                "content": [{
                    "type": "text",
                    "text": format!("Surface '{}' CV ({},{}) position: ({:.4}, {:.4}, {:.4})", name, u_index, v_index, pos[0], pos[1], pos[2])
                }]
            }
        }))
    }
}