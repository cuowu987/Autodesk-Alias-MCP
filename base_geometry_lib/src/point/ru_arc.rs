//全局忽视命名警告
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::*;
use std::f64::consts::PI;

//按照C语言内存规则
#[repr(C)]
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
//使用默认参数
pub struct RU_Arc {
    pub m_plane: RU_Plane,
    pub m_radius: f64,
    pub m_angle: RU_Interval,
}
impl RU_Arc {
    /// # 构造函数
    /// m_plane:ON_Plane
    ///
    /// m_radius: f64
    ///
    /// m_angle:RU_Interval (0-2PI)
    ///
    /// m_start:RU_3dPoint
    pub fn create(plane: RU_Plane, radius: f64, angle: RU_Interval) -> Self {
        Self {
            m_plane: plane,
            m_radius: radius,
            m_angle: angle,
        }
    }
    /// # 构造圆
    /// plane:ON_Plane
    ///
    /// radius: f64
    ///
    /// angle:RU_Interval
    pub fn createCircle(plane: RU_Plane, radius: f64) -> Self {
        Self {
            m_plane: plane,
            m_radius: radius,
            m_angle: RU_Interval::createByStartAndEnd(0.0, 2.0 * PI),
        }
    }

    pub fn Center(&self) -> RU_3dPoint {
        self.m_plane.origin
    }

    pub const NAN: Self = Self {
        m_plane: RU_Plane::NAN,
        m_radius: f64::NAN,
        m_angle: RU_Interval::NAN,
    };
    pub fn is_nan(&self) -> bool {
        self.m_plane.is_nan() || self.m_radius.is_nan() || self.m_angle.is_nan()
    }

    pub fn PointAt(&self, t: f64) -> RU_3dPoint {
        self.m_plane.origin
            + self.m_plane.xaxis * (self.m_radius * t.cos())
            + self.m_plane.yaxis * (self.m_radius * t.sin())
    }
    pub fn StartPoint(&self) -> RU_3dPoint {
        self.PointAt(self.m_angle.m_t0)
    }
    pub fn StartDir(&self) -> RU_3dVector {
        let theta = self.m_angle.m_t0;
        (-self.m_plane.xaxis * theta.sin() + self.m_plane.yaxis * theta.cos()).UnitVector()
    }

    pub fn EndPoint(&self) -> RU_3dPoint {
        self.PointAt(self.m_angle.m_t1)
    }
    pub fn EndDir(&self) -> RU_3dVector {
        let theta = self.m_angle.m_t1;
        (-self.m_plane.xaxis * theta.sin() + self.m_plane.yaxis * theta.cos()).UnitVector()
    }
}

impl Default for RU_Arc {
    fn default() -> Self {
        Self {
            m_plane: RU_Plane::default(),
            m_radius: 1.0,
            m_angle: RU_Interval::createByStartAndEnd(0.0, 2.0 * PI),
        }
    }
}
