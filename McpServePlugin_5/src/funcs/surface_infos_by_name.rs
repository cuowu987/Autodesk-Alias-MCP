use crate::*;
use serde_json::{Value, json};

pub struct SurfaceInfosByName;

impl SurfaceInfosByName {
    pub fn info() -> Value {
        json!({
            "name": "surface_infos_by_name",
            "description": "Get specific information about a surface by name. Specify the surface name and info type to retrieve details. Supports: name, u_degree, v_degree, area, control_points, u_knots, v_knots.",
            "examples": [
                {
                    "description": "Get area of surface 'MySurface'",
                    "command": "alias_lic surface_infos_by_name --name MySurface --info_type area"
                },
                {
                    "description": "Get U degree of surface 'TestSurface'",
                    "command": "alias_lic surface_infos_by_name --name TestSurface --info_type u_degree"
                },
                {
                    "description": "Get all control points of surface 'MySurface'",
                    "command": "alias_lic surface_infos_by_name --name MySurface --info_type control_points"
                }
            ],
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Name of the surface to query"
                    },
                    "info_type": {
                        "type": "string",
                        "description": "Type of information to retrieve. Options: 'name', 'u_degree', 'v_degree', 'area', 'control_points', 'u_knots', 'v_knots'"
                    }
                },
                "required": ["name", "info_type"]
            }
        })
    }

    pub fn func(args: &Value, id_val: &Value) -> Result<Value, String> {
        let name = args["name"].as_str().unwrap_or("");
        let info_type = args["info_type"].as_str().unwrap_or("");
        
        if name.is_empty() {
            return Err("name is required".to_string());
        }
        if info_type.is_empty() {
            return Err("info_type is required. Options: name, u_degree, v_degree, area, control_points, u_knots, v_knots".to_string());
        }
        
        let obj = AlPickList::pick_name(name)
            .ok_or_else(|| format!("Object '{}' not found", name))?;
        let surface_node = obj
            .as_surface_node()
            .map_err(|_| format!("Object '{}' is not a surface", name))?;
        let surface = surface_node.surface().ok_or("Surface is null")?;

        match info_type {
            "name" => {
                let name = surface_node.name();
                Ok(json!({
                    "jsonrpc": "2.0",
                    "id": id_val,
                    "result": {
                        "content": [{
                            "type": "text",
                            "text": format!("Surface name: {}", name)
                        }]
                    }
                }))
            }
            "u_degree" => {
                let u_degree = surface.u_degree();
                Ok(json!({
                    "jsonrpc": "2.0",
                    "id": id_val,
                    "result": {
                        "content": [{
                            "type": "text",
                            "text": format!("Surface u_degree: {}", u_degree)
                        }]
                    }
                }))
            }
            "v_degree" => {
                let v_degree = surface.v_degree();
                Ok(json!({
                    "jsonrpc": "2.0",
                    "id": id_val,
                    "result": {
                        "content": [{
                            "type": "text",
                            "text": format!("Surface v_degree: {}", v_degree)
                        }]
                    }
                }))
            }
            "area" => {
                let area = surface.area(None, None)?;
                Ok(json!({
                    "jsonrpc": "2.0",
                    "id": id_val,
                    "result": {
                        "content": [{
                            "type": "text",
                            "text": format!("Surface area: {:.4} cm2", area)
                        }]
                    }
                }))
            }
            "control_points" => {
                let (cvs, _, _) = surface.cvs_world_position()?;
                let control_points: Vec<Vec<f64>> =
                    cvs.iter().map(|cv| vec![cv[0], cv[1], cv[2]]).collect();
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
                            "text": format!("Surface control points ({}): {}", control_points.len(), points_str)
                        }]
                    }
                }))
            }
            "u_knots" => {
                let u_degree = surface.u_degree();
                let u_cv_count = surface.u_num_cvs();
                let mut u_knots = vec![0.0; (u_cv_count + u_degree + 1) as usize];
                surface.real_u_knot_vector(&mut u_knots)?;
                let knots_str = u_knots.iter().map(|k| format!("{:.4}", k)).collect::<Vec<_>>().join(", ");
                Ok(json!({
                    "jsonrpc": "2.0",
                    "id": id_val,
                    "result": {
                        "content": [{
                            "type": "text",
                            "text": format!("Surface u_knots ({}): [{}]", u_knots.len(), knots_str)
                        }]
                    }
                }))
            }
            "v_knots" => {
                let v_degree = surface.v_degree();
                let v_cv_count = surface.v_num_cvs();
                let mut v_knots = vec![0.0; (v_cv_count + v_degree + 1) as usize];
                surface.real_v_knot_vector(&mut v_knots)?;
                let knots_str = v_knots.iter().map(|k| format!("{:.4}", k)).collect::<Vec<_>>().join(", ");
                Ok(json!({
                    "jsonrpc": "2.0",
                    "id": id_val,
                    "result": {
                        "content": [{
                            "type": "text",
                            "text": format!("Surface v_knots ({}): [{}]", v_knots.len(), knots_str)
                        }]
                    }
                }))
            }
            _ => Err(format!(
                "Invalid info_type: {}. Options: name, u_degree, v_degree, area, control_points, u_knots, v_knots",
                info_type
            )),
        }
    }
}