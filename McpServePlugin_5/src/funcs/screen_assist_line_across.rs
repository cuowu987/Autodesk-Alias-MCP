use crate::*;
use serde_json::{Value, json};
use super::super::layered_canvas;

pub struct ScreenAssistLineAcross;

impl ScreenAssistLineAcross {
    pub fn info() -> Value {
        json!({
            "name": "screen_assist_line_across",
            "description": "Display horizontal or vertical assist lines spanning across the screen.",
            "examples": [
                {
                    "description": "Add a horizontal assist line at Y=300",
                    "command": "alias_lic screen_assist_line_across --type horizontal --pos 300"
                },
                {
                    "description": "Add a vertical red assist line at X=200",
                    "command": "alias_lic screen_assist_line_across --type vertical --pos 200 --color [1,0,0,1] --width 3"
                },
                {
                    "description": "Clear all assist lines",
                    "command": "alias_lic screen_assist_line_across --clear true"
                }
            ],
            "inputSchema": {
                "type": "object",
                "properties": {
                    "type": {
                        "type": "string",
                        "enum": ["horizontal", "vertical"],
                        "description": "Type of line: horizontal or vertical"
                    },
                    "pos": {
                        "type": "number",
                        "description": "Position of line in screen coordinates"
                    },
                    "width": {
                        "type": "number",
                        "description": "Line width in pixels. Default: 2.0"
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
                        "description": "If true, clear all assist lines instead of adding new. Default: false"
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
                let mut guard = layered_canvas::ASSIST_LINES.lock().unwrap();
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
                        "text": "All assist lines cleared"
                    }]
                }
            }));
        }

        let line_type = args["type"].as_str().unwrap_or("horizontal").to_string();
        let pos = args["pos"].as_f64().unwrap_or(100.0) as i32;
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
            let mut guard = layered_canvas::ASSIST_LINES.lock().unwrap();
            guard.push((line_type.clone(), pos, width, color));
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
                    "text": format!("{} assist line added across the screen at position {} with width {}px", line_type, pos, width)
                }]
            }
        }))
    }
}
