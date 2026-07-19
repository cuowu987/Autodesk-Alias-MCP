#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::*;

pub trait Plane_Trait {
    fn origin(&self) -> RU_3dPoint;
    fn axis(&self) -> (RU_3dVector, RU_3dVector, RU_3dVector);
    fn to_RU_Plane(&self) -> Result<RU_Plane, String> {
        let (xaxis, yaxis, _zaxis) = self.axis();
        RU_Plane::new(self.origin(), xaxis, yaxis)
    }
    fn IntersectLine(&self, line: RU_Line) -> Option<(RU_3dPoint, f64)> {
        const EPS: f64 = 1e-12;

        let (_xaxis, _yaxis, normal) = self.axis();
        //let normal = self.zaxis;
        let origin = self.origin();

        let start = line.from;
        let dir = line.Direction();

        let denom = normal * dir;

        // 平行或共面
        if denom.abs() < EPS {
            return None;
        }

        let t = (normal * (origin - start)) / denom;
        let point = start + dir * t;

        Some((point, t))
    }
}
