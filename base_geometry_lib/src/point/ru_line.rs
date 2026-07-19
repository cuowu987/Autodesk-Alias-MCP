//全局忽视命名警告
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


use crate::*;


//按照C语言内存规则
#[repr(C)]
#[derive(Default, Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct RU_Line {
    pub from: RU_3dPoint,
    pub to: RU_3dPoint,
}


impl RU_Line {
    pub fn new(from: RU_3dPoint, to: RU_3dPoint) -> Self {
        if (from - to).LengthSquared() < 1e-12 {
            panic!("from and to are the same point");
        }
        Self { from, to }
    }


    /// # 获取线段长度
    pub fn Length(&self) -> f64 {
        (self.to - self.from).Length()
    }
    /// # 获取线段长度的平方
    pub fn LengthSquared(&self) -> f64 {
        (self.to - self.from).LengthSquared()
    }
    /// # 直线的方向
    pub fn Direction(&self) -> RU_3dVector {
        (self.to - self.from).UnitVector()
    }
    /// # 两直线相交
    /// 即使不相交也会返回一个最近点
    ///
    /// p1: 第一条直线的起点
    ///
    /// v1: 第一条直线的方向
    ///
    /// p2: 第二条直线的起点
    ///
    /// v2: 第二条直线的方向
    ///
    /// return : (相交点, 是否是精确点)
    pub fn Intersect_p(
        p1: RU_3dPoint,
        v1: RU_3dVector,
        p2: RU_3dPoint,
        v2: RU_3dVector,
    ) -> (RU_3dPoint, (f64, f64), bool) {
        let w0 = p1 - p2;
        let a = v1 * v1;
        let b = v1 * v2;
        let c = v2 * v2;
        let d = v1 * w0;
        let e = v2 * w0;


        let denom = a * c - b * b;
        let (sc, tc) = if denom.abs() < 1e-10 {
            // 平行
            let sc = 0.0;
            let tc = if b.abs() > c.abs() { d / b } else { e / c };
            (sc, tc)
        } else {
            let sc = (b * e - c * d) / denom;
            let tc = (a * e - b * d) / denom;
            (sc, tc)
        };


        //
        let point_on_l1 = p1 + v1 * sc;
        let point_on_l2 = p2 + v2 * tc;


        // 判断两点是否重合
        let distance = (point_on_l1 - point_on_l2).LengthSquared();
        let s = distance < 1e-12;
        // 返回两条直线最近点的中点,如果距离小于1e-12则认为是精确点
        ((point_on_l1 + point_on_l2) * 0.5, (sc, tc), s)
    }
    /// # 两直线相交
    /// 即使不相交也会返回一个最近点
    ///
    /// line1: 第一条直线
    ///
    /// line2: 第一条直线
    ///
    /// isExact: 是否在曲线上,是否是精确值
    pub fn Intersect(&self, line2: RU_Line) -> (RU_3dPoint, (f64, f64), bool) {
        return Self::Intersect_p(self.from, self.Direction(), line2.from, line2.Direction());
    }


    /// # 获得最近点
    /// point: 输入点
    ///
    ///
    /// return: 最近点,最近点在直线上对应的参数
    pub fn ClosestPointTo(&self, point: RU_3dPoint) -> (RU_3dPoint, f64) {
        let vector = (self.to - self.from).UnitVector();
        let t = (point - self.from) * vector;
        (self.PointAt(t), t)
    }


    pub fn Domain(&self) -> RU_Interval {
        RU_Interval::createByStartAndEnd(0.0, self.Length())
    }
    /// # boundingbox
    pub fn BoundingBox(&self) -> RU_BoundingBox {
        let min_p = RU_3dPoint::new(
            self.from.x.min(self.to.x),
            self.from.y.min(self.to.y),
            self.from.z.min(self.to.z),
        );
        let max_p = RU_3dPoint::new(
            self.from.x.max(self.to.x),
            self.from.y.max(self.to.y),
            self.from.z.max(self.to.z),
        );
        RU_BoundingBox::createByStartAndEnd(min_p, max_p)
    }


    /// # PointAt
    pub fn PointAt(&self, t: f64) -> RU_3dPoint {
        self.Direction() * t + self.from
    }
    pub fn PointAtByNormalized(&self, Normalized_t: f64) -> RU_3dPoint {
        self.from + (self.to - self.from) * Normalized_t
    }


}





