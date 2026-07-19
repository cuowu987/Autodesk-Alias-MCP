use crate::*;
use serde_json::{Value, json};

pub struct CurveTransform;

impl CurveTransform {
    pub fn info() -> Value {
        json!({
            "name": "curve_transform",
            "description": "Transform (scale and rotate) all CV points of a curve to fit between new start and end coordinates. The curve is translated, scaled, and rotated so that its first CV moves to the new start position and its last CV moves to the new end position.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Name of the curve to transform"
                    },
                    "start_pos": {
                        "type": "array",
                        "items": {
                            "type": "number"
                        },
                        "description": "New start position [x, y, z] for the first CV point"
                    },
                    "end_pos": {
                        "type": "array",
                        "items": {
                            "type": "number"
                        },
                        "description": "New end position [x, y, z] for the last CV point"
                    }
                },
                "required": ["name", "start_pos", "end_pos"]
            },
            "examples": [
                {
                    "description": "Transform a curve to fit between new start and end positions",
                    "command": "alias_lic curve_transform --name curve1 --start_pos [0.0, 0.0, 0.0] --end_pos [10.0, 5.0, 2.0]"
                }
            ]
        })
    }

    fn normalize(v: [f64; 3]) -> [f64; 3] {
        let len = (v[0].powi(2) + v[1].powi(2) + v[2].powi(2)).sqrt();
        if len < 1e-10 {
            return [1.0, 0.0, 0.0];
        }
        [v[0] / len, v[1] / len, v[2] / len]
    }

    fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    }

    fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }

    fn rotate_point(point: [f64; 3], axis: [f64; 3], angle: f64) -> [f64; 3] {
        let cos_theta = angle.cos();
        let sin_theta = angle.sin();
        let one_minus_cos = 1.0 - cos_theta;

        let x = point[0];
        let y = point[1];
        let z = point[2];
        let ax = axis[0];
        let ay = axis[1];
        let az = axis[2];

        let rx = (cos_theta + ax * ax * one_minus_cos) * x
            + (ax * ay * one_minus_cos - az * sin_theta) * y
            + (ax * az * one_minus_cos + ay * sin_theta) * z;

        let ry = (ay * ax * one_minus_cos + az * sin_theta) * x
            + (cos_theta + ay * ay * one_minus_cos) * y
            + (ay * az * one_minus_cos - ax * sin_theta) * z;

        let rz = (az * ax * one_minus_cos - ay * sin_theta) * x
            + (az * ay * one_minus_cos + ax * sin_theta) * y
            + (cos_theta + az * az * one_minus_cos) * z;

        [rx, ry, rz]
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let name = args["name"].as_str().unwrap_or("");
        
        if name.is_empty() {
            return Err("name is required".to_string());
        }

        if !args["start_pos"].is_array() {
            return Err("start_pos must be an array [x, y, z]".to_string());
        }
        if !args["end_pos"].is_array() {
            return Err("end_pos must be an array [x, y, z]".to_string());
        }

        let start_arr = args["start_pos"].as_array().unwrap();
        let end_arr = args["end_pos"].as_array().unwrap();

        if start_arr.len() != 3 {
            return Err("start_pos must have exactly 3 elements [x, y, z]".to_string());
        }
        if end_arr.len() != 3 {
            return Err("end_pos must have exactly 3 elements [x, y, z]".to_string());
        }

        let start_pos = [
            start_arr[0].as_f64().unwrap_or(0.0),
            start_arr[1].as_f64().unwrap_or(0.0),
            start_arr[2].as_f64().unwrap_or(0.0),
        ];

        let end_pos = [
            end_arr[0].as_f64().unwrap_or(0.0),
            end_arr[1].as_f64().unwrap_or(0.0),
            end_arr[2].as_f64().unwrap_or(0.0),
        ];

        let obj = AlPickList::pick_name(name)
            .ok_or_else(|| format!("Curve '{}' not found", name))?;
        let curve_node = obj
            .as_curve_node()
            .map_err(|_| format!("Object '{}' is not a curve", name))?;
        let curve = curve_node.curve().ok_or("Curve is null")?;

        let cv_num = curve.number_of_cvs();
        if cv_num < 2 {
            return Err("Curve must have at least 2 CV points".to_string());
        }

        let (cvs, _) = curve.cvs_world_position()?;

        let p0 = [cvs[0][0], cvs[0][1], cvs[0][2]];
        let pn = [cvs[cv_num - 1][0], cvs[cv_num - 1][1], cvs[cv_num - 1][2]];

        let v1 = [pn[0] - p0[0], pn[1] - p0[1], pn[2] - p0[2]];
        let v2 = [end_pos[0] - start_pos[0], end_pos[1] - start_pos[1], end_pos[2] - start_pos[2]];

        let len1 = (v1[0].powi(2) + v1[1].powi(2) + v1[2].powi(2)).sqrt();
        let len2 = (v2[0].powi(2) + v2[1].powi(2) + v2[2].powi(2)).sqrt();

        if len1 < 1e-10 {
            return Err("Curve start and end points are identical".to_string());
        }

        let scale = len2 / len1;

        let v1_norm = Self::normalize(v1);
        let v2_norm = Self::normalize(v2);

        let dot_product = Self::dot(v1_norm, v2_norm);
        let dot_clamped = dot_product.max(-1.0).min(1.0);
        let angle = dot_clamped.acos();

        let axis = if angle < 1e-10 {
            [0.0, 0.0, 1.0]
        } else {
            Self::normalize(Self::cross(v1_norm, v2_norm))
        };

        let mut transformed_cvs = Vec::new();

        for cv in &cvs {
            let point = [cv[0], cv[1], cv[2]];
            
            let translated = [
                point[0] - p0[0],
                point[1] - p0[1],
                point[2] - p0[2],
            ];

            let scaled = [
                translated[0] * scale,
                translated[1] * scale,
                translated[2] * scale,
            ];

            let rotated = Self::rotate_point(scaled, axis, angle);

            let final_point = [
                rotated[0] + start_pos[0],
                rotated[1] + start_pos[1],
                rotated[2] + start_pos[2],
            ];

            transformed_cvs.push([final_point[0], final_point[1], final_point[2], cv[3]]);
        }

        for i in 0..cv_num {
            let mut cv = curve
                .get_cv(i as i32)
                .ok_or_else(|| format!("Failed to get CV at index {}", i))?;
            
            let new_pos = &transformed_cvs[i];
            cv.set_world_position_4d(new_pos[0], new_pos[1], new_pos[2], new_pos[3], true)?;
        }

        curve_node.update_draw_info().ok();
        AlUniverse::redraw_screen().ok();

        let start_str = format!("({:.4}, {:.4}, {:.4})", p0[0], p0[1], p0[2]);
        let end_str = format!("({:.4}, {:.4}, {:.4})", pn[0], pn[1], pn[2]);
        let new_start_str = format!("({:.4}, {:.4}, {:.4})", start_pos[0], start_pos[1], start_pos[2]);
        let new_end_str = format!("({:.4}, {:.4}, {:.4})", end_pos[0], end_pos[1], end_pos[2]);

        Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "result": {
                "content": [{
                    "type": "text",
                    "text": format!("Curve '{}' transformed: {} CVs scaled by {:.4}, rotated by {:.4} rad. From start={} end={} to start={} end={}", name, cv_num, scale, angle, start_str, end_str, new_start_str, new_end_str)
                }]
            }
        }))
    }
}