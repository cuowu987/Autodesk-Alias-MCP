//全局忽视命名警告
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


use core::f64;


use crate::*;


//按照C语言内存规则
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
//使用默认参数
pub struct RU_BoundingBox {
    pub m_min: RU_3dPoint,
    pub m_max: RU_3dPoint,
}


impl RU_BoundingBox {
    //构造函数
    pub fn createByStartAndEnd(min: RU_3dPoint, max: RU_3dPoint) -> Self {
        Self {
            m_min: min,
            m_max: max,
        }
    }
    pub fn DiagonalLength(&self) -> f64 {
        let diagonal = self.m_max - self.m_min;
        return diagonal.Length();
    }
    pub fn Center(&self) -> RU_3dPoint {
        let center = (self.m_max + self.m_min) / 2.0;
        return center;
    }
    pub fn Corners(&self) -> [RU_3dPoint; 8] {
        let corners = [
            RU_3dPoint::new(self.m_min.x, self.m_min.y, self.m_min.z),
            RU_3dPoint::new(self.m_min.x, self.m_min.y, self.m_max.z),
            RU_3dPoint::new(self.m_min.x, self.m_max.y, self.m_min.z),
            RU_3dPoint::new(self.m_min.x, self.m_max.y, self.m_max.z),
            RU_3dPoint::new(self.m_max.x, self.m_min.y, self.m_min.z),
            RU_3dPoint::new(self.m_max.x, self.m_min.y, self.m_max.z),
            RU_3dPoint::new(self.m_max.x, self.m_max.y, self.m_min.z),
            RU_3dPoint::new(self.m_max.x, self.m_max.y, self.m_max.z),
        ];
        return corners;
    }
    /// #  判断包围盒是否与平面相交
    pub fn IntersectsPlane(&self, plane: RU_Plane) -> bool {
        // 取包围盒的 8 个角点
        let corners = self.Corners();


        let mut has_pos = false;
        let mut has_neg = false;
        const EPS: f64 = 1e-9;


        for corner in corners {
            let d = plane.zaxis * (corner - plane.origin);


            if d.abs() < EPS {
                return true; // 角点在平面上
            } else if d > 0.0 {
                has_pos = true;
            } else {
                has_neg = true;
            }


            if has_pos && has_neg {
                return true; // 包围盒跨平面
            }
        }


        false // 所有角点在平面一侧
    }


    // 判断线段是否和 AABB 相交
    pub fn Intersects_line(&self, p0: [f64; 2], p1: [f64; 2]) -> bool {
        let (x0, y0) = (p0[0], p0[1]);
        let (x1, y1) = (p1[0], p1[1]);


        // Liang-Barsky 算法
        let dx = x1 - x0;
        let dy = y1 - y0;


        let mut t0 = 0.0;
        let mut t1 = 1.0;


        let checks = [
            (-dx, x0 - self.m_min.x),
            (dx, self.m_max.x - x0),
            (-dy, y0 - self.m_min.y),
            (dy, self.m_max.y - y0),
        ];


        for (p, q) in checks.iter() {
            if p.abs() < 1e-12 {
                if *q < 0.0 {
                    return false; // 平行且在外部
                }
            } else {
                let r = q / p;
                if *p < 0.0 {
                    if r > t1 {
                        return false;
                    }
                    if r > t0 {
                        t0 = r;
                    }
                } else {
                    if r < t0 {
                        return false;
                    }
                    if r < t1 {
                        t1 = r;
                    }
                }
            }
        }


        true
    }
    /// 判断点 p0 的正 X 方向射线是否与 AABB 相交
    pub fn intersects_xray(&self, p0: [f64; 2]) -> bool {
        let x = p0[0];
        let y = p0[1];


        // 射线在 y 方向要在盒子范围内才可能相交
        if y < self.m_min.y || y > self.m_max.y {
            return false;
        }


        // 射线正 X 方向与盒子相交条件：x < 盒子最右边
        x <= self.m_max.x
    }
}
/// # 默认构造函数(m_min 无穷大, m_max 无穷小)
impl Default for RU_BoundingBox {
    fn default() -> Self {
        Self {
            m_min: RU_3dPoint::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            m_max: RU_3dPoint::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
        }
    }
}


impl std::ops::AddAssign<RU_3dPoint> for RU_BoundingBox {
    fn add_assign(&mut self, other: RU_3dPoint) {
        self.m_min.x = self.m_min.x.min(other.x);
        self.m_min.y = self.m_min.y.min(other.y);
        self.m_min.z = self.m_min.z.min(other.z);


        self.m_max.x = self.m_max.x.max(other.x);
        self.m_max.y = self.m_max.y.max(other.y);
        self.m_max.z = self.m_max.z.max(other.z);
    }
}


// 可选：返回新的合并结果
impl std::ops::Add<RU_BoundingBox> for RU_BoundingBox {
    type Output = RU_BoundingBox;
    fn add(self, other: RU_BoundingBox) -> RU_BoundingBox {
        RU_BoundingBox {
            m_min: RU_3dPoint {
                x: self.m_min.x.min(other.m_min.x),
                y: self.m_min.y.min(other.m_min.y),
                z: self.m_min.z.min(other.m_min.z),
            },
            m_max: RU_3dPoint {
                x: self.m_max.x.max(other.m_max.x),
                y: self.m_max.y.max(other.m_max.y),
                z: self.m_max.z.max(other.m_max.z),
            },
        }
    }
}





