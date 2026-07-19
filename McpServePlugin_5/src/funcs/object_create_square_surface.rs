use crate::*;
use serde_json::{Value, json};

pub struct ObjectCreateSquareSurface;

impl ObjectCreateSquareSurface {
    pub fn info() -> Value {
        json!({
            "name": "object_create_square_surface",
            "description": "Create a square surface from four existing points by their names. The four points must already exist and form a quadrilateral shape.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "p1" : { "type": "string","description": "Name of the first corner point" },
                    "p2": { "type": "string","description": "Name of the second corner point" },
                    "p3": { "type": "string","description": "Name of the third corner point" },
                    "p4": { "type": "string","description": "Name of the fourth corner point" }
                },
                "required": ["p1","p2","p3","p4"]
        },
        "examples": [
            {
                "description": "Create a square surface from four points",
                "command": "alias_lic object_create_square_surface --p1 Point1 --p2 Point2 --p3 Point3 --p4 Point4"
            }
        ]
    })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let p1_name = args["p1"].as_str().unwrap_or("");
        let p2_name = args["p2"].as_str().unwrap_or("");
        let p3_name = args["p3"].as_str().unwrap_or("");
        let p4_name = args["p4"].as_str().unwrap_or("");
        let p1 =
            AlPickList::pick_name(p1_name).ok_or("Point p1 not found".to_string())?;
        let p2 =
            AlPickList::pick_name(p2_name).ok_or("Point p2 not found".to_string())?;
        let p3 =
            AlPickList::pick_name(p3_name).ok_or("Point p3 not found".to_string())?;
        let p4 =
            AlPickList::pick_name(p4_name).ok_or("Point p4 not found".to_string())?;
        let p1_point = p1
            .as_space_point()
            .map_err(|_| "p1 is not a space point".to_string())?;
        let p2_point = p2
            .as_space_point()
            .map_err(|_| "p2 is not a space point".to_string())?;
        let p3_point = p3
            .as_space_point()
            .map_err(|_| "p3 is not a space point".to_string())?;
        let p4_point = p4
            .as_space_point()
            .map_err(|_| "p4 is not a space point".to_string())?;
        let points = vec![
            p1_point.world_position()?.ToRU_4dPoint(),
            p2_point.world_position()?.ToRU_4dPoint(),
            p4_point.world_position()?.ToRU_4dPoint(),
            p3_point.world_position()?.ToRU_4dPoint(),
        ];
        let mut alsurface = AlSurface::new();
        alsurface.create_1(
            1,
            1,
            curveFormType::kOpen,
            curveFormType::kOpen,
            2,
            2,
            &vec![0.0, 1.0],
            &vec![0.0, 1.0],
            2,
            2,
            &points,
        )?;
        let node = AlSurfaceNode::new_create(&alsurface)?;
        node.set_name("ai_surface").ok();
        AlUniverse::redraw_screen().ok();
        Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "result": {
                "content": [{
                    "type": "text",
                    "text": format!("Surface{{name:{}}} created", node.name())
                }]
            }
        }))
    }
}