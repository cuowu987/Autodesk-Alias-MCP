use crate::functionsinfo::mcp_tool_list;
use crate::*;

pub fn handle_mcp_request(req: &Value) -> Result<Value, String> {
    let method = req["method"].as_str().unwrap_or("");
    let id_val = req["id"].clone();

    match method {
        "initialize" => Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "result": {
                "protocolVersion": "2025-11-25",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "McpServePlugin_1 MCP Server",
                    "version": "0.1.0"
                }
            }
        })),
        "tools/list" => Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "result": mcp_tool_list()
        })),
        "tools/call" => {
            let tool_name = req["params"]["name"].as_str().unwrap_or("");
            let args = &req["params"]["arguments"];
            match tool_name {
                "time_get_current" => funcs::TimeGetCurrent::func(args, &id_val),
                "math_add" => funcs::MathAdd::func(args, &id_val),
                "select_objects" => funcs::SelectObjects::func(args, &id_val),
                "curve_assist_display" => funcs::CurveAssistDisplay::func(args, &id_val),
                "screen_assist_display_clear" => funcs::ScreenAssistDisplayClear::func(args, &id_val),
                "curve_infos_by_name" => funcs::CurveInfosByName::func(args, &id_val),
                "curve_cv_translate" => funcs::CurveCvTranslate::func(args, &id_val),
                "curve_cv_get" => funcs::CurveCvGet::func(args, &id_val),
                "curve_increment_degree" => funcs::CurveIncrementDegree::func(args, &id_val),
                "curve_cv_display" => funcs::CurveCvDisplay::func(args, &id_val),
                "surface_cv_display" => funcs::SurfaceCvDisplay::func(args, &id_val),
                "surface_cv_get" => funcs::SurfaceCvGet::func(args, &id_val),
                "surface_cv_translate" => funcs::SurfaceCvTranslate::func(args, &id_val),
                "surface_infos_by_name" => funcs::SurfaceInfosByName::func(args, &id_val),
                "object_translate_by_name" => funcs::ObjectTranslateByName::func(args, &id_val),
                "object_create_point" => funcs::ObjectCreatePoint::func(args, &id_val),
                "object_delete_by_name" => funcs::ObjectDeleteByName::func(args, &id_val),
                "object_create_line" => funcs::ObjectCreateLine::func(args, &id_val),
                "object_create_square_surface" => funcs::ObjectCreateSquareSurface::func(args, &id_val),
                "object_create_group" => funcs::ObjectCreateGroup::func(args, &id_val),
                "layer_create" => funcs::LayerCreate::func(args, &id_val),
                "object_assign_to_layer" => funcs::ObjectAssignToLayer::func(args, &id_val),
                "stage_save" => funcs::StageSave::func(args, &id_val),
                "stage_screenshot" => funcs::StageScreenshot::func(args, &id_val),
                "image_add_ruler" => funcs::ImageAddRuler::func(args, &id_val),
                "screen_assist_point" => funcs::ScreenAssistPoint::func(args, &id_val),
                "screen_arrow_assist" => funcs::ScreenArrowAssist::func(args, &id_val),
                "screen_info" => funcs::ScreenInfo::func(args, &id_val),
                "plane_create_construction" => funcs::PlaneCreateConstruction::func(args, &id_val),
                "screen_to_world_position" => funcs::ScreenToWorldPosition::func(args, &id_val),
                "camera_get_info" => funcs::CameraGetInfo::func(args, &id_val),
                "view_plane_construction" => funcs::ViewPlaneConstruction::func(args, &id_val),
                "screen_assist_line_across" => funcs::ScreenAssistLineAcross::func(args, &id_val),
                "screen_assist_rectangle" => funcs::ScreenAssistRectangle::func(args, &id_val),
                "curve_transform" => funcs::CurveTransform::func(args, &id_val),
                "object_set_display_mode" => funcs::ObjectSetDisplayMode::func(args, &id_val),
                "object_set_all_invisible" => funcs::ObjectSetAllInvisible::func(args, &id_val),
                "window_display_toggle" => funcs::WindowDisplayToggle::func(args, &id_val),
                _ => Ok(json!({
                    "jsonrpc": "2.0",
                    "id": id_val,
                    "error": {
                        "code": -32601,
                        "message": format!("Tool not found: {}", tool_name)
                    }
                })),
            }
        }
        "notifications/initialized" => Ok(json!({
            "jsonrpc": "2.0",
            "id": Value::Null,
            "result": {}
        })),
        _ => Ok(json!({
            "jsonrpc": "2.0",
            "id": id_val,
            "error": {
                "code": -32601,
                "message": format!("Method not found: {}", method)
            }
        })),
    }
}
