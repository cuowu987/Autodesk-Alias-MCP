use crate::*;
use serde_json::{Value, json};

pub struct CurveInfosByName;

impl CurveInfosByName {
    pub fn info() -> Value {
        json!({
            "name": "curve_infos_by_name",
            "description": "Get specific information about a curve by name. Specify the curve name and info type to retrieve details. Supports: name, degree, knots, control_points, length.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Name of the curve to query"
                    },
                    "info_type": {
                        "type": "string",
                        "description": "Type of information to retrieve. Options: 'name', 'degree', 'knots', 'control_points', 'length'"
                    }
                },
                "required": ["name", "info_type"]
            },
            "examples": [
                {
                    "description": "Get the degree of a curve",
                    "command": "alias_lic curve_infos_by_name --name curve1 --info_type degree"
                },
                {
                    "description": "Get all control points of a curve",
                    "command": "alias_lic curve_infos_by_name --name curve2 --info_type control_points"
                },
                {
                    "description": "Get the length of a curve",
                    "command": "alias_lic curve_infos_by_name --name curve3 --info_type length"
                }
            ]
        })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let name = args["name"].as_str().unwrap_or("");
        let info_type = args["info_type"].as_str().unwrap_or("");

        if name.is_empty() {
            return Err("name is required".to_string());
        }
        if info_type.is_empty() {
            return Err(
                "info_type is required. Options: name, degree, knots, control_points, length"
                    .to_string(),
            );
        }

        let obj =
            AlPickList::pick_name(name).ok_or_else(|| format!("Curve '{}' not found", name))?;
        let curve_node = obj
            .as_curve_node()
            .map_err(|_| format!("Object '{}' is not a curve", name))?;
        let curve = curve_node.curve().ok_or("Curve is null")?;

        match info_type {
            "name" => {
                let name = curve_node.name();
                Ok(json!({
                    "jsonrpc": "2.0",
                    "id": id_val,
                    "result": {
                        "content": [{
                            "type": "text",
                            "text": format!("Curve name: {}", name)
                        }]
                    }
                }))
            }
            "degree" => {
                let degree = curve.degree();
                Ok(json!({
                    "jsonrpc": "2.0",
                    "id": id_val,
                    "result": {
                        "content": [{
                            "type": "text",
                            "text": format!("Curve degree: {}", degree)
                        }]
                    }
                }))
            }
            "knots" => {
                let knots = curve.real_knot_vector()?;
                let knots_str = knots
                    .iter()
                    .map(|k| format!("{:.4}", k))
                    .collect::<Vec<_>>()
                    .join(", ");
                Ok(json!({
                    "jsonrpc": "2.0",
                    "id": id_val,
                    "result": {
                        "content": [{
                            "type": "text",
                            "text": format!("Curve knots ({}): [{}]", knots.len(), knots_str)
                        }]
                    }
                }))
            }
            "control_points" => {
                let (cvs, _) = curve.cvs_world_position()?;
                let control_points: Vec<[f64; 3]> =
                    cvs.iter().map(|cv| [cv[0], cv[1], cv[2]]).collect();
                let points_str = control_points
                    .iter()
                    .enumerate()
                    .map(|(i, p)| format!("CV[{}]=({:.4}, {:.4}, {:.4})", i, p[0], p[1], p[2]))
                    .collect::<Vec<_>>()
                    .join("; ");
                Ok(json!({
                    "jsonrpc": "2.0",
                    "id": id_val,
                    "result": {
                        "content": [{
                            "type": "text",
                            "text": format!("Curve control points ({}): {}", control_points.len(), points_str)
                        }]
                    }
                }))
            }
            "length" => {
                let length = curve.length(None, None)?;
                Ok(json!({
                    "jsonrpc": "2.0",
                    "id": id_val,
                    "result": {
                        "content": [{
                            "type": "text",
                            "text": format!("Curve length: {:.4} cm", length)
                        }]
                    }
                }))
            }
            _ => Err(format!(
                "Invalid info_type: {}. Options: name, degree, knots, control_points, length",
                info_type
            )),
        }
    }
}
