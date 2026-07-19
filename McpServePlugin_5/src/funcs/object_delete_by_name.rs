use crate::*;
use serde_json::{Value, json};

pub struct ObjectDeleteByName;

impl ObjectDeleteByName {
    pub fn info() -> Value {
        json!({
            "name": "object_delete_by_name",
            "description": "Delete an object by its name. Use this when you know the exact name of the object you want to remove from the scene.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": { "type": "string","description": "Name of the object to delete" }
                },
                "required": ["name"]
        },
        "examples": [
            {
                "description": "Delete an object by name",
                "command": "alias_lic object_delete_by_name --name Curve1"
            }
        ]
    })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let name = args["name"].as_str().unwrap_or("");
        let obj = AlPickList::pick_name(name).ok_or("No object deleted".to_string())?;
        obj.delete_object()?;
        AlUniverse::redraw_screen().ok();
        Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "result": {
                "content": [{
                    "type": "text",
                    "text": format!("{} deleted", name)
                }]
            }
        }))
    }
}