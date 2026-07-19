use crate::imageruler:: add_ruler_to_image_with_params;
use serde_json::{Value, json};

pub struct ImageAddRuler;

impl ImageAddRuler {
    pub fn info() -> Value {
        json!({
            "name": "image_add_ruler",
            "description": "Add green pixel coordinate rulers directly on image (without resizing). Rulers are on image edges with major ticks at every 100 pixels.",
            "examples": [
                {
                    "description": "Add rulers to 'input.jpg' with default settings",
                    "command": "alias_lic image_add_ruler --input_path input.jpg"
                },
                {
                    "description": "Add rulers to 'screenshot.png' and save as 'output.png'",
                    "command": "alias_lic image_add_ruler --input_path screenshot.png --output_path output.png"
                },
                {
                    "description": "Add rulers with custom tick settings",
                    "command": "alias_lic image_add_ruler --input_path image.jpg --major_tick 100 --minor_tick 20"
                },
                {
                    "description": "Add rulers with larger font size (scale 1.5)",
                    "command": "alias_lic image_add_ruler --input_path image.jpg --font_scale 1.5"
                },
                {
                    "description": "Add rulers with smaller font size (scale 0.75)",
                    "command": "alias_lic image_add_ruler --input_path image.jpg --font_scale 0.75"
                }
            ],
            "inputSchema": {
                "type": "object",
                "properties": {
                    "input_path": {
                        "type": "string",
                        "description": "Full path to the input image file"
                    },
                    "output_path": {
                        "type": "string",
                        "description": "Full path to save the output image with rulers (default: input_with_ruler.png)"
                    },
                    "major_tick": {
                        "type": "integer",
                        "description": "Major tick interval in pixels (default: 100)"
                    },
                    "minor_tick": {
                        "type": "integer",
                        "description": "Minor tick interval in pixels (default: 20)"
                    },
                    "font_scale": {
                        "type": "number",
                        "description": "Font scale factor (default: 1.0, larger value = bigger font)"
                    }
                },
                "required": ["input_path"]
            }
        })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let input_path = args["input_path"].as_str().unwrap_or("");
        if input_path.is_empty() {
            return Err("input_path is required".to_string());
        }
        let output_path = args["output_path"].as_str().unwrap_or("");
        let major_tick = args["major_tick"].as_u64().unwrap_or(100) as u32;
        let minor_tick = args["minor_tick"].as_u64().unwrap_or(20) as u32;
        let font_scale = args["font_scale"].as_f64().unwrap_or(1.0);
        let out_path = if output_path.is_empty() {
            let mut path = input_path.to_string();
            if let Some(dot_pos) = path.rfind('.') {
                path.insert_str(dot_pos, "_with_ruler");
            } else {
                path.push_str("_with_ruler");
            }
            path
        } else {
            output_path.to_string()
        };
        add_ruler_to_image_with_params(
            input_path, &out_path, major_tick, minor_tick, font_scale,
        )?;
        Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "result": {
                "content": [{
                    "type": "text",
                    "text": format!("Image with rulers saved to: {}", out_path)
                }]
            }
        }))
    }
}
