#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiMarkerColor {
    NeonGreen,   // 1. 荧光绿
    Cyan,        // 2. 纯青
    ElectricPurp,// 3. 电光紫
    Magenta,     // 4. 荧光粉
    Yellow,      // 5. 纯黄
    VividOrange, // 6. 鲜艳橙
    PureRed,     // 7. 大红
    SkyBlue,     // 8. 天蓝
    MintGreen,   // 9. 薄荷绿
    DeepIndigo,  // 10. 深靛蓝
    DeepPink,    // 11. 深粉
    Golden,      // 12. 金黄
    Lime,        // 13. 青柠
    Teal,        // 14. 深青
    PureBlue,    // 15. 纯蓝
    Coral,       // 16. 珊瑚红
    Lavender,    // 17. 淡紫
    Amber,       // 18. 琥珀
    GrassGreen,  // 19. 草绿
    PureWhite,   // 20. 纯白
}

impl AiMarkerColor {
    /// 转换为 AlToolShader 所需的 [f32; 4] RGBA 数组
    pub fn to_rgba_f32(&self) -> [f32; 4] {
        match self {
            Self::NeonGreen    => [0.0, 1.0, 0.0, 1.0],
            Self::Cyan         => [0.0, 1.0, 1.0, 1.0],
            Self::ElectricPurp => [0.56, 0.0, 1.0, 1.0],
            Self::Magenta      => [1.0, 0.0, 1.0, 1.0],
            Self::Yellow       => [1.0, 1.0, 0.0, 1.0],
            Self::VividOrange  => [1.0, 0.5, 0.0, 1.0],
            Self::PureRed      => [1.0, 0.0, 0.0, 1.0],
            Self::SkyBlue      => [0.0, 0.5, 1.0, 1.0],
            Self::MintGreen    => [0.0, 1.0, 0.5, 1.0],
            Self::DeepIndigo   => [0.3, 0.0, 0.5, 1.0],
            Self::DeepPink     => [1.0, 0.0, 0.5, 1.0],
            Self::Golden       => [1.0, 0.84, 0.0, 1.0],
            Self::Lime         => [0.75, 1.0, 0.0, 1.0],
            Self::Teal         => [0.0, 0.5, 0.5, 1.0],
            Self::PureBlue     => [0.0, 0.0, 1.0, 1.0],
            Self::Coral        => [1.0, 0.25, 0.25, 1.0],
            Self::Lavender     => [0.78, 0.64, 0.78, 1.0],
            Self::Amber        => [1.0, 0.75, 0.0, 1.0],
            Self::GrassGreen   => [0.5, 1.0, 0.0, 1.0],
            Self::PureWhite    => [1.0, 1.0, 1.0, 1.0],
        }
    }

    /// 获取所有颜色的列表，方便循环调用
    pub fn all() ->[AiMarkerColor; 20] {
        [
            Self::NeonGreen, Self::Cyan, Self::ElectricPurp, Self::Magenta,
            Self::Yellow, Self::VividOrange, Self::PureRed, Self::SkyBlue,
            Self::MintGreen, Self::DeepIndigo, Self::DeepPink, Self::Golden,
            Self::Lime, Self::Teal, Self::PureBlue, Self::Coral,
            Self::Lavender, Self::Amber, Self::GrassGreen, Self::PureWhite,
        ]
    }
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}