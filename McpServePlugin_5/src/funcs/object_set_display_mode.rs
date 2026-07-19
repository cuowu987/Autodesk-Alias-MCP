use crate::*;
use serde_json::{Value, json};

pub struct ObjectSetDisplayMode;

impl ObjectSetDisplayMode {
    pub fn info() -> Value {
        json!({
            "name": "object_set_display_mode",
            "description": "Set the display mode of an object by its name. Controls visibility and rendering style of the object. Only 4 modes are supported: 'bounding_box' (show as bounding box), 'invisible' (hide object), 'template' (display as template), 'dashed' (display as dashed lines).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Name of the object to modify"
                    },
                    "display_mode": {
                        "type": "string",
                        "enum": ["bounding_box", "invisible", "template", "dashed"],
                        "description": "Display mode to set. Options: 'bounding_box', 'invisible', 'template', 'dashed'"
                    },
                    "enable": {
                        "type": "boolean",
                        "description": "true to enable the display mode, false to disable it"
                    }
                },
                "required": ["name", "display_mode", "enable"]
            },
            "examples": [
                {
                    "description": "Set object to invisible mode",
                    "command": "alias_lic object_set_display_mode --name Curve1 --display_mode invisible --enable true"
                },
                {
                    "description": "Set object to bounding box mode",
                    "command": "alias_lic object_set_display_mode --name Curve1 --display_mode bounding_box --enable true"
                },
                {
                    "description": "Set object to dashed mode",
                    "command": "alias_lic object_set_display_mode --name Curve1 --display_mode dashed --enable true"
                }
            ]
        })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let name = args["name"].as_str().unwrap_or("");
        let display_mode_str = args["display_mode"].as_str().unwrap_or("");
        let enable = args["enable"].as_bool().unwrap_or(true);

        if name.is_empty() {
            return Err("name is required".to_string());
        }
        if display_mode_str.is_empty() {
            return Err("display_mode is required. Options: bounding_box, invisible, template, dashed".to_string());
        }

        let display_mode = match display_mode_str {
            "bounding_box" => AlDisplayModeType::kDisplayModeBoundingBox,
            "invisible" => AlDisplayModeType::kDisplayModeInvisible,
            "template" => AlDisplayModeType::kDisplayModeTemplate,
            "dashed" => AlDisplayModeType::kDisplayModeDashed,
            _ => return Err(format!(
                "Invalid display_mode: '{}'. Options: bounding_box, invisible, template, dashed",
                display_mode_str
            )),
        };

        let obj = AlPickList::pick_name(name)
            .ok_or_else(|| format!("Object '{}' not found", name))?;
        let dag = obj
            .as_dag_node()
            .map_err(|_| format!("Object '{}' is not a dag node", name))?;

        dag.set_display_mode(display_mode, enable)?;

        AlUniverse::redraw_screen().ok();

        Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "result": {
                "content": [{
                    "type": "text",
                    "text": format!("Object '{}' display mode '{}' {}", name, display_mode_str, if enable { "enabled" } else { "disabled" })
                }]
            }
        }))
    }
}
