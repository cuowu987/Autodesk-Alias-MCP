use crate::*;
use serde_json::{Value, json};

pub struct StageSave;

impl StageSave {
    pub fn info() -> Value {
        json!({
            "name": "stage_save",
            "description": "Save the current Alias stage (scene) to a specified directory. Include the file name in the path.",
            "examples": [
                {
                    "description": "Save stage to 'D:/work/my_scene.wire'",
                    "command": "alias_lic stage_save --dir D:/work/my_scene.wire"
                },
                {
                    "description": "Save stage to 'C:/projects/test_scene.wire'",
                    "command": "alias_lic stage_save --dir C:/projects/test_scene.wire"
                }
            ],
            "inputSchema": {
                "type": "object",
                "properties": {
                    "dir": { "type": "string","description": "Full path to save the stage, including file name" },
                },
                "required": ["dir"]
            }
        })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let dir = args["dir"].as_str().unwrap_or("");
        AlUniverse::store(dir, None)?;
        Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "result": {
                "content": [{
                    "type": "text",
                    "text": format!("Stage saved to: {}", dir)
                }]
            }
        }))
    }
}