use crate::*;
use serde_json::{Value, json};

pub struct CurveIncrementDegree;

impl CurveIncrementDegree {
    pub fn info() -> Value {
        json!({
            "name": "curve_increment_degree",
            "description": "Increment the degree of a curve by 1. This increases the curve's smoothness and adds more control points.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Name of the curve to increment degree"
                    }
                },
                "required": ["name"]
            },
            "examples": [
                {
                    "description": "Increase the degree of a curve by 1 to make it smoother",
                    "command": "alias_lic curve_increment_degree --name curve1"
                }
            ]
        })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let name = args["name"].as_str().unwrap_or("");

        if name.is_empty() {
            return Err("name is required".to_string());
        }

        let obj = AlPickList::pick_name(name)
            .ok_or_else(|| format!("Curve '{}' not found", name))?;
        let curve_node = obj
            .as_curve_node()
            .map_err(|_| format!("Object '{}' is not a curve", name))?;
        let curve = curve_node.curve().ok_or("Curve is null")?;

        let old_degree = curve.degree();

        curve.increment_degree()?;

        let new_degree = curve.degree();

        curve_node.update_draw_info().ok();
        AlUniverse::redraw_screen().ok();

        Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "result": {
                "content": [{
                    "type": "text",
                    "text": format!("Curve '{}' degree incremented from {} to {}", name, old_degree, new_degree)
                }]
            }
        }))
    }
}
