use crate::*;
use serde_json::{Value, json};

pub struct SurfaceCvDisplay;

impl SurfaceCvDisplay {
    pub fn info() -> Value {
        json!({
            "name": "surface_cv_display",
            "description": "Enable or disable CV (control vertex) display for a surface.",
            "examples": [
                {
                    "description": "Show CVs for a surface named 'MySurface'",
                    "command": "alias_lic surface_cv_display --name MySurface --enable true"
                },
                {
                    "description": "Hide CVs for a surface named 'TestSurface'",
                    "command": "alias_lic surface_cv_display --name TestSurface --enable false"
                }
            ],
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Name of the surface"
                    },
                    "enable": {
                        "type": "boolean",
                        "description": "true to show CVs, false to hide CVs"
                    }
                },
                "required": ["name", "enable"]
            }
        })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let name = args["name"].as_str().unwrap_or("");
        let enable = args["enable"].as_bool().unwrap_or(false);

        if name.is_empty() {
            return Err("name is required".to_string());
        }

        let obj = AlPickList::pick_name(name)
            .ok_or_else(|| format!("Surface '{}' not found", name))?;
        let surface_node = obj
            .as_surface_node()
            .map_err(|_| format!("Object '{}' is not a surface", name))?;
        let mut surface = surface_node.surface().ok_or("Surface is null")?;

        let was_visible = surface.is_display_mode_set(AlDisplayModeType::kDisplayGeomCVs);

        surface.set_display_mode(AlDisplayModeType::kDisplayGeomCVs, enable)?;

        surface_node.update_draw_info().ok();
        AlUniverse::redraw_screen().ok();

        Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "result": {
                "content": [{
                    "type": "text",
                    "text": format!("Surface '{}' CV display {} (was {})", name, if enable { "enabled" } else { "disabled" }, if was_visible { "visible" } else { "hidden" })
                }]
            }
        }))
    }
}