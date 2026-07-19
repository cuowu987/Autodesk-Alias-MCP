use crate::imageruler::{add_ruler_to_image};
use crate::*;
use image::{GenericImageView, Rgba, ImageBuffer};
use serde_json::{Value, json};

pub struct StageScreenshot;

impl StageScreenshot {
    pub fn info() -> Value {
        json!({
            "name": "stage_screenshot",
            "description": "Take a screenshot of the current Alias stage view, including assist display overlay. Optionally add pixel coordinate rulers around the image for AI visual recognition. Optionally crop the image to a specific coordinate range before adding rulers.",
            "examples": [
                {
                    "description": "Take a simple screenshot",
                    "command": "alias_lic stage_screenshot"
                },
                {
                    "description": "Take a screenshot with coordinate rulers",
                    "command": "alias_lic stage_screenshot --include_ruler true"
                },
                {
                    "description": "Take a cropped screenshot from (100, 100) to (300, 300) with rulers",
                    "command": "alias_lic stage_screenshot --include_ruler true --x_start 100 --x_end 300 --y_start 100 --y_end 300"
                }
            ],
            "inputSchema": {
                "type": "object",
                "properties": {
                    "include_ruler": {
                        "type": "boolean",
                        "description": "If true, add pixel coordinate rulers around the screenshot to help AI identify coordinates (default false)"
                    },
                    "x_start": {
                        "type": "integer",
                        "description": "Start x-coordinate for cropping (optional)"
                    },
                    "x_end": {
                        "type": "integer",
                        "description": "End x-coordinate for cropping (optional, must be provided with x_start)"
                    },
                    "y_start": {
                        "type": "integer",
                        "description": "Start y-coordinate for cropping (optional)"
                    },
                    "y_end": {
                        "type": "integer",
                        "description": "End y-coordinate for cropping (optional, must be provided with y_start)"
                    }
                },
                "required": []
            }
        })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let include_ruler = args["include_ruler"].as_bool().unwrap_or(false);
        
        // Get cropping coordinates
        let x_start = args["x_start"].as_i64();
        let x_end = args["x_end"].as_i64();
        let y_start = args["y_start"].as_i64();
        let y_end = args["y_end"].as_i64();
        
        let has_crop = x_start.is_some() && x_end.is_some() && y_start.is_some() && y_end.is_some();
        
        let dir = DIR.load("DIR load is error")?;
        let window =
            AlUniverse::current_window().ok_or("No window found".to_string())?;
        let (width, height) = window.resolution().unwrap();
        let capture_path = format!("{}/screenshot.jpg", dir);
        let output_path = format!("{}/screenshot.png", dir);
        openalias_rs::AlUniverse::store_current_window(&capture_path, width, height, true)?;

        let img = image::open(&capture_path).map_err(|e| format!("Open image failed: {}", e))?;
        let (img_w, img_h) = img.dimensions();
        let mut img_data = img.to_rgba8();

        let canvas_guard = layered_canvas::OVERLAY_CANVAS.lock().map_err(|e| e.to_string())?;
        if let Some(canvas) = canvas_guard.as_ref() {
            let overlay_w = canvas.width() as u32;
            let overlay_h = canvas.height() as u32;
            let overlay_rgba = canvas.read_pixels_rgba();

            let min_w = img_w.min(overlay_w);
            let min_h = img_h.min(overlay_h);

            for y in 0..min_h {
                for x in 0..min_w {
                    let overlay_idx = ((y * overlay_w + x) * 4) as usize;

                    let overlay_r = overlay_rgba[overlay_idx] as f32 / 255.0;
                    let overlay_g = overlay_rgba[overlay_idx + 1] as f32 / 255.0;
                    let overlay_b = overlay_rgba[overlay_idx + 2] as f32 / 255.0;
                    let overlay_a = overlay_rgba[overlay_idx + 3] as f32 / 255.0;

                    if overlay_a > 0.0 {
                        let pixel = img_data.get_pixel(x, y);
                        let img_r = pixel[0] as f32 / 255.0;
                        let img_g = pixel[1] as f32 / 255.0;
                        let img_b = pixel[2] as f32 / 255.0;

                        let inv_a = 1.0 - overlay_a;
                        let out_r = (overlay_r * overlay_a + img_r * inv_a) * 255.0;
                        let out_g = (overlay_g * overlay_a + img_g * inv_a) * 255.0;
                        let out_b = (overlay_b * overlay_a + img_b * inv_a) * 255.0;

                        img_data.put_pixel(x, y, Rgba([out_r as u8, out_g as u8, out_b as u8, 255]));
                    }
                }
            }
        }

        let final_img: ImageBuffer<Rgba<u8>, Vec<u8>>;
        let crop_info: String;
        
        if has_crop {
            // Apply cropping
            let x1 = x_start.unwrap().max(0) as u32;
            let x2 = x_end.unwrap().max(0) as u32;
            // 注意：用户提供的是左下角坐标，需要转换为左上角坐标
            let y_bottom1 = y_start.unwrap().max(0) as u32;
            let y_bottom2 = y_end.unwrap().max(0) as u32;
            let y_top1 = if y_bottom1 < img_h { img_h - 1 - y_bottom1 } else { 0 };
            let y_top2 = if y_bottom2 < img_h { img_h - 1 - y_bottom2 } else { 0 };
            
            // Ensure valid coordinates
            let x_min = x1.min(x2).min(img_w - 1);
            let x_max = x1.max(x2).min(img_w - 1);
            // 转换后的 y 坐标我们也要取 min 和 max，因为用户可能给反了
            let y_min = y_top1.min(y_top2).min(img_h - 1);
            let y_max = y_top1.max(y_top2).min(img_h - 1);
            
            let crop_w = x_max - x_min + 1;
            let crop_h = y_max - y_min + 1;
            
            if crop_w == 0 || crop_h == 0 {
                return Err("Invalid crop coordinates: resulting image would be empty".to_string());
            }
            
            // Create cropped image
            final_img = image::imageops::crop_imm(&img_data, x_min, y_min, crop_w, crop_h).to_image();
            crop_info = format!(" (cropped from ({}, {}) to ({}, {}))", x_min, y_min, x_max, y_max);
        } else {
            // No cropping, use original image
            final_img = img_data;
            crop_info = String::new();
        }
        
        final_img.save(&output_path).map_err(|e| format!("Save image failed: {}", e))?;

        if include_ruler {
            let ruler_path = format!("{}/screenshot_with_ruler.png", dir);
            let upscale_factor = add_ruler_to_image(&output_path, &ruler_path, img_w, img_h)?;
            let scale_info = if upscale_factor > 1.0 {
                format!(" (upscaled at {:.2}x)", upscale_factor)
            } else {
                String::new()
            };
            Ok(json!({
                "jsonrpc": "2.0",
                "id": id_val,
                "result": {
                    "content": [{
                        "type": "text",
                        "text": format!("Stage screenshot (with ruler{}{}) saved to: {}", crop_info, scale_info, ruler_path)
                    }]
                }
            }))
        } else {
            Ok(json!({
                "jsonrpc": "2.0",
                "id": id_val,
                "result": {
                    "content": [{
                        "type": "text",
                        "text": format!("Stage screenshot{} saved to: {}", crop_info, output_path)
                    }]
                }
            }))
        }
    }
}
