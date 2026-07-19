use crate::*;
use serde_json::{Value, json};

pub struct PlaneCreateConstruction;

impl PlaneCreateConstruction {
    pub fn info() -> Value {
        json!({
            "name": "plane_create_construction",
            "description": "Create a construction plane with specified origin and type (xy or yz).",
            "examples": [
                {
                    "description": "Create XY plane at origin with default axes",
                    "command": "alias_lic plane_create_construction --type xy --origin [0,0,0]"
                },
                {
                    "description": "Create YZ plane at (10, 0, 0) with custom name",
                    "command": "alias_lic plane_create_construction --type yz --origin [10,0,0] --name MyYZPlane"
                },
                {
                    "description": "Create XY plane with custom axes and origin",
                    "command": "alias_lic plane_create_construction --type xy --origin [5,5,0] --x_axis [1,0,0] --y_axis [0,1,0] --name MyXYPlane"
                }
            ],
            "inputSchema": {
                "type": "object",
                "properties": {
                    "type": {
                        "type": "string",
                        "enum": ["xy", "yz"],
                        "description": "Type of plane: 'xy' for XY-plane, 'yz' for YZ-plane. Default: 'xy'"
                    },
                    "origin": {
                        "type": "array",
                        "items": {
                            "type": "number"
                        },
                        "minItems": 3,
                        "maxItems": 3,
                        "description": "Origin point coordinates [x, y, z]. Default: [0, 0, 0]"
                    },
                    "x_axis": {
                        "type": "array",
                        "items": {
                            "type": "number"
                        },
                        "minItems": 3,
                        "maxItems": 3,
                        "description": "X-axis vector [x, y, z] (only for xy type). Default: [1, 0, 0]"
                    },
                    "y_axis": {
                        "type": "array",
                        "items": {
                            "type": "number"
                        },
                        "minItems": 3,
                        "maxItems": 3,
                        "description": "Y-axis vector [x, y, z] (for xy type) or Y-axis vector (for yz type). Default: [0, 1, 0]"
                    },
                    "normal": {
                        "type": "array",
                        "items": {
                            "type": "number"
                        },
                        "minItems": 3,
                        "maxItems": 3,
                        "description": "Normal vector [x, y, z] (only for yz type). Default: [1, 0, 0]"
                    },
                    "name": {
                        "type": "string",
                        "description": "Optional name for the plane. Default: 'ConstructionPlane'"
                    }
                }
            }
        })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let plane_type = args["type"].as_str().unwrap_or("xy").to_string();

        let origin = if let Some(arr) = args["origin"].as_array() {
            if arr.len() >= 3 {
                RU_3dPoint::new(
                    arr[0].as_f64().unwrap_or(0.0),
                    arr[1].as_f64().unwrap_or(0.0),
                    arr[2].as_f64().unwrap_or(0.0),
                )
            } else {
                RU_3dPoint::new(0.0, 0.0, 0.0)
            }
        } else {
            RU_3dPoint::new(0.0, 0.0, 0.0)
        };

        let mut name = args["name"].as_str().unwrap_or("ConstructionPlane").to_string();

        let (_plane, plane_type_str, axis1, axis2) = if plane_type == "yz" {
            let y_axis = if let Some(arr) = args["y_axis"].as_array() {
                if arr.len() >= 3 {
                    RU_3dVector::new(
                        arr[0].as_f64().unwrap_or(0.0),
                        arr[1].as_f64().unwrap_or(1.0),
                        arr[2].as_f64().unwrap_or(0.0),
                    )
                } else {
                    RU_3dVector::new(0.0, 1.0, 0.0)
                }
            } else {
                RU_3dVector::new(0.0, 1.0, 0.0)
            };

            let normal = if let Some(arr) = args["normal"].as_array() {
                if arr.len() >= 3 {
                    RU_3dVector::new(
                        arr[0].as_f64().unwrap_or(1.0),
                        arr[1].as_f64().unwrap_or(0.0),
                        arr[2].as_f64().unwrap_or(0.0),
                    )
                } else {
                    RU_3dVector::new(1.0, 0.0, 0.0)
                }
            } else {
                RU_3dVector::new(1.0, 0.0, 0.0)
            };

            let plane = AlConstructionPlane::new_create_yz_axis(origin, y_axis, normal)?;
            plane.set_name(&name).ok();
            name = plane.name();
            AlUniverse::redraw_screen().ok();

            (plane, "yz".to_string(), y_axis, normal)
        } else {
            let x_axis = if let Some(arr) = args["x_axis"].as_array() {
                if arr.len() >= 3 {
                    RU_3dVector::new(
                        arr[0].as_f64().unwrap_or(1.0),
                        arr[1].as_f64().unwrap_or(0.0),
                        arr[2].as_f64().unwrap_or(0.0),
                    )
                } else {
                    RU_3dVector::new(1.0, 0.0, 0.0)
                }
            } else {
                RU_3dVector::new(1.0, 0.0, 0.0)
            };

            let y_axis = if let Some(arr) = args["y_axis"].as_array() {
                if arr.len() >= 3 {
                    RU_3dVector::new(
                        arr[0].as_f64().unwrap_or(0.0),
                        arr[1].as_f64().unwrap_or(1.0),
                        arr[2].as_f64().unwrap_or(0.0),
                    )
                } else {
                    RU_3dVector::new(0.0, 1.0, 0.0)
                }
            } else {
                RU_3dVector::new(0.0, 1.0, 0.0)
            };

            let plane = AlConstructionPlane::new_create_xy_axis(origin, x_axis, y_axis)?;
            plane.set_name(&name).ok();
            name = plane.name();
            AlUniverse::redraw_screen().ok();

            (plane, "xy".to_string(), x_axis, y_axis)
        };

        let plane_info = json!({
            "type": plane_type_str,
            "origin": [origin.x, origin.y, origin.z],
            "axis1": [axis1.x, axis1.y, axis1.z],
            "axis2": [axis2.x, axis2.y, axis2.z],
            "name": name
        });
        Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "result": {
                "content": [{
                    "type": "text",
                    "text": plane_info.to_string()
                }]
            }
        }))
    }
}
