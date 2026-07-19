use crate::*;

pub fn mcp_tool_list() -> Value {
    json!({
        "tools": [
            funcs::TimeGetCurrent::info(),
            funcs::MathAdd::info(),
            funcs::SelectObjects::info(),
            funcs::CurveAssistDisplay::info(),
            funcs::ScreenAssistDisplayClear::info(),
            funcs::CurveInfosByName::info(),
            funcs::CurveCvTranslate::info(),
            funcs::CurveCvGet::info(),
            funcs::CurveIncrementDegree::info(),
            funcs::CurveCvDisplay::info(),
            funcs::SurfaceCvDisplay::info(),
            funcs::SurfaceCvGet::info(),
            funcs::SurfaceCvTranslate::info(),
            funcs::SurfaceInfosByName::info(),
            funcs::ObjectTranslateByName::info(),
            funcs::ObjectCreatePoint::info(),
            funcs::ObjectDeleteByName::info(),
            funcs::ObjectCreateLine::info(),
            funcs::ObjectCreateSquareSurface::info(),
            funcs::ObjectCreateGroup::info(),
            funcs::LayerCreate::info(),
            funcs::ObjectAssignToLayer::info(),
            funcs::StageSave::info(),
            funcs::StageScreenshot::info(),
            funcs::ImageAddRuler::info(),
            funcs::ScreenAssistPoint::info(),
            funcs::ScreenArrowAssist::info(),
            funcs::ScreenInfo::info(),
            funcs::PlaneCreateConstruction::info(),
            funcs::ScreenToWorldPosition::info(),
            funcs::CameraGetInfo::info(),
            funcs::ViewPlaneConstruction::info(),
            funcs::ScreenAssistLineAcross::info(),
            funcs::ScreenAssistRectangle::info(),
            funcs::CurveTransform::info(),
            funcs::ObjectSetDisplayMode::info(),
            funcs::ObjectSetAllInvisible::info(),
            funcs::WindowDisplayToggle::info()
        ]
    })
}
