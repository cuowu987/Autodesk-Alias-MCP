use crate::*;
use serde_json::{Value, json};

pub struct LayerCreate;

impl LayerCreate {
    pub fn info() -> Value {
        json!({
            "name": "layer_create",
            "description": "Create a new layer with the specified name. Layers help organize objects in the scene by category or type.",
            "examples": [
                {
                    "description": "Create a layer named 'MyLayer'",
                    "command": "alias_lic layer_create --name MyLayer"
                },
                {
                    "description": "Create a layer named 'Construction'",
                    "command": "alias_lic layer_create --name Construction"
                }
            ],
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": { "type": "string","description": "Name of the new layer" }
                },
                "required": ["name"]
            }
        })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let name = args["name"].as_str().unwrap_or("");
        let mut layer = AlLayer::new();
        layer.create(name)?;
        layer.set_name(name).ok();
        Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "result": {
                "content": [{
                    "type": "text",
                    "text": format!("Layer{{name:{}, number:{}}} created", layer.name(), layer.number())
                }]
            }
        }))
    }
}