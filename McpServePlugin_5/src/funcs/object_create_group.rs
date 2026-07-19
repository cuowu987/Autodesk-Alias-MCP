use crate::*;
use serde_json::{Value, json};

pub struct ObjectCreateGroup;

impl ObjectCreateGroup {
    pub fn info() -> Value {
        json!({
            "name": "object_create_group",
            "description": "Create a new group and add multiple objects to it by their names. Useful for organizing related objects together.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "object_names" : { "type": "array","description": "Array of object names to include in the group" }
                },
                "required": ["object_names"]
        },
        "examples": [
            {
                "description": "Create a group with multiple objects",
                "command": "alias_lic object_create_group --object_names Curve1 Curve2 Curve3"
            }
        ]
    })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let object_names = args["object_names"]
            .as_array()
            .ok_or("object_names is not an array".to_string())?
            .iter()
            .map(|x| x.as_str().unwrap_or("").to_string())
            .collect::<Vec<String>>();
        let mut group = AlGroupNode::new_create()?;
        group.set_name("ai_group").ok();
        let objs = object_names
            .into_iter()
            .map(|n| {
                AlPickList::pick_name(&n)
                    .ok_or_else(|| format!("Object '{}' not found", n))?
                    .as_dag_node()
            })
            .collect::<Result<Vec<_>, String>>()?;
        for obj in objs.iter() {
            group.add_child_node(obj)?;
        }
        Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "result": {
                "content": [{
                    "type": "text",
                    "text": format!("Group{{name:{}}} created", group.name())
                }]
            }
        }))
    }
}