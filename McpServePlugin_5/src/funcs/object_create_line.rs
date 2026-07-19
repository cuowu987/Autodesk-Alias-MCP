use crate::*;
use serde_json::{Value, json};

pub struct ObjectCreateLine;

impl ObjectCreateLine {
    pub fn info() -> Value {
        json!({
            "name": "object_create_line",
            "description": "Create a new line between two existing points by their names. The start and end points must already exist in the scene.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "start": { "type": "string","description": "Name of the start point" },
                    "end": { "type": "string","description": "Name of the end point" }
                },
                "required": ["start","end"]
        },
        "examples": [
            {
                "description": "Create a line between two points",
                "command": "alias_lic object_create_line --start Point1 --end Point2"
            }
        ]
    })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let start_name = args["start"].as_str().unwrap_or("");
        let end_name = args["end"].as_str().unwrap_or("");
        let start = AlPickList::pick_name(start_name)
            .ok_or("Start point not found".to_string())?;
        let end =
            AlPickList::pick_name(end_name).ok_or("End point not found".to_string())?;
        let start_point = start
            .as_space_point()
            .map_err(|_| "Start is not a space point".to_string())?;
        let end_point = end
            .as_space_point()
            .map_err(|_| "End is not a space point".to_string())?;
        let start_pos = start_point.world_position()?;
        let end_pos = end_point.world_position()?;
        let alcurve = AlCurve::create_line_deg1(start_pos, end_pos)?;
        let node = AlCurveNode::new_create(&alcurve)?;
        node.set_name("ai_line").ok();
        AlUniverse::redraw_screen().ok();
        Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "result": {
                "content": [{
                    "type": "text",
                    "text": format!("Line{{name:{}}} created", node.name())
                }]
            }
        }))
    }
}