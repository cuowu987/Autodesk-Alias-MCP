use crate::*;
use serde_json::{Value, json};
use super::super::layered_canvas;

pub struct ScreenAssistRectangle;

impl ScreenAssistRectangle {
    pub fn info() -> Value {
        json!({
            "name": "screen_assist_rectangle",
            "description": "Display an assist rectangle on the screen using two corner points.",
            "examples": [
                {
                    "description": "Add a white rectangle from (50, 50) to (150, 150)",
                    "command": "alias_lic screen_assist_rectangle --point1 [50,50] --point2 [150,150]"
                },
                {
                    "description": "Add a blue thicker rectangle from (0, 0) to (200, 100)",
                    "command": "alias_lic screen_assist_rectangle --point1 [0,0] --point2 [200,100] --width 4 --color [0,0,1,1]"
                },
                {
                    "description": "Clear all assist rectangles",
                    "command": "alias_lic screen_assist_rectangle --clear true"
                }
            ],
            "inputSchema": {
                "type": "object",
                "properties": {
                    "point1": {
                        "type": "array",
                        "items": {
                            "type": "number"
                        },
                        "description": "First corner point of the rectangle [x, y] in screen coordinates"
                    },
                    "point2": {
                        "type": "array",
                        "items": {
                            "type": "number"
                        },
                        "description": "Second corner point of the rectangle [x, y] in screen coordinates"
                    },
                    "width": {
                        "type": "number",
                        "description": "Line width of the rectangle in pixels. Default: 2.0"
                    },
                    "color": {
                        "type": "array",
                        "items": {
                            "type": "number"
                        },
                        "minItems": 3,
                        "maxItems": 4,
                        "description": "RGB/RGBA color values (0-1). Default: [1, 1, 1, 1]"
                    },
                    "clear": {
                        "type": "boolean",
                        "description": "If true, clear all assist rectangles instead of adding new. Default: false"
                    }
                },
                "required": []
            }
        })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let clear = args["clear"].as_bool().unwrap_or(false);
        
        if clear {
            {
                let mut guard = layered_canvas::ASSIST_RECTANGLES.lock().unwrap();
                guard.clear();
            }

            let window = AlUniverse::current_window().ok_or("no alwindow")?;
            let alcamera = window.camera().ok_or("no camera")?;
            let (camera, camera_info) = layered_canvas::build_camera(alcamera)?;
            layered_canvas::redraw_canvas(camera, camera_info)?;

            return Ok(json!({
                "jsonrpc": "2.0",
                "id": id_val,
                "result": {
                    "content": [{
                        "type": "text",
                        "text": "All assist rectangles cleared"
                    }]
                }
            }));
        }

        let point1 = if let Some(arr) = args["point1"].as_array() {
            if arr.len() >= 2 {
                let x = arr[0].as_f64().unwrap_or(0.0) as i32;
                let y = arr[1].as_f64().unwrap_or(0.0) as i32;
                RU_2iPoint::new(x, y)
            } else {
                RU_2iPoint::new(50, 50)
            }
        } else {
            RU_2iPoint::new(50, 50)
        };

        let point2 = if let Some(arr) = args["point2"].as_array() {
            if arr.len() >= 2 {
                let x = arr[0].as_f64().unwrap_or(0.0) as i32;
                let y = arr[1].as_f64().unwrap_or(0.0) as i32;
                RU_2iPoint::new(x, y)
            } else {
                RU_2iPoint::new(150, 150)
            }
        } else {
            RU_2iPoint::new(150, 150)
        };

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
            let mut guard = layered_canvas::ASSIST_RECTANGLES.lock().unwrap();
            guard.push((point1, point2, width, color));
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
                    "text": format!("Assist rectangle added at [({},{}) ({},{})] with width {}px", point1.x, point1.y, point2.x, point2.y, width)
                }]
            }
        }))
    }
}
