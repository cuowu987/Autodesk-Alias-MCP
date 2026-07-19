use crate::*;
use serde_json::{Value, json};

pub struct SelectObjects;

impl SelectObjects {
    pub fn info() -> Value {
        json!({
            "name": "select_objects",
            "description": "Get names and types information about the currently selected objects in Alias. You must first select an object in the Alias scene before calling this tool.",
            "examples": [
                {
                    "description": "Get information about currently selected objects",
                    "command": "alias_lic select_objects"
                }
            ],
            "inputSchema": {
                "type": "object",
                "properties": {},
                "required": []
            }
        })
    }

    pub fn func(_args: &Value, id_val: &Value) -> Result<Value, String> {
        let obj = AlPickList::pick_all().collect::<Vec<_>>();
        if obj.is_empty() {
            Err("No object selected".to_string())
        } else {
            let names = obj.iter().map(|o| o.name()).collect::<Vec<_>>();
            let types = obj
                .iter()
                .map(|o| format!("{:?}", o.type_()))
                .collect::<Vec<_>>();
            Ok(json!({
                "jsonrpc": "2.0",
                "id": id_val,
                "result": {
                    "content": [{
                        "type": "text",
                        "text": format!("Selected objects: names=[{}], types=[{}]", names.join(", "), types.join(", "))
                    }]
                }
            }))
        }
    }
}