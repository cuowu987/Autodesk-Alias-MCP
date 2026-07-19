use serde_json::{Value, json};

pub struct MathAdd;

impl MathAdd {
    pub fn info() -> Value {
        json!({
            "name": "math_add",
            "description": "Calculate the sum of two numeric values a and b. Use this for basic arithmetic addition operations.",
            "examples": [
                {
                    "description": "Add 2 and 3 together",
                    "command": "alias_lic math_add --a 2 --b 3"
                },
                {
                    "description": "Add 10.5 and -5.2 together",
                    "command": "alias_lic math_add --a 10.5 --b -5.2"
                }
            ],
            "inputSchema": {
                "type": "object",
                "properties": {
                    "a": {"type": "number","description": "First number to add"},
                    "b": {"type": "number","description": "Second number to add"}
                },
                "required": ["a", "b"]
            }
        })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let a = args["a"].as_f64().unwrap_or(0.0);
        let b = args["b"].as_f64().unwrap_or(0.0);
        let sum = a + b;
        Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "result": {
                "content": [{ "type": "text", "text": format!("{} + {} = {}", a, b, sum) }]
            }
        }))
    }
}