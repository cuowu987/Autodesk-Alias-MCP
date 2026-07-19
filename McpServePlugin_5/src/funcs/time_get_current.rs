use serde_json::{Value, json};

pub struct TimeGetCurrent;

impl TimeGetCurrent {
    pub fn info() -> Value {
        json!({
            "name": "time_get_current",
            "description": "Get the current local date and time as a formatted string. Use this when you need to know the current time for timestamping or logging purposes.",
            "examples": [
                {
                    "description": "Get current local time",
                    "command": "alias_lic time_get_current"
                }
            ],
            "inputSchema": {
                "type": "object",
                "properties": {},
                "required": []
            }
        })
    }

    pub fn func(_args: &Value, id_val: &Value) -> Result<Value, String> {
        let now = chrono::Local::now();
        let time_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
        Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "result": {
                "content": [{
                    "type": "text",
                    "text": format!("Current local time: {}", time_str)
                }]
            }
        }))
    }
}