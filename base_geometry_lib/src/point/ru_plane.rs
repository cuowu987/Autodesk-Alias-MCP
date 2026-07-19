//全局忽视命名警告
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::*;

//按照C语言内存规则
#[repr(C)]
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
//使用默认参数
pub struct RU_Plane {
    pub origin: RU_3dPoint,
    pub xaxis: RU_3dVector,
    pub yaxis: RU_3dVector,
    pub zaxis: RU_3dVector,
}

impl RU_Plane {
    /// # 创建平面
    /// y轴会被重塑
    pub fn new(origin: RU_3dPoint, xaxis: RU_3dVector, yaxis: RU_3dVector) -> Result<Self, String> {
        let xaxis = xaxis.UnitVector();
        let zaxis_raw = xaxis ^ yaxis;
        let len = zaxis_raw.Length();

        if len < 1e-12 {
            return Err(format!(
                "Cannot create RU_Plane: xaxis({:?}) and yaxis({:?}) are parallel or degenerate.",
                xaxis, yaxis
            ));
        }

        let zaxis = zaxis_raw / len;
        let yaxis = (zaxis ^ xaxis).UnitVector();

        Ok(Self {
            origin,
            xaxis,
            yaxis,
            zaxis,
        })
    }

    pub const NAN: Self = Self {
        origin: RU_3dPoint::NAN,
        xaxis: RU_3dVector::NAN,
        yaxis: RU_3dVector::NAN,
        zaxis: RU_3dVector::NAN,
    };
    pub fn is_nan(&self) -> bool {
        self.origin.is_nan() && self.xaxis.is_nan() && self.yaxis.is_nan() && self.zaxis.is_nan()
    }
    /// # plane 最近点
    pub fn PointAt(&self, u: f64, v: f64) -> RU_3dPoint {
        self.origin + self.xaxis * u + self.yaxis * v
    }
    pub fn normal(&self) -> RU_3dVector {
        self.zaxis
    }
    pub const XY: Self = Self {
        origin: RU_3dPoint::ORIGIN,
        xaxis: RU_3dVector::X_AXIS,
        yaxis: RU_3dVector::Y_AXIS,
        zaxis: RU_3dVector::Z_AXIS,
    };
    pub const YZ: Self = Self {
        origin: RU_3dPoint::ORIGIN,
        xaxis: RU_3dVector::Y_AXIS,
        yaxis: RU_3dVector::Z_AXIS,
        zaxis: RU_3dVector::X_AXIS,
    };
    pub const ZX: Self = Self {
        origin: RU_3dPoint::ORIGIN,
        xaxis: RU_3dVector::Z_AXIS,
        yaxis: RU_3dVector::X_AXIS,
        zaxis: RU_3dVector::Y_AXIS,
    };
}
impl Default for RU_Plane {
    fn default() -> Self {
        Self {
            origin: RU_3dPoint::default(),
            xaxis: RU_3dVector::new(1.0, 0.0, 0.0),
            yaxis: RU_3dVector::new(0.0, 1.0, 0.0),
            zaxis: RU_3dVector::new(0.0, 0.0, 1.0),
        }
    }
}
impl Plane_Trait for RU_Plane {
    fn origin(&self) -> RU_3dPoint {
        self.origin
    }
    fn axis(&self) -> (RU_3dVector, RU_3dVector, RU_3dVector) {
        (self.xaxis, self.yaxis, self.zaxis)
    }
}
