use crate::*;
use serde_json::{Value, json};

pub struct CurveCvDisplay;

impl CurveCvDisplay {
    pub fn info() -> Value {
        json!({
            "name": "curve_cv_display",
            "description": "Enable or disable CV (control vertex) display for a curve.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Name of the curve"
                    },
                    "enable": {
                        "type": "boolean",
                        "description": "true to show CVs, false to hide CVs"
                    }
                },
                "required": ["name", "enable"]
            },
            "examples": [
                {
                    "description": "Show the control vertices of a curve",
                    "command": "alias_lic curve_cv_display --name curve1 --enable true"
                },
                {
                    "description": "Hide the control vertices of a curve",
                    "command": "alias_lic curve_cv_display --name curve1 --enable false"
                }
            ]
        })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let name = args["name"].as_str().unwrap_or("");
        let enable = args["enable"].as_bool().unwrap_or(false);

        if name.is_empty() {
            return Err("name is required".to_string());
        }

        let obj = AlPickList::pick_name(name)
            .ok_or_else(|| format!("Curve '{}' not found", name))?;
        let curve_node = obj
            .as_curve_node()
            .map_err(|_| format!("Object '{}' is not a curve", name))?;
        let curve = curve_node.curve().ok_or("Curve is null")?;

        let was_visible = curve.is_display_mode_set(AlDisplayModeType::kDisplayGeomCVs);

        curve.set_display_mode(AlDisplayModeType::kDisplayGeomCVs, enable)?;

        curve_node.update_draw_info().ok();
        AlUniverse::redraw_screen().ok();

        Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "result": {
                "content": [{
                    "type": "text",
                    "text": format!("Curve '{}' CV display {} (was {})", name, if enable { "enabled" } else { "disabled" }, if was_visible { "visible" } else { "hidden" })
                }]
            }
        }))
    }
}