#![allow(non_camel_case_types)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlObjectType {
    kAmbientLightType,
    kAreaLightType,
    kBoxLightType,
    kCameraEyeType,
    kCameraType,
    kCameraUpType,
    kCameraViewType,
    kClusterNodeType,
    kClusterType,
    kClusterMemberType,
    kConeLightType,
    kCurveNodeType,
    kCurveOnSurfaceType,
    kCurveType,
    kCurveCVType,
    kCylinderLightType,
    kDagNodeType,
    kDirectionLightType,
    kFaceNodeType,
    kFaceType,
    kGroupNodeType,
    kUnusedType,
    kLightLookAtNodeType,
    kLightNodeType,
    kLightType,
    kLightUpNodeType,
    kLinearLightType,
    kNonAmbientLightType,
    kOrthographicCameraType,
    kPerspectiveCameraType,
    kPointLightType,
    kSetType,
    kSetMemberType,
    kSphereLightType,
    kSpotLightType,
    kSurfaceNodeType,
    kSurfaceType,
    kSurfaceCurveType,
    kSurfaceCVType,
    kTorusLightType,
    kVolumeLightType,
    kWindowType,

    kChannelType,
    kActionType,
    kParamActionType,
    kMotionActionType,
    kKeyframeType,
    kStreamType,

    kEnvironmentType,
    kShaderType,
    kTextureType,

    kPolysetNodeType,
    kPolysetType,
    kPolygonType,
    kPolysetVertexType,

    kAttributeType,
    kArcAttributeType,
    kLineAttributeType,
    kCurveAttributeType,
    kPlaneAttributeType,
    kConicAttributeType,
    kRevSurfAttributeType,

    kJointType,
    kConstraintType,
    kPointConstraintType,
    kOrientationConstraintType,
    kAimConstraintType,

    kTextureNodeType,

    kShellNodeType,
    kShellType,

    kTrimRegionType,
    kTrimBoundaryType,
    kTrimCurveType,

    kCommandType,
    kCommandRefType,
    kContactType,

    kCharacterSpaceType,
    kCharacterType,
    kCharSnippetType,
    kCharTransitionType,

    kIKHandleType,
    kIKHandleNodeType,

    kLocatorType,
    kAnnotationLocatorType,
    kDistanceLocatorType,
    kAngleLocatorType,
    kRadialLocatorType,
    kDeviationLocatorType,
    kMinmaxLocatorType,

    kConstructionEntityType,
    kConstructionVectorType,
    kConstructionPlaneType,
    kConstructionFutureType1,
    kConstructionFutureType2,
    kPointType,
    kSpacePointType,
    kCurvePointType,
    kSurfacePointType,
    kCurveOnSurfacePointType,
    kFuturePointType1,
    kFuturePointType2,
    kFuturePointType3,
    kFuturePointType4,
    kFuturePointType5,

    kLayerType,

    kCloudType,

    kBlendCurveType,
    kBlendPointType,

    kCategoryType,

    kMeshType,
    kMeshNodeType,

    kEvaluateType,

    kReferenceFileType,
    kReferenceFileSetType,

    kSwitchShaderType,

    kReferenceObjectType,
    kReferenceLayerType,

    kLayeredShaderType,

    kCanvasType,
    kMetaDataType,
}

impl AlObjectType {
    pub fn from_i32(v: i32) -> Result<Self,String> {
        // 检查范围以确保安全
        if v >= 0 && v <= Self::kCanvasType as i32 {
            Ok(unsafe { std::mem::transmute(v) })
        } else {
            Err("Invalid AlObjectType".to_string())
        }
    }
}
impl Default for AlObjectType {
    fn default() -> Self {
        Self::kUnusedType
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlSurfaceCurveType {
    kSurfaceCurveU = 0,
    kSurfaceCurveV = 1,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlLightGlowType {
    kLightGlowOff = 0,
    kLightGlowLinear = 1,
    kLightGlowExponential = 2,
    kLightGlowBall = 3,
    kLightGlowSpectral = 4,
    kLightGlowRainbow = 5,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlLightHaloType {
    kLightHaloOff = 0,
    kLightHaloLinear = 1,
    kLightHaloExponential = 2,
    kLightHaloBall = 3,
    kLightHaloLensFlare = 4,
    kLightHaloRimHalo = 5,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlLightFogType {
    kLightFogOff = 0,
    kLightFogLinear = 1,
    kLightFogExponential = 2,
    kLightFogBall = 3,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlDisplayModeType {
    kDisplayModeBoundingBox,
    kDisplayModeInvisible,
    kDisplayModeTemplate,
    kDisplayModeDashed,

    kDisplayGeomHull,
    kDisplayGeomEditPoints,
    kDisplayGeomKeyPoints,
    kDisplayGeomCVs,

    kDisplayModeConstructionPlane,
    kDisplayModeCompressedSbd,
}
