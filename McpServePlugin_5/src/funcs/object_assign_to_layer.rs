use crate::*;
use serde_json::{Value, json};

pub struct ObjectAssignToLayer;

impl ObjectAssignToLayer {
    pub fn info() -> Value {
        json!({
            "name": "object_assign_to_layer",
            "description": "Assign an existing object to a layer by their names. Use this to organize objects into specific layers for better scene management.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "object_name": { "type": "string","description": "Name of the object to assign" },
                    "layer_name": { "type": "string","description": "Name of the target layer" }
                },
                "required": ["object_name", "layer_name"]
        },
        "examples": [
            {
                "description": "Assign an object to a layer",
                "command": "alias_lic object_assign_to_layer --object_name Curve1 --layer_name default"
            }
        ]
    })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let object_name = args["object_name"].as_str().unwrap_or("");
        let layer_name = args["layer_name"].as_str().unwrap_or("");
        let obj =
            AlPickList::pick_name(object_name).ok_or("Object not found".to_string())?;
        let dag = obj
            .as_dag_node()
            .map_err(|_| "Object is not a dag node".to_string())?;
        let layer = AlUniverse::layer_by_name(layer_name)?;
        dag.set_layer(&layer)?;
        Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "result": {
                "content": [{
                    "type": "text",
                    "text": format!("Object{{name:{}}} assigned to layer{{name:{}}}", object_name, layer_name)
                }]
            }
        }))
    }
}