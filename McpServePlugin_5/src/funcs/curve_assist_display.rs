use crate::*;
use serde_json::{Value, json};

pub struct CurveAssistDisplay;

impl CurveAssistDisplay {
    pub fn info() -> Value {
        json!({
            "name": "curve_assist_display",
            "description": "Highlight a curve with specified color and line width using OpenGL overlay.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Name of the curve to highlight"
                    },
                    "color": {
                        "type": "array",
                        "items": {
                            "type": "number"
                        },
                        "minItems": 3,
                        "maxItems": 4,
                        "description": "RGB/RGBA color values (0-1). Default: [1, 0, 0, 0.8]"
                    },
                    "width": {
                        "type": "number",
                        "description": "Line width in pixels. Default: 2.0"
                    }
                },
                "required": ["name"]
            },
            "examples": [
                {
                    "description": "Highlight a curve with default red color and width",
                    "command": "alias_lic curve_assist_display --name curve1"
                },
                {
                    "description": "Highlight a curve with custom blue color and larger line width",
                    "command": "alias_lic curve_assist_display --name curve2 --color [0, 0, 1, 0.9] --width 4.0"
                }
            ]
        })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let name = args["name"].as_str().unwrap_or("");

        if name.is_empty() {
            return Err("name is required".to_string());
        }

        let color = if let Some(color_array) = args["color"].as_array() {
            if color_array.len() >= 3 {
                let r = color_array[0].as_f64().unwrap_or(1.0) as f32;
                let g = color_array[1].as_f64().unwrap_or(0.0) as f32;
                let b = color_array[2].as_f64().unwrap_or(0.0) as f32;
                let a = if color_array.len() >= 4 {
                    color_array[3].as_f64().unwrap_or(0.8) as f32
                } else {
                    0.8
                };
                RU_Color::new(r, g, b, a)
            } else {
                RU_Color::new(1.0, 0.0, 0.0, 0.8)
            }
        } else {
            RU_Color::new(1.0, 0.0, 0.0, 0.8)
        };

        let width = args["width"].as_f64().unwrap_or(2.0) as f32;

        let obj =
            AlPickList::pick_name(name).ok_or_else(|| format!("Curve '{}' not found", name))?;
        let curve_node = obj
            .as_curve_node()
            .map_err(|_| format!("Object '{}' is not a curve", name))?;
        let curve = curve_node.curve().ok_or("Curve is null")?;

        let curve_points = layered_canvas::tessellate_curve(&curve, 100)?;

        {
            let mut guard = layered_canvas::CURVE_POINTS.lock().unwrap();
            *guard = Some(curve_points);
        }

        {
            let mut guard = layered_canvas::CURVE_COLOR.lock().unwrap();
            *guard = color;
        }

        {
            let mut guard = layered_canvas::CURVE_WIDTH.lock().unwrap();
            *guard = width;
        }

        {
            let mut guard = layered_canvas::CURVE_NAME.lock().unwrap();
            *guard = curve_node.name_ex();
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
                    "text": format!("Curve '{}' highlighted with color {:?} and width {}px", name, color, width)
                }]
            }
        }))
    }
}
