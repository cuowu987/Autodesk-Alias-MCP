use crate::*;
use serde_json::{Value, json};

pub struct ViewPlaneConstruction;

impl ViewPlaneConstruction {
    pub fn info() -> Value {
        json!({
            "name": "view_plane_construction",
            "description": "Create a construction plane from the current view window's eye plane with optional axis constraints.",
            "examples": [
                {
                    "description": "Create a view plane at origin",
                    "command": "alias_lic view_plane_construction --origin [0,0,0]"
                },
                {
                    "description": "Create a view plane at (10, 5, 0) with custom name",
                    "command": "alias_lic view_plane_construction --origin [10,5,0] --name MyViewPlane"
                },
                {
                    "description": "Create a view plane with Y-axis constraint fixed to Z direction",
                    "command": "alias_lic view_plane_construction --origin [0,0,0] --constraint { \"fix_y\": [0,0,1] }"
                }
            ],
            "inputSchema": {
                "type": "object",
                "required": ["origin"],
                "properties": {
                    "origin": {
                        "type": "array",
                        "items": {
                            "type": "number"
                        },
                        "minItems": 3,
                        "maxItems": 3,
                        "description": "Required origin point coordinates [x, y, z]."
                    },
                    "name": {
                        "type": "string",
                        "description": "Optional name for the construction plane. Default: 'ViewPlane'"
                    },
                    "constraint": {
                        "type": "object",
                        "description": "Optional axis constraint to fix one axis and recalculate normal.",
                        "properties": {
                            "fix_y": {
                                "type": "array",
                                "items": {
                                    "type": "number"
                                },
                                "minItems": 3,
                                "maxItems": 3,
                                "description": "Fix Y-axis to this vector (e.g., [0,0,1]). X-axis will be kept from eye plane, Z-axis (normal) will be recalculated."
                            },
                            "fix_x": {
                                "type": "array",
                                "items": {
                                    "type": "number"
                                },
                                "minItems": 3,
                                "maxItems": 3,
                                "description": "Fix X-axis to this vector (e.g., [0,0,1]). Y-axis will be kept from eye plane, Z-axis (normal) will be recalculated."
                            }
                        }
                    }
                }
            }
        })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let window = AlUniverse::current_window()
            .ok_or("Current window is null".to_string())?;

        let mut eye_plane = window.eye_plane()?;

        let origin_arr = args["origin"].as_array()
            .ok_or("origin is required")?;
        if origin_arr.len() < 3 {
            return Err("origin must have 3 coordinates".to_string());
        }
        eye_plane.origin = RU_3dPoint::new(
            origin_arr[0].as_f64().ok_or("origin x must be a number")?,
            origin_arr[1].as_f64().ok_or("origin y must be a number")?,
            origin_arr[2].as_f64().ok_or("origin z must be a number")?,
        );

        let axis = eye_plane.axis();
        let mut x_axis = axis.0;
        let mut y_axis = axis.1;

        if let Some(constraint) = args["constraint"].as_object() {
            if let Some(arr) = constraint["fix_y"].as_array() {
                if arr.len() >= 3 {
                    y_axis = RU_3dVector::new(
                        arr[0].as_f64().unwrap_or(0.0),
                        arr[1].as_f64().unwrap_or(0.0),
                        arr[2].as_f64().unwrap_or(0.0),
                    );
                }
            } else if let Some(arr) = constraint["fix_x"].as_array() {
                if arr.len() >= 3 {
                    x_axis = RU_3dVector::new(
                        arr[0].as_f64().unwrap_or(0.0),
                        arr[1].as_f64().unwrap_or(0.0),
                        arr[2].as_f64().unwrap_or(0.0),
                    );
                }
            }
        }

        let plane = AlConstructionPlane::new_create_xy_axis(eye_plane.origin, x_axis, y_axis)?;

        let name = args["name"].as_str().unwrap_or("ViewPlane").to_string();
        plane.set_name(&name).ok();

        AlUniverse::redraw_screen().ok();

        let normal = x_axis.cross(y_axis);
        let plane_info = json!({
            "origin": [eye_plane.origin.x, eye_plane.origin.y, eye_plane.origin.z],
            "x_axis": [x_axis.x, x_axis.y, x_axis.z],
            "y_axis": [y_axis.x, y_axis.y, y_axis.z],
            "normal": [normal.x, normal.y, normal.z],
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