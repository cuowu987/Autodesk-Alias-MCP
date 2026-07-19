use crate::*;
use serde_json::{Value, json};

pub struct ScreenAssistDisplayClear;

impl ScreenAssistDisplayClear {
    pub fn info() -> Value {
        json!({
            "name": "screen_assist_display_clear",
            "description": "Clear all assist display overlay including curves, points, lines, and rectangles.",
            "examples": [
                {
                    "description": "Clear all assist display overlays",
                    "command": "alias_lic screen_assist_display_clear"
                }
            ],
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    pub fn func(_args: &Value, id_val: &Value) -> Result<Value, String> {
        let (had_curve, had_points, had_lines, had_rectangles) = {
            let mut curve_guard = layered_canvas::CURVE_POINTS.lock().unwrap();
            let had_curve = curve_guard.is_some();
            *curve_guard = None;

            let mut name_guard = layered_canvas::CURVE_NAME.lock().unwrap();
            *name_guard = String::new();

            let mut assist_points_guard = layered_canvas::ASSIST_POINTS.lock().unwrap();
            let had_points = !assist_points_guard.is_empty();
            assist_points_guard.clear();

            let mut assist_arrows_guard = layered_canvas::ASSIST_ARROWS.lock().unwrap();
            let had_arrows = !assist_arrows_guard.is_empty();
            assist_arrows_guard.clear();

            let mut assist_lines_guard = layered_canvas::ASSIST_LINES.lock().unwrap();
            let had_lines = !assist_lines_guard.is_empty();
            assist_lines_guard.clear();

            let mut assist_rectangles_guard = layered_canvas::ASSIST_RECTANGLES.lock().unwrap();
            let had_rectangles = !assist_rectangles_guard.is_empty();
            assist_rectangles_guard.clear();

            (had_curve, had_points || had_arrows, had_lines, had_rectangles)
        };

        let window = AlUniverse::current_window().ok_or("no alwindow")?;
        let alcamera = window.camera().ok_or("no camera")?;
        let (camera, camera_info) = layered_canvas::build_camera(alcamera)?;
        layered_canvas::redraw_canvas(camera, camera_info)?;

        if had_curve || had_points || had_lines || had_rectangles {
            Ok(json!({
                "jsonrpc": "2.0",
                "id": id_val,
                "result": {
                    "content": [{
                        "type": "text",
                        "text": "Screen assist display overlay cleared"
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
                        "text": "No screen assist display overlay to clear"
                    }]
                }
            }))
        }
    }
}
