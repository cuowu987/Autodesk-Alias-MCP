use crate::*;
use serde_json::{Value, json};

pub struct ScreenInfo;

impl ScreenInfo {
    pub fn info() -> Value {
        json!({
            "name": "screen_info",
            "description": "Get screen information including origin coordinates, width and height.",
            "examples": [
                {
                    "description": "Get current screen resolution",
                    "command": "alias_lic screen_info"
                }
            ],
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        })
    }

    pub fn func(_args: &Value, id_val: &Value) -> Result<Value, String> {
        let window = AlUniverse::current_window().ok_or("当前没有窗口".to_string())?;
        let (w, h) = window.resolution()?;

        Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "result": {
                "content": [{
                    "type": "text",
                    //以左下角为原点
                    "text": format!("Screen info:  width={}, height={} (bottom left corner as origin)", w, h)
                }]
            }
        }))
    }
}
