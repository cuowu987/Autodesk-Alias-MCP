use crate::*;
use serde_json::{Value, json};
use super::super::layered_canvas;

pub struct ScreenArrowAssist;

impl ScreenArrowAssist {
    pub fn info() -> Value {
        json!({
            "name": "screen_arrow_assist",
            "description": "Display assist arrow on screen using pixel coordinates.",
            "examples": [
                {
                    "description": "Add a yellow arrow from (100, 100) to (200, 200)",
                    "command": "alias_lic screen_arrow_assist --x1 100 --y1 100 --x2 200 --y2 200"
                },
                {
                    "description": "Add a red arrow with custom size and thickness",
                    "command": "alias_lic screen_arrow_assist --x1 50 --y1 50 --x2 300 --y2 150 --color [1,0,0,1] --arrow_size 30 --line_width 5"
                }
            ],
            "inputSchema": {
                "type": "object",
                "properties": {
                    "x1": {
                        "type": "number",
                        "description": "X pixel coordinate of arrow start point"
                    },
                    "y1": {
                        "type": "number",
                        "description": "Y pixel coordinate of arrow start point"
                    },
                    "x2": {
                        "type": "number",
                        "description": "X pixel coordinate of arrow end point"
                    },
                    "y2": {
                        "type": "number",
                        "description": "Y pixel coordinate of arrow end point"
                    },
                    "color": {
                        "type": "array",
                        "items": {
                            "type": "number"
                        },
                        "minItems": 3,
                        "maxItems": 4,
                        "description": "RGB/RGBA color values (0-1). Default: [1, 1, 0, 1]"
                    },
                    "arrow_size": {
                        "type": "number",
                        "description": "Arrow head size in pixels. Default: 20.0"
                    },
                    "line_width": {
                        "type": "number",
                        "description": "Line width in pixels. Default: 3.0"
                    }
                },
                "required": ["x1", "y1", "x2", "y2"]
            }
        })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let x1 = args["x1"].as_f64().ok_or("x1 is required")? as i32;
        let y1 = args["y1"].as_f64().ok_or("y1 is required")? as i32;
        let x2 = args["x2"].as_f64().ok_or("x2 is required")? as i32;
        let y2 = args["y2"].as_f64().ok_or("y2 is required")? as i32;

        let color = if let Some(color_array) = args["color"].as_array() {
            if color_array.len() >= 3 {
                let r = color_array[0].as_f64().unwrap_or(1.0) as f32;
                let g = color_array[1].as_f64().unwrap_or(1.0) as f32;
                let b = color_array[2].as_f64().unwrap_or(0.0) as f32;
                let a = if color_array.len() >= 4 {
                    color_array[3].as_f64().unwrap_or(1.0) as f32
                } else {
                    1.0
                };
                RU_Color::new(r, g, b, a)
            } else {
                RU_Color::new(1.0, 1.0, 0.0, 1.0)
            }
        } else {
            RU_Color::new(1.0, 1.0, 0.0, 1.0)
        };

        let arrow_size = args["arrow_size"].as_f64().unwrap_or(20.0) as f32;
        let line_width = args["line_width"].as_f64().unwrap_or(3.0) as f32;

        {
            let mut guard = layered_canvas::ASSIST_ARROWS.lock().unwrap();
            guard.push((RU_2iPoint::new(x1, y1), RU_2iPoint::new(x2, y2), color, arrow_size, line_width));
        }

        let window = AlUniverse::current_window().ok_or("no alwindow")?;
        let alcamera = window.camera().ok_or("no camera")?;
        let (camera, camera_info) = layered_canvas::build_camera(alcamera)?;
        layered_canvas::redraw_canvas(camera, camera_info)?;

        Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "result": {
                "content": [{
                    "type": "text",
                    "text": format!("Assist arrow added from ({}, {}) to ({}, {})", x1, y1, x2, y2)
                }]
            }
        }))
    }
}