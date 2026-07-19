use crate::*;
use serde_json::{Value, json};

pub struct ObjectTranslateByName;

impl ObjectTranslateByName {
    pub fn info() -> Value {
        json!({
            "name": "object_translate_by_name",
            "description": "Translate (move) an object by its name in 3D space. Use this when you know the exact name of the object but haven't selected it. Supports optional duplication.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": { "type": "string","description": "Name of the object to translate" },
                    "x": { "type": "number","description": "Translation distance along X axis"},
                    "y": { "type": "number","description": "Translation distance along Y axis" },
                    "z": { "type": "number","description": "Translation distance along Z axis" },
                    "isDuplicate" : { "type": "boolean","description": "If true, duplicate the object before translating (default false)" }
                },
                "required": ["name","x", "y", "z", "isDuplicate"]
        },
        "examples": [
            {
                "description": "Translate an object without duplication",
                "command": "alias_lic object_translate_by_name --name Curve1 --x 10 --y 5 --z 0 --isDuplicate false"
            },
            {
                "description": "Translate and duplicate an object",
                "command": "alias_lic object_translate_by_name --name Curve1 --x 20 --y 10 --z 0 --isDuplicate true"
            }
        ]
    })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let name = args["name"].as_str().unwrap_or("");
        let obj =
            AlPickList::pick_name(name).ok_or("No object selected".to_string())?;
        let dag = obj
            .as_dag_node()
            .map_err(|_| "Selected object is not a dag node".to_string())?;
        let x = args["x"].as_f64().unwrap_or(0.0);
        let y = args["y"].as_f64().unwrap_or(0.0);
        let z = args["z"].as_f64().unwrap_or(0.0);
        let is_duplicate = args["isDuplicate"].as_bool().unwrap_or(false);
        let mut dag_1 = match is_duplicate {
            true => dag
                .copy_object(None)
                .ok_or("Failed to copy object".to_string())?
                .as_dag_node()?,
            false => dag,
        };
        dag_1.translate(x, y, z)?;
        let point = dag_1.translation()?;
        Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "result": {
                "content": [{
                    "type": "text",
                    "text": format!("{} at ({},{},{})", dag_1.name(), point[0], point[1], point[2])
                }]
            }
        }))
    }
}