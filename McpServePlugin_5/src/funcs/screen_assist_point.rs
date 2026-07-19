use crate::*;
use serde_json::{Value, json};
use super::super::layered_canvas;

pub struct ScreenAssistPoint;

impl ScreenAssistPoint {
    pub fn info() -> Value {
        json!({
            "name": "screen_assist_point",
            "description": "Display assist points on screen using pixel coordinates.",
            "examples": [
                {
                    "description": "Add a white assist point at (100, 150)",
                    "command": "alias_lic screen_assist_point --x 100 --y 150"
                },
                {
                    "description": "Add a red larger assist point at (200, 300)",
                    "command": "alias_lic screen_assist_point --x 200 --y 300 --width 5 --color [1,0,0,1]"
                }
            ],
            "inputSchema": {
                "type": "object",
                "properties": {
                    "x": {
                        "type": "number",
                        "description": "X pixel coordinate on screen"
                    },
                    "y": {
                        "type": "number",
                        "description": "Y pixel coordinate on screen"
                    },
                    "width": {
                        "type": "number",
                        "description": "Point width in pixels. Default: 2.0"
                    },
                    "color": {
                        "type": "array",
                        "items": {
                            "type": "number"
                        },
                        "minItems": 3,
                        "maxItems": 4,
                        "description": "RGB/RGBA color values (0-1). Default: [1, 1, 1, 1]"
                    }
                },
                "required": ["x", "y"]
            }
        })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let x = args["x"].as_f64().ok_or("x is required")? as i32;
        let y = args["y"].as_f64().ok_or("y is required")? as i32;
        let width = args["width"].as_f64().unwrap_or(2.0) as f32;

        let color = if let Some(color_array) = args["color"].as_array() {
            if color_array.len() >= 3 {
                let r = color_array[0].as_f64().unwrap_or(1.0) as f32;
                let g = color_array[1].as_f64().unwrap_or(1.0) as f32;
                let b = color_array[2].as_f64().unwrap_or(1.0) as f32;
                let a = if color_array.len() >= 4 {
                    color_array[3].as_f64().unwrap_or(1.0) as f32
                } else {
                    1.0
                };
                RU_Color::new(r, g, b, a)
            } else {
                RU_Color::WHITE
            }
        } else {
            RU_Color::WHITE
        };

        {
            let mut guard = layered_canvas::ASSIST_POINTS.lock().unwrap();
            guard.push((RU_2iPoint::new(x, y), width, color));
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
                    "text": format!("Assist point added at ({}, {}) with width {}px and color {:?}", x, y, width, color)
                }]
            }
        }))
    }
}