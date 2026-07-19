use crate::*;
use serde_json::{Value, json};

pub struct ObjectSetAllInvisible;

impl ObjectSetAllInvisible {
    pub fn info() -> Value {
        json!({
            "name": "object_set_all_invisible",
            "description": "Set the invisible display mode for ALL objects in the scene at once. Iterates through all dag nodes and applies the invisible display mode. Use enable=true to hide all objects, enable=false to show all objects.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "enable": {
                        "type": "boolean",
                        "description": "true to hide all objects (enable invisible mode), false to show all objects (disable invisible mode)"
                    }
                },
                "required": ["enable"]
            },
            "examples": [
                {
                    "description": "Hide all objects",
                    "command": "alias_lic object_set_all_invisible --enable true"
                },
                {
                    "description": "Show all objects",
                    "command": "alias_lic object_set_all_invisible --enable false"
                }
            ]
        })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let enable = args["enable"].as_bool().unwrap_or(true);

        let mut count: usize = 0;
        let mut failed: usize = 0;

        for dag in AlUniverse::dag_nodes() {
            match dag.set_display_mode(AlDisplayModeType::kDisplayModeInvisible, enable) {
                Ok(()) => count += 1,
                Err(_) => failed += 1,
            }
        }

        AlUniverse::redraw_screen().ok();

        Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "result": {
                "content": [{
                    "type": "text",
                    "text": format!("Applied invisible mode {} to {} objects (failed: {})", if enable { "enabled" } else { "disabled" }, count, failed)
                }]
            }
        }))
    }
}
